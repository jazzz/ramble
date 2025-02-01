use anyhow::Context;
use convert_case::{Case, Casing};
use handlebars::handlebars_helper;
use handlebars::{to_json, Handlebars, RenderErrorReason};
use serde_json::Map;
use serde_json::Value;
use std::path::PathBuf;

use super::generate::Lang;
use super::FileObject;
use crate::packet::FieldType;
use crate::RambleConfig;

handlebars_helper!(upper_camel: |x: str| x.to_case(Case::UpperCamel));
handlebars_helper!(snake: |x: str| x.to_case(Case::Snake));

handlebars_helper!(map_type: |x: str| {
    let ty = FieldType::try_from(x).map_err(|e| RenderErrorReason::Other(e.to_string()) )?; // TODO: Improve error handling
    TargetRust::type_map(&ty).to_string()
});

pub struct TargetRust {}

impl Lang for TargetRust {
    fn type_map(ft: &FieldType) -> &str {
        match ft {
            FieldType::Uint8T => "u8",
        }
    }

    fn render_template(rfg: &RambleConfig) -> anyhow::Result<Vec<FileObject>> {
        let path = PathBuf::from("src/codegen/templates/rust/ramble.rs.hbs");

        let filename = path
            .file_stem()
            .context("unable to get filename from path")?
            .to_os_string();

        let mut handlebars = Handlebars::new();
        handlebars.register_template_file(
            "src",
            path.to_str()
                .context("Program Error: Check path variable ")?,
        )?;

        handlebars.register_helper("upper_camel", Box::new(upper_camel));
        handlebars.register_helper("upper", Box::new(snake));
        handlebars.register_helper("map_type", Box::new(map_type));

        let mut data = Map::<String, Value>::new();
        data.insert("packets".into(), to_json(&rfg.messages));

        Ok(vec![FileObject {
            filename,
            contents: handlebars.render("src", &data)?,
        }])
    }
}
