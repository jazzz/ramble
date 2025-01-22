use convert_case::{Case, Casing};

use super::generate::Lang;
use crate::consts::C_INDENT;
use crate::packet::{Field, FieldType, Packet};

pub struct TargetC {}

impl Lang for TargetC {
    fn gen_packet(pkt: &Packet) -> Vec<String> {
        let mut lines = Vec::<String>::new();
        let struct_name = pkt.name().to_case(Case::UpperCamel);

        lines.push("#pragma pack(push, 1)".into());
        lines.push(format!("typedef struct {} {{", &struct_name));
        for f in pkt.fields() {
            lines.push(Self::gen_field(f));
        }
        lines.push(format!("}} {}_t;", &struct_name));
        lines.push("#pragma pack(pop)".into());
        lines.push("".into());

        lines
    }

    fn gen_field(f: &Field) -> String {
        format!(
            "{:indent$}{} {};",
            "",
            Self::type_map(f.field_type()),
            f.name(),
            indent = C_INDENT
        )
    }

    fn type_map(ft: &FieldType) -> &str {
        match ft {
            FieldType::Uint8T => "uint8_t",
        }
    }
}
