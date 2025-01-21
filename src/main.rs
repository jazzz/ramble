use anyhow::Result;
use clap::{Parser, Subcommand};
use paris::{info, Logger};
use std::fs::File;
use std::io::Read;
use yaml_rust2::{YamlEmitter, YamlLoader};

mod packet;
mod parse;

use parse::Scanner;
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// ensure ramble.yaml file is valid
    Verify {
        file: Option<String>,
    },
    Generate {
        lang: String,
    },
}

fn load_file(filepath: &str) -> Result<String> {
    let mut f = File::open(filepath)?;
    let mut file_data = String::new();
    f.read_to_string(&mut file_data)?;
    Ok(file_data)
}

fn main() -> Result<()> {
    info!("Starting Ramble");

    let scanner = Scanner {};
    let cfg = load_file("ramble.yaml")?;
    scanner.parse_yaml(&cfg)?;

    info!("Done");

    Ok(())
}
