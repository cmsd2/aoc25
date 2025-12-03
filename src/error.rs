use thiserror::Error;

#[derive(Error, Debug)]
pub enum AocError {
    #[error("Parse error: {0}")]
    ParseError(String),

    #[error("Nom error: {0}")]
    NomError(String),

    #[error("IO error: {0}")]
    IoError(String),
}
