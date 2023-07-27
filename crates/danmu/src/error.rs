use thiserror::Error;

pub type Result<T> = std::result::Result<T, SeamDanmuError>;

#[derive(Error, Debug)]
pub enum SeamDanmuError {
    #[error("IO error: {0}")]
    IO(#[from] std::io::Error),
    #[error("Path error: {0}")]
    Path(String),
    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),
    #[error("unknown data store error")]
    Unknown,
}
