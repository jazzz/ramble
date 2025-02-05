use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("{0}")]
    IOError(#[from] std::io::Error),
    #[error("missing required parameter: {0}")]
    MissingParameter(String),
    #[error("version:{0} is not supported")]
    VersionNotSupported(String),
    #[error("unknown fieldtype: {0}")]
    InvalidFieldType(String),
    #[error("yaml file contains invalid sequences")]
    BadYaml(#[from] yaml_rust2::ScanError),
    #[error("file contains unexpected data: {0}")]
    BadFormat(String),
    #[error("program error(this should not have been possible): {0}")]
    ProgramError(String),
    #[error("unexpected type: expected:{0} found:{1} ")]
    UnexpectedType(String, String),
}
