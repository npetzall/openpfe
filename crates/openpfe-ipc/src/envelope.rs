//! JSON envelope types per [specification](https://github.com/npetzall/openpfe/blob/main/.dev/crates/openpfe-ipc/specification.md).

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Protocol version carried on every frame (v1).
pub const PROTOCOL_VERSION: u64 = 1;

/// Maximum UTF-8 JSON body size in bytes (length prefix excluded).
pub const MAX_FRAME_BODY: usize = 16_777_216;

/// Default UDS path relative to project root / process cwd.
pub const DEFAULT_SOCKET_PATH: &str = "./.openpfe/server/socket";

/// Envelope `type` field values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EnvelopeType {
    Echo,
    Mcp,
    Shutdown,
    Error,
}

/// `type: error` payload `code` values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ErrorCode {
    UnsupportedVersion,
    UnknownType,
    InvalidFrame,
    PayloadTooLarge,
}

impl std::fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnsupportedVersion => f.write_str("unsupported_version"),
            Self::UnknownType => f.write_str("unknown_type"),
            Self::InvalidFrame => f.write_str("invalid_frame"),
            Self::PayloadTooLarge => f.write_str("payload_too_large"),
        }
    }
}

/// Wire envelope (`v`, `type`, `id`, `payload`).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Envelope {
    pub v: u64,
    #[serde(rename = "type")]
    pub kind: EnvelopeType,
    pub id: u64,
    pub payload: Value,
}

impl Envelope {
    pub fn request(id: u64, kind: EnvelopeType, payload: Value) -> Self {
        Self {
            v: PROTOCOL_VERSION,
            kind,
            id,
            payload,
        }
    }

    pub fn error_response(request_id: u64, code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            v: PROTOCOL_VERSION,
            kind: EnvelopeType::Error,
            id: request_id,
            payload: serde_json::json!({
                "code": code,
                "message": message.into(),
            }),
        }
    }

    pub fn error_code(&self) -> Option<ErrorCode> {
        if self.kind != EnvelopeType::Error {
            return None;
        }
        self.payload
            .get("code")
            .and_then(|v| serde_json::from_value(v.clone()).ok())
    }

    pub fn error_message(&self) -> Option<&str> {
        self.payload.get("message").and_then(|v| v.as_str())
    }
}

/// Parsed fields from a successful `type: echo` response.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EchoResponse {
    pub http_base_url: String,
    pub pid: Option<u64>,
    pub version: Option<String>,
}

impl EchoResponse {
    pub fn from_envelope(envelope: &Envelope) -> Result<Self, crate::error::IpcError> {
        if envelope.kind == EnvelopeType::Error {
            let code = envelope.error_code().unwrap_or(ErrorCode::InvalidFrame);
            let message = envelope
                .error_message()
                .unwrap_or("echo failed")
                .to_string();
            return Err(crate::error::IpcError::remote(code, message));
        }
        if envelope.kind != EnvelopeType::Echo {
            return Err(crate::error::IpcError::UnexpectedType {
                expected: "echo",
                actual: format!("{:?}", envelope.kind),
            });
        }
        let url = envelope
            .payload
            .get("http_base_url")
            .and_then(|v| v.as_str())
            .ok_or(crate::error::IpcError::MissingField("http_base_url"))?;
        let pid = envelope.payload.get("pid").and_then(|v| v.as_u64());
        let version = envelope
            .payload
            .get("version")
            .and_then(|v| v.as_str())
            .map(str::to_owned);
        Ok(Self {
            http_base_url: url.to_string(),
            pid,
            version,
        })
    }
}
