use thiserror::Error;

#[derive(Debug, Error)]
pub enum LlmError {
    #[error("config: {0}")]
    Config(String),
    #[error("io: {0}")]
    Io(#[from] std::io::Error),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("download: {0}")]
    Download(String),
    #[error("verify: {0}")]
    Verify(String),
    #[error("inference busy")]
    Busy,
    #[error("engine: {0}")]
    Engine(String),
    #[error("invalid request: {0}")]
    InvalidRequest(String),
}

pub type Result<T> = std::result::Result<T, LlmError>;

impl LlmError {
    pub fn config(msg: impl Into<String>) -> Self {
        Self::Config(msg.into())
    }

    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }

    pub fn download(msg: impl Into<String>) -> Self {
        Self::Download(msg.into())
    }

    pub fn verify(msg: impl Into<String>) -> Self {
        Self::Verify(msg.into())
    }

    pub fn engine(msg: impl Into<String>) -> Self {
        Self::Engine(msg.into())
    }

    pub fn invalid(msg: impl Into<String>) -> Self {
        Self::InvalidRequest(msg.into())
    }
}
