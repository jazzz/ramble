use anyhow::Result;
use clap::{Parser, Subcommand};
use paris::{error, info};
use std::io::Read;
use std::{fs::File, path::Path};

use ramble::targets::{CodeGenerator, TargetC};
use ramble::{RambleConfig, Scanner};

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

fn load_ramble_file(filename: &str) -> Result<RambleConfig> {
    let scanner = Scanner {};
    let cfg = load_file(filename)?;
    scanner.parse_yaml(&cfg)
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
            let ramble_config = match load_ramble_file(filename) {
                Err(e) => panic!("{} is invalid - {} ", filename, e),
                Ok(pkts) => pkts,
            };

            let code_generator = CodeGenerator::new(out_path);

            if target_c {
                info!("Generating C/C++ Target to {:?}", out_path);
                let files_written = code_generator.to_code::<TargetC>(&ramble_config)?;

                for file in files_written {
                    info!("    file written: {:#?}", file);
                }
            };
        }
    };

    info!("Done");
    Ok(())
}
