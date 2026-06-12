use serde::Serialize;

#[derive(Debug, thiserror::Error, Serialize)]
#[serde(tag = "type", content = "message")]
pub enum CfError {
    #[error("IO error: {0}")]
    Io(String),
    #[error("Scan not found: {0}")]
    ScanNotFound(String),
    #[error("AI error: {0}")]
    Ai(String),
    #[error("Rule error: {0}")]
    Rule(String),
    #[error("{0}")]
    Other(String),
}

impl From<anyhow::Error> for CfError {
    fn from(e: anyhow::Error) -> Self {
        Self::Other(e.to_string())
    }
}

impl From<std::io::Error> for CfError {
    fn from(e: std::io::Error) -> Self {
        Self::Io(e.to_string())
    }
}

pub type Result<T> = std::result::Result<T, CfError>;
