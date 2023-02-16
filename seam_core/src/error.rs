use thiserror::Error;

pub type Result<T> = std::result::Result<T, SeamError>;

#[derive(Error, Debug)]
pub enum SeamError {
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Type error: {0}")]
    Type(String),
    #[error("unknown data store error")]
    Unknown,
}
