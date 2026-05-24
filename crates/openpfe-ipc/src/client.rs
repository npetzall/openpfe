//! IPC client (`echo`, `shutdown`, `send_mcp`).

use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};

use serde_json::Value;

use crate::adapters::unix::{UnixConnection, UnixTransport};
use crate::connection::Connection;
use crate::envelope::{
    DEFAULT_SOCKET_PATH, EchoResponse, Envelope, EnvelopeType, ErrorCode, PROTOCOL_VERSION,
};
use crate::error::IpcError;
use crate::transport::IpcTransport;

/// Client for the project control socket ([`DEFAULT_SOCKET_PATH`] by default).
pub struct IpcClient {
    conn: UnixConnection,
    next_id: AtomicU64,
}

impl IpcClient {
    pub async fn connect(socket_path: impl AsRef<Path>) -> Result<Self, IpcError> {
        let conn = UnixTransport::connect(socket_path.as_ref()).await?;
        Ok(Self {
            conn,
            next_id: AtomicU64::new(1),
        })
    }

    pub async fn connect_default() -> Result<Self, IpcError> {
        Self::connect(DEFAULT_SOCKET_PATH).await
    }

    fn next_id(&self) -> u64 {
        self.next_id.fetch_add(1, Ordering::Relaxed)
    }

    async fn roundtrip(
        &mut self,
        kind: EnvelopeType,
        payload: Value,
    ) -> Result<Envelope, IpcError> {
        let id = self.next_id();
        let req = Envelope::request(id, kind, payload);
        self.conn.write_envelope(&req).await?;
        let resp = self.conn.read_envelope().await?;
        if resp.kind == EnvelopeType::Error {
            let code = resp.error_code().unwrap_or(ErrorCode::InvalidFrame);
            let message = resp.error_message().unwrap_or("request failed").to_string();
            return Err(IpcError::remote(code, message));
        }
        if resp.id != id {
            return Err(IpcError::Protocol(
                crate::error::ProtocolError::InvalidFrame(format!(
                    "response id mismatch: expected {id}, got {}",
                    resp.id
                )),
            ));
        }
        Ok(resp)
    }

    /// Admin `type: echo` — returns `http_base_url` and optional metadata.
    pub async fn echo(&mut self) -> Result<EchoResponse, IpcError> {
        let resp = self
            .roundtrip(EnvelopeType::Echo, serde_json::json!({}))
            .await?;
        EchoResponse::from_envelope(&resp)
    }

    /// Admin `type: shutdown`.
    pub async fn shutdown(&mut self) -> Result<(), IpcError> {
        let resp = self
            .roundtrip(
                EnvelopeType::Shutdown,
                serde_json::json!({ "graceful": true }),
            )
            .await?;
        if resp.kind != EnvelopeType::Shutdown {
            return Err(IpcError::UnexpectedType {
                expected: "shutdown",
                actual: format!("{:?}", resp.kind),
            });
        }
        let ok = resp
            .payload
            .get("ok")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        if !ok {
            return Err(IpcError::MissingField("ok"));
        }
        Ok(())
    }

    /// `type: mcp` with opaque JSON-RPC (or batch) payload.
    pub async fn send_mcp(&mut self, payload: Value) -> Result<Value, IpcError> {
        let resp = self.roundtrip(EnvelopeType::Mcp, payload).await?;
        if resp.kind != EnvelopeType::Mcp {
            return Err(IpcError::UnexpectedType {
                expected: "mcp",
                actual: format!("{:?}", resp.kind),
            });
        }
        Ok(resp.payload)
    }
}

/// Validate envelope version and known `type` before server handler dispatch.
pub(crate) fn validate_request(envelope: &Envelope) -> Result<(), Envelope> {
    if envelope.v != PROTOCOL_VERSION {
        return Err(Envelope::error_response(
            envelope.id,
            ErrorCode::UnsupportedVersion,
            format!("unsupported version {}", envelope.v),
        ));
    }
    match envelope.kind {
        EnvelopeType::Echo | EnvelopeType::Mcp | EnvelopeType::Shutdown => Ok(()),
        EnvelopeType::Error => Err(Envelope::error_response(
            envelope.id,
            ErrorCode::UnknownType,
            "client must not send type error",
        )),
    }
}
