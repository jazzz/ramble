use anyhow::Result;
use std::io::Read;
use std::{fs::File, path::Path};

use crate::{Packet, Scanner};

fn read_file<P: AsRef<Path>>(filepath: P) -> Result<String> {
    let mut f = File::open(filepath)?;
    let mut file_data = String::new();
    f.read_to_string(&mut file_data)?;
    Ok(file_data)
}

pub fn load_ramble_file<P: AsRef<Path>>(filepath: P) -> Result<Vec<Packet>> {
    let scanner = Scanner {};
    let cfg = read_file(filepath)?;
    let pkts = scanner.parse_yaml(&cfg)?;

    Ok(pkts)
}
