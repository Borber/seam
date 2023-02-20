use thiserror::Error;

pub type Result<T> = std::result::Result<T, SeamStatusError>;

#[derive(Error, Debug)]
pub enum SeamStatusError {
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Type error: {0}")]
    Type(String),
    #[error("Serde json error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("unknown data store error")]
    Unknown,
}
