use thiserror::Error;

#[derive(Debug, Error)]
pub enum GraphError {
    #[error("grafeo: {0}")]
    Grafeo(#[from] grafeo::Error),
    #[error("{0}")]
    Message(String),
}

pub type Result<T> = std::result::Result<T, GraphError>;

impl GraphError {
    pub fn msg(s: impl Into<String>) -> Self {
        Self::Message(s.into())
    }
}
