use std::fmt;

#[derive(Debug, thiserror::Error)]
pub enum ScaffoldError {
    #[error("invalid project name: {0}")]
    InvalidName(String),

    #[error("project directory already exists: {0}")]
    DirExists(String),

    #[error("file already exists: {0}")]
    FileExists(String),

    #[error("template not found: {0} ({1})")]
    TemplateNotFound(String, String),

    #[error("template render error in {0}: {1}")]
    RenderError(String, String),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("validation failed:\n{0}")]
    ValidationFailed(String),

    #[error("git error: {0}")]
    Git(String),

    #[error("config error: {0}")]
    Config(String),
}

pub type Result<T> = std::result::Result<T, ScaffoldError>;