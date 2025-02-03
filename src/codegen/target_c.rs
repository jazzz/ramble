use anyhow::Context;
use convert_case::{Case, Casing};
use handlebars::handlebars_helper;
use handlebars::{to_json, Handlebars, RenderErrorReason};
use serde_json::Map;
use serde_json::Value;
use std::path::PathBuf;

use super::generate::Lang;
use super::FileObject;
use crate::config::packet::{FieldType, RambleConfig};

handlebars_helper!(upper_camel: |x: str| x.to_case(Case::UpperCamel));
handlebars_helper!(upper: |x: str| x.to_case(Case::UpperSnake));

handlebars_helper!(map_type: |x: str| {
    let ty = FieldType::try_from(x).map_err(|e| RenderErrorReason::Other(e.to_string()) )?; // TODO: Improve error handling
    TargetC::type_map(&ty).to_string()
});

handlebars_helper!(skip_first: |x: u64| x > 0);

pub struct TargetC {}

impl Lang for TargetC {
    fn type_map(ft: &FieldType) -> &str {
        match ft {
            FieldType::Uint8T => "uint8_t",
        }
    }

    fn render_template(rfg: &RambleConfig) -> anyhow::Result<Vec<FileObject>> {
        // This makes the assumption that the hbs files are available to the binary at runtime, and is not a
        // sustainable solution. Currently this also requires that the executable is called from the project
        // root, so they relative path works.
        let path = PathBuf::from("src/codegen/templates/cpp/ramble.hpp.hbs");

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
        handlebars.register_helper("upper", Box::new(upper));
        handlebars.register_helper("map_type", Box::new(map_type));
        handlebars.register_helper("skip_first", Box::new(skip_first));

        let mut data = Map::<String, Value>::new();
        data.insert("packets".into(), to_json(&rfg.messages));

        Ok(vec![FileObject {
            filename,
            contents: handlebars.render("src", &data)?,
        }])
    }
}
