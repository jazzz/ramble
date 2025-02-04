use std::io::{self, Read};
use std::{fs::File, path::Path};

use super::error::ConfigError;
use super::{RambleConfig, Scanner};

fn read_file<P: AsRef<Path>>(filepath: P) -> Result<String, io::Error> {
    let mut f = File::open(filepath)?;
    let mut file_data = String::new();
    f.read_to_string(&mut file_data)?;
    Ok(file_data)
}

pub fn load_ramble_file<P: AsRef<Path>>(filepath: P) -> Result<RambleConfig, ConfigError> {
    let scanner = Scanner {};
    let cfg = read_file(filepath)?;

    scanner.parse_yaml(&cfg)
}
