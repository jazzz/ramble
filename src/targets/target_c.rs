use convert_case::{Case, Casing};

use super::generate::Lang;
use crate::consts::C_INDENT;
use crate::packet::{Field, FieldType, Packet};
use crate::utils::indent;

pub struct TargetC {}

impl Lang for TargetC {
    fn gen_packet(pkt: &Packet) -> Vec<String> {
        let mut lines = Vec::<String>::new();
        let struct_name = pkt.name().to_case(Case::UpperCamel);

        lines.push("#pragma pack(push, 1)".into());
        lines.push(format!("typedef struct {}", &struct_name));
        lines.push("{".into());
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

    fn gen_boilerplate(packets: &[Packet]) -> Vec<String> {
        let mut lines = Vec::<String>::new();

        // Build PacketTypes
        let enum_offset = 0x50;

        lines.push("enum PacketTypes".into());
        lines.push("{".into());
        for (i, pkt) in packets.iter().enumerate() {
            lines.push(format!(
                "{:indent$}{} = {},",
                "",
                pkt.name().to_case(Case::UpperSnake),
                (enum_offset + i),
                indent = C_INDENT
            ));
        }
        lines.push("};".into());
        lines.push("".into());

        // Build WrappingStruct
        lines.push("typedef struct Packet".into());
        lines.push("{".into());
        lines.push(indent!(1, "uint8_t packet_type;"));
        lines.push(indent!(1, "union"));
        lines.push(indent!(1, "{"));
        for pkt in packets {
            let name = pkt.name();
            lines.push(indent!(
                2,
                "{}_t {};",
                name.to_case(Case::UpperCamel),
                name.to_case(Case::Snake)
            ));
        }
        lines.push(indent!(1, "};"));
        lines.push("} Packet_t;".into());
        lines.push("".into());

        // Serialize Code - Generic
        lines.push("template <typename T>".into());
        lines.push("size_t serialize(uint8_t *buf, T *pkt)".into());
        lines.push("{".into());
        lines.push(indent!(1, "memcpy(buf, pkt, sizeof(T));"));
        lines.push(indent!(1, "return sizeof(T);"));
        lines.push("}".into());
        lines.push("".into());

        // Serialize Code - Packet_t
        lines.push("template <>".into());
        lines.push("size_t serialize<Packet_t>(uint8_t *buf, Packet_t *pkt)".into());
        lines.push("{".into());
        lines.push(indent!(1, "size_t bytes_written = 0;"));
        lines.push(indent!(1, "buf[bytes_written++] = pkt->packet_type;"));
        lines.push(indent!(1, ""));
        lines.push(indent!(
            1,
            "switch (static_cast<PacketTypes>(pkt->packet_type))"
        ));
        lines.push(indent!(1, "{"));
        for pkt in packets {
            let name = pkt.name();
            lines.push(indent!(1, "case {}:", name.to_case(Case::UpperSnake)));
            lines.push(indent!(
                2,
                "bytes_written += serialize(&buf[bytes_written], &pkt->{});",
                name.to_case(Case::Snake)
            ));
            lines.push(indent!(2, "break;"));
            lines.push("".into());
        }
        lines.push(indent!(1, "}"));
        lines.push(indent!(1, " return bytes_written;"));
        lines.push("}".into());
        lines.push("".into());

        // DeSerialize Code
        lines.push("Packet_t *deserialize(uint8_t *buf)".into());
        lines.push("{".into());
        lines.push(indent!(1, "return (Packet_t *)buf;"));
        lines.push("}".into());

        lines
    }
}
