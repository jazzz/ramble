use anyhow::{Error, Result};
use log::debug;
use paris::{info, warn};
use yaml_rust2::Yaml::Hash;
use yaml_rust2::{Yaml, YamlLoader};

use super::packet::{Field, FieldType, Packet, RambleConfig};

const KEY_PACKETS: &str = "packets";
const KEY_NAME: &str = "name";
const KEY_PROPS: &str = "properties";
const KEY_FIELDS: &str = "fields";

pub struct Scanner {}

impl Scanner {
    pub fn parse_yaml(&self, cfg_string: &str) -> Result<RambleConfig> {
        let docs = YamlLoader::load_from_str(cfg_string)?;
        let doc = &docs[0];

        let mut ramble_config = RambleConfig::default();

        if let Some(params) = doc.as_hash() {
            for (k, v) in params.iter() {
                Self::process_root_param(&mut ramble_config, k, v)?;
            }
        }

        let version = match ramble_config.params.get("version") {
            Some(v) => v,
            None => return Err(Error::msg("Missing Parameter (Version)")),
        };

        if version == "1" {
            Self::process_yaml_v1(ramble_config, doc)
        } else {
            Err(Error::msg(format!("Version:{} is not supported", version)))
        }
    }

    fn process_yaml_v1(mut ramble_config: RambleConfig, doc: &Yaml) -> Result<RambleConfig> {
        info!("Ramble::v1 detected");

        if let Some(pkts) = &(doc[KEY_PACKETS]).as_vec() {
            for pkt in pkts.iter() {
                if let Ok(p) = Self::process_struct(pkt) {
                    debug!("Adding packet: {:?}", p);
                    ramble_config.add_msg(p);
                };
            }
        }

        Ok(ramble_config)
    }

    fn process_struct(doc: &Yaml) -> Result<Packet> {
        let name = doc[KEY_NAME]
            .as_str()
            .expect("Struct entry contains no name");

        let has_props = doc[KEY_PROPS].is_null();
        if has_props {
            warn!("Packet properties are not implemented")
        }

        let mut pkt = Packet::new(name);

        if let Some(fields) = doc[KEY_FIELDS].as_vec() {
            for field in fields.iter() {
                if let Hash(h) = field {
                    for (field_id, field_type) in h {
                        debug!(" Field_Id:{:?}  Field_Type:{:?}", field_id, field_type);

                        let s = field_id.as_str().expect("BAD field name");
                        let ft = match field_type.as_str().expect("BAD field type") {
                            "u8" => FieldType::Uint8T,
                            _ => return Err(Error::msg("Unrecognized fieldType")),
                        };

                        pkt.add_field(Field::new(s.into(), ft));
                    }
                }
            }
        }

        Ok(pkt)
    }

    fn process_root_param(
        ramble_config: &mut RambleConfig,
        key: &Yaml,
        value: &Yaml,
    ) -> Result<()> {
        let key_str = key
            .as_str()
            .ok_or(Error::msg(format!("Key must be a string: found {:?}", key)))?;

        // Ignore some values
        if key_str == "packets" {
            return Ok(());
        }

        let val_str = match value.as_str() {
            Some(s) => s,
            None => {
                warn!(
                    "Ignoring configuration key:{} - this value handler is not implemented  ",
                    key_str
                );
                return Ok(());
            }
        };

        ramble_config.add_param(key_str.to_string(), val_str.to_string());

        Ok(())
    }
}
