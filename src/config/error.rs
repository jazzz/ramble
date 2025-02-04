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
    #[error("unexpected type for {0}. expected:{1} found:{2} ")]
    UnexpectedType(String, String, String),
}
