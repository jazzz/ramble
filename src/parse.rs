use crate::packet::{Field, FieldType, Packet};
use anyhow::{Error, Result};
use log::{debug, warn};
use paris::info;
use yaml_rust2::Yaml::Hash;
use yaml_rust2::{Yaml, YamlLoader};

const KEY_VERSION: &str = "version";
const KEY_PACKETS: &str = "packets";
const KEY_NAME: &str = "name";
const KEY_PROPS: &str = "properties";
const KEY_FIELDS: &str = "fields";

pub struct Scanner {}

impl Scanner {
    pub fn parse_yaml(&self, cfg_string: &str) -> Result<Vec<Packet>> {
        let docs = YamlLoader::load_from_str(cfg_string)?;
        let doc = &docs[0];

        let param_version = doc[KEY_VERSION].as_i64();
        if param_version.is_none() {
            return Err(Error::msg("Missing Parameter (Version)"));
        }

        match doc[KEY_VERSION].as_i64() {
            None => Err(Error::msg("Missing Parameter (Version)")),
            Some(1) => self.process_yaml_v1(doc),
            Some(v) => Err(Error::msg(format!("Version:{} is not supported", v))),
        }
    }

    fn process_yaml_v1(&self, doc: &Yaml) -> Result<Vec<Packet>> {
        info!("Ramble::v1 detected");

        let mut packets = vec![];

        if let Some(pkts) = &(doc[KEY_PACKETS]).as_vec() {
            for pkt in pkts.iter() {
                if let Ok(p) = self.process_struct(pkt) {
                    debug!("Adding packet: {:?}", p);
                    packets.push(p);
                };
            }
        }

        Ok(packets)
    }

    fn process_struct(&self, doc: &Yaml) -> Result<Packet> {
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
}
