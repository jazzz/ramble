use anyhow::Result;
use clap::{Parser, Subcommand};
use log::debug;
use packet::Packet;
use paris::{error, info};
use std::fs::File;
use std::io::Read;

mod packet;
mod parse;

use parse::Scanner;

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

fn load_ramble_file(filename: &str) -> Result<Vec<Packet>> {
    let scanner = Scanner {};
    let cfg = load_file(filename)?;
    let pkts = scanner.parse_yaml(&cfg)?;

    Ok(pkts)
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
                Ok(pkts) => {
                    debug!("Packets: {:?}", pkts);
                }
            }
        }
        _ => {
            unimplemented!()
        }
    };

    info!("Done");
    Ok(())
}
