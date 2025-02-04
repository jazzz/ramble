use thiserror::Error;

#[derive(Error, Debug)]
pub enum CodegenError {
    #[error("program error: {0}")]
    ProgramError(String),
    #[error("io:{0}")]
    IO(#[from] std::io::Error),
    #[error("template could not be loaded: {0}")]
    TemplateError(#[from] handlebars::TemplateError),
    #[error("template could not be rendered: {0}")]
    RenderError(#[from] handlebars::RenderError),
    #[error("Unknown:{0}")]
    Unknown(String),
}
