use std::fmt::format;

use log::{debug, info};
use paris::warn;
use yaml_rust2::Yaml::Hash;
use yaml_rust2::{Yaml, YamlLoader};

use super::error::ConfigError;
use super::packet::{Field, FieldType, Packet, RambleConfig};

const KEY_PACKETS: &str = "packets";
const KEY_NAME: &str = "name";
const KEY_PROPS: &str = "properties";
const KEY_FIELDS: &str = "fields";

pub struct Scanner {}

impl Scanner {
    pub fn parse_yaml(&self, cfg_string: &str) -> Result<RambleConfig, ConfigError> {
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
            None => return Err(ConfigError::MissingParameter("version".into())),
        };

        if version == "1" {
            Self::process_yaml_v1(ramble_config, doc)
        } else {
            Err(ConfigError::VersionNotSupported(version.clone()))
        }
    }

    fn process_yaml_v1(
        mut ramble_config: RambleConfig,
        doc: &Yaml,
    ) -> Result<RambleConfig, ConfigError> {
        if let Some(pkts) = &(doc[KEY_PACKETS]).as_vec() {
            for pkt in pkts.iter() {
                let p = Self::process_struct(pkt)?;
                debug!("Adding packet: {:?}", p);
                ramble_config.add_msg(p);
            }
        }

        Ok(ramble_config)
    }

    fn process_struct(doc: &Yaml) -> Result<Packet, ConfigError> {
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

                        let ids = field_id.as_str().ok_or(ConfigError::UnexpectedType(
                            "key".into(),
                            "String".into(),
                            format!("{:?}", field),
                        ))?;

                        let fts = field_type.as_str().ok_or(ConfigError::UnexpectedType(
                            format!("fieldType for {}", ids),
                            "String".into(),
                            format!("{:?}", field_type),
                        ))?;

                        let ft = match fts {
                            "u8" => FieldType::Uint8T,
                            _ => return Err(ConfigError::InvalidFieldType(fts.into())),
                        };

                        pkt.add_field(Field::new(ids.into(), ft));
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
    ) -> Result<(), ConfigError> {
        let key_str = key.as_str().ok_or(ConfigError::BadFormat(format!(
            "Key must be a string found {:?}",
            key
        )))?;

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

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::{CodeGenerator, TargetC};

    use super::*;

    #[test]
    fn minimal_file() {
        let filecontents = "version: '1'";

        let ramble_config = Scanner {}.parse_yaml(filecontents).expect("should error");

        println!("{:?}", ramble_config);

        let outpath = Path::new("./generated/cpp");
        let code_generator = CodeGenerator::new(outpath);

        let files_written = code_generator.to_code::<TargetC>(&ramble_config).unwrap();
        println!("{:?}", files_written);
    }

    #[test]
    fn version_mismatch() {
        let fails = Scanner {}.parse_yaml("version: '99'");
        assert!(fails.is_err(), "error not thrown on bad version");

        let succeeds = Scanner {}.parse_yaml("version: '1'");
        assert!(succeeds.is_ok(), "error not thrown on good version");
    }

    #[test]
    fn invalid_field() {
        let func = |x| -> String {
            let mut v = vec![];
            v.push("version: '1'".into());
            v.push("packets:".into());
            v.push(" - name: hello".into());
            v.push("   fields:".into());
            v.push(format!("    - field: {}", x));

            v.join("\n")
        };

        println!("{}", func("notatype"));
        let fails = Scanner {}.parse_yaml(&func("notatype"));
        assert!(fails.is_err(), "error not thrown on invalid type");

        let fails = Scanner {}.parse_yaml(&func("u8"));
        assert!(fails.is_ok(), "error thrown on valid type");
    }
}
