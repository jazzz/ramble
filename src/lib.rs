mod codegen;
mod config;

pub use codegen::{CodeGenerator, TargetC, TargetRust};
pub use config::load_ramble_file;
