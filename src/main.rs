use anyhow::Result;
use clap::{Parser, Subcommand};
use paris::{error, info};
use std::fs;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::{fs::File, path::Path};

mod consts;
mod packet;
mod parse;
mod targets;
mod utils;

use packet::Packet;
use parse::Scanner;
use targets::{CodeGenerator, Lang, TargetC};

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
    Verify { file: Option<String> },
    Generate {
        #[arg(short, long)]
        /// The ramblefile to be loaded. "ramble.yaml" if not specified
        file: Option<String>,
        #[arg(short, long)]
        /// Location to save the generated code
        output_dir: Option<String>,
        #[arg(long = "C")]
        /// Output a C/C++ Library
        target_c: bool,
    },
}

fn load_file(filepath: &str) -> Result<String> {
    let mut f = File::open(filepath)?;
    let mut file_data = String::new();
    f.read_to_string(&mut file_data)?;
    Ok(file_data)
}

fn load_ramble_file(filename: &str) -> Result<Vec<Packet>> {
    let scanner = Scanner {};
    let cfg = load_file(filename)?;
    let pkts = scanner.parse_yaml(&cfg)?;

    Ok(pkts)
}

fn save_file(filename: &PathBuf, contents: &[String]) -> Result<()> {
    // Ensure entire path exists
    fs::create_dir_all(filename.parent().expect("invalid parent path"))?;

    let mut file = File::create(filename)?;

    // TODO: avoid all these copies
    let content_str = contents.join("\n");
    file.write_all(content_str.as_bytes())?;

    info!("File written to {}", filename.display().to_string());
    Ok(())
}

fn add_preamble(mut content: Vec<String>) -> Vec<String> {
    let mut preamble = vec![
        "///////////////////////////////////////////////".into(),
        "// This file was generated using Ramble.".into(),
        "///////////////////////////////////////////////".into(),
        "".into(),
    ];

    preamble.append(&mut content);
    preamble
}

fn generate_target<T: Lang>(packets: &[Packet], output_file: &PathBuf) -> Result<()> {
    let contents = CodeGenerator {}.to_code::<T>(packets);
    let wrapped_contents = add_preamble(contents);
    save_file(&output_file, &wrapped_contents)?;
    Ok(())
}

fn main() -> Result<()> {
    info!("Starting Ramble");

    let args = Cli::parse();

    match args.command {
        Commands::Verify { file } => {
            // Load Ramble file
            let filename = file.as_deref().unwrap_or("ramble.yaml");
            match load_ramble_file(filename) {
                Err(e) => error!("{} is invalid - {} ", filename, e),
                Ok(_) => {
                    info!("Verify Completed")
                }
            }
        }
        Commands::Generate {
            file,
            output_dir,
            target_c,
        } => {
            let out_path = match output_dir.as_deref() {
                Some(o) => Path::new(o),
                None => Path::new("./"),
            };

            // Load Ramble file
            let filename = file.as_deref().unwrap_or("ramble.yaml");
            let packets = match load_ramble_file(filename) {
                Err(e) => panic!("{} is invalid - {} ", filename, e),
                Ok(pkts) => pkts,
            };

            if target_c {
                info!("Generating C/C++ Target");
                generate_target::<TargetC>(&packets, &out_path.join("ramble.hpp"))?;
            };
        }
    };

    info!("Done");
    Ok(())
}
