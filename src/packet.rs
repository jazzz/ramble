use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

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
}

impl TryFrom<&str> for FieldType {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self> {
        match value {
            "Uint8T" => Ok(Self::Uint8T),
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
