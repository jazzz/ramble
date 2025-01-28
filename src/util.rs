use anyhow::Result;
use std::io::Read;
use std::{fs::File, path::Path};

use crate::{RambleConfig, Scanner};

fn read_file<P: AsRef<Path>>(filepath: P) -> Result<String> {
    let mut f = File::open(filepath)?;
    let mut file_data = String::new();
    f.read_to_string(&mut file_data)?;
    Ok(file_data)
}

pub fn load_ramble_file<P: AsRef<Path>>(filepath: P) -> Result<RambleConfig> {
    let scanner = Scanner {};
    let cfg = read_file(filepath)?;

    scanner.parse_yaml(&cfg)
}
