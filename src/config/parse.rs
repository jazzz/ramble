use std::fmt::format;

use log::debug;
use paris::warn;
use yaml_rust2::Yaml::Hash;
use yaml_rust2::{Yaml, YamlLoader};

use super::error::ConfigError;
use super::packet::{Field, FieldType, Packet, RambleConfig};

const KEY_PACKETS: &str = "packets";
const KEY_NAME: &str = "name";
const KEY_PROPS: &str = "properties";
const KEY_FIELDS: &str = "fields";

enum SupportedVersion {
    V1,
}

impl TryFrom<&str> for SupportedVersion {
    type Error = ConfigError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "1" {
            return Ok(SupportedVersion::V1);
        }

        Err(ConfigError::VersionNotSupported(value.into()))
    }
}

pub struct Scanner {}

impl Scanner {
    pub fn parse_yaml(&self, cfg_string: &str) -> Result<RambleConfig, ConfigError> {
        let docs = YamlLoader::load_from_str(cfg_string)?;
        let doc = &docs[0];

        let mut ramble_config = RambleConfig::default();

        // Load root paramters first as they will affect parsing of the rest of the file.
        if let Some(params) = doc.as_hash() {
            for (k, v) in params.iter() {
                Self::process_root_param(&mut ramble_config, k, v)?;
            }
        }

        match Self::parse_version(&ramble_config)? {
            SupportedVersion::V1 => Self::process_yaml_v1(ramble_config, doc),
        }
    }

    fn parse_version(ramble_config: &RambleConfig) -> Result<SupportedVersion, ConfigError> {
        let version = ramble_config
            .params
            .get("version")
            .ok_or(ConfigError::MissingParameter("version".into()))?;

        SupportedVersion::try_from(version.as_str())
    }

    /// Parse the struct definitions using V1 logic.
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

    /// Each struct in the definition file, needs to have its fields and params parsed.
    fn process_struct(doc: &Yaml) -> Result<Packet, ConfigError> {
        let name = doc[KEY_NAME]
            .as_str()
            .expect("Struct entry contains no name");

        let has_props = doc[KEY_PROPS].is_null();
        if has_props {
            warn!("Packet properties are not implemented")
        }

        let mut pkt = Packet::new(name);

        // Feilds are stored as a Sequence(HashMap) to preserve order. This makes parsing more complex
        // as the extra layer needs to be handled. It does seem like the yaml_rust2 implementation does
        // preserve order in mappings, this is not defined in the spec (https://yaml.org/spec/1.2.2/#mapping)
        if let Some(fields) = doc[KEY_FIELDS].as_vec() {
            for field in fields.iter() {
                if let Hash(hash_map) = field {
                    if !hash_map.len() == 1 {
                        return Err(ConfigError::BadFormat(
                            "fields must be defined as a sequence of mapping types. check that every mapping has a preceeding '-'".into(),
                        ));
                    }

                    let (key, value) = hash_map.front().ok_or(ConfigError::ProgramError(
                        "there must be 1 and only 1 field".into(),
                    ))?;

                    let field_struct = Self::process_field(key, value)?;
                    pkt.add_field(field_struct);
                }
            }
        }

        Ok(pkt)
    }

    fn process_field(i: &Yaml, f: &Yaml) -> Result<Field, ConfigError> {
        let field_id = require_str(i)?;
        let field_type = require_str(f)?;
        Field::try_from_config(field_id, field_type)
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

fn require_str(yaml: &Yaml) -> Result<&str, ConfigError> {
    yaml.as_str().ok_or(ConfigError::UnexpectedType(
        "String".into(),
        format!("{:?}", yaml),
    ))
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

        let fails = Scanner {}.parse_yaml(&func("U8"));
        assert!(fails.is_ok(), "error thrown on valid type");
    }
}
