use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct Packet {
    name: String,
    fields: Vec<Field>,
}

impl Packet {
    pub fn new(name: &str) -> Self {
        Packet {
            name: name.into(),
            fields: vec![],
        }
    }

    pub fn add_field(&mut self, f: Field) {
        self.fields.push(f);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FieldType {
    Uint8T,
    Uint16T,
    Uint32T,
    Uint64T,
}

impl TryFrom<&str> for FieldType {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        match value {
            "Uint8T" => Ok(Self::Uint8T),
            "Uint16T" => Ok(Self::Uint16T),
            "Uint32T" => Ok(Self::Uint32T),
            "Uint64T" => Ok(Self::Uint64T),
            _ => bail!("Unknown FieldType"),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct Field {
    name: String,
    field_type: FieldType,
}

impl Field {
    pub fn new(name: String, field_type: FieldType) -> Self {
        Field { name, field_type }
    }
}

#[derive(Debug, Serialize)]
pub struct RambleConfig {
    pub params: HashMap<String, String>,
    pub messages: Vec<Packet>,
}

impl RambleConfig {
    pub fn default() -> Self {
        Self {
            params: HashMap::new(),
            messages: vec![],
        }
    }

    pub fn add_msgs(&mut self, msgs: Vec<Packet>) {
        self.messages = msgs;
    }

    pub fn add_msg(&mut self, msg: Packet) {
        self.messages.push(msg);
    }

    pub fn add_param(&mut self, key: String, val: String) {
        self.params.insert(key, val);
    }
}
