use crate::packet::{Field, FieldType, Packet};

pub trait Lang {
    fn gen_packet(pkt: &Packet) -> Vec<String>;
    fn gen_field(f: &Field) -> String;
    fn type_map(ft: &FieldType) -> &str;
}

pub struct CodeGenerator {}

impl CodeGenerator {
    pub fn to_code<T: Lang>(&self, packets: &Vec<Packet>) -> Vec<String> {
        let mut out_lines: Vec<String> = Vec::new();
        for pkt in packets {
            let mut lines = T::gen_packet(pkt);

            out_lines.append(&mut lines);
        }

        out_lines
    }
}
