#[derive(Debug)]
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

#[derive(Debug)]
pub enum FieldType {
    Uint8T,
}

#[derive(Debug)]
pub struct Field {
    name: String,
    field_type: FieldType,
}

impl Field {
    pub fn new(name: String, field_type: FieldType) -> Self {
        Field { name, field_type }
    }
}
