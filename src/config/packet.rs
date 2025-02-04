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

/// All currently supported types.
#[derive(Debug, Serialize, Deserialize)]
pub enum FieldType {
    U8,
    U16,
    U32,
    U64,
    I8,
    I16,
    I32,
    I64,
}

impl TryFrom<&str> for FieldType {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        match value {
            "U8" => Ok(Self::U8),
            "U16" => Ok(Self::U16),
            "U32" => Ok(Self::U32),
            "U64" => Ok(Self::U64),
            "I8" => Ok(Self::I8),
            "I16" => Ok(Self::I16),
            "I32" => Ok(Self::I32),
            "I64" => Ok(Self::I64),
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
