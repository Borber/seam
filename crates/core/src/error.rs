use thiserror::Error;

pub type Result<T> = std::result::Result<T, SeamError>;

#[derive(Error, Debug)]
pub enum SeamError {
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Type error: {0}")]
    Type(String),
    #[error("Serde json error: {0}")]
    Json(#[from] serde_json::Error),
    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),
    #[error("Urlencoding error: {0}")]
    Decode(#[from] std::string::FromUtf8Error),
    #[error("InvalidHeaderValue error: {0}")]
    InvalidHeaderValue(#[from] reqwest::header::InvalidHeaderValue),
    #[error("ParseInt error: {0}")]
    ParseInt(#[from] std::num::ParseIntError),
    #[error("{0}")]
    Plugin(String),
    #[error("Need fix {0}")]
    NeedFix(&'static str),
    #[error("Not on")]
    None,
    #[error("Error msg: {0}")]
    Unknown(String),
}
