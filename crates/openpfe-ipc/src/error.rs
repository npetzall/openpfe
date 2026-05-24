//! IPC errors: protocol validation vs I/O.

use crate::envelope::ErrorCode;

/// Errors from framing, envelopes, transport, and remote `type: error` responses.
#[derive(Debug, thiserror::Error)]
pub enum IpcError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),

    #[error("json error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("protocol: {0}")]
    Protocol(#[from] ProtocolError),

    #[error("unexpected envelope type: expected {expected}, got {actual}")]
    UnexpectedType {
        expected: &'static str,
        actual: String,
    },

    #[error("missing required field in response: {0}")]
    MissingField(&'static str),

    #[error("remote error {code}: {message}")]
    Remote { code: ErrorCode, message: String },
}

/// Framing and envelope validation failures (local, before handler).
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum ProtocolError {
    #[error("invalid frame: {0}")]
    InvalidFrame(String),

    #[error("payload too large: {length} bytes (max {max})")]
    PayloadTooLarge { length: u32, max: usize },

    #[error("unsupported protocol version: {0}")]
    UnsupportedVersion(u64),

    #[error("incomplete frame: need {needed} bytes, have {available}")]
    Incomplete { needed: usize, available: usize },
}

impl IpcError {
    pub(crate) fn remote(code: ErrorCode, message: impl Into<String>) -> Self {
        Self::Remote {
            code,
            message: message.into(),
        }
    }
}
