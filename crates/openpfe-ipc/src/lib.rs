//! Domain-free IPC: length-prefixed JSON envelopes over Unix domain sockets (v1).
//!
//! Normative wire format: [openpfe-ipc specification](https://github.com/npetzall/openpfe/blob/main/.dev/crates/openpfe-ipc/specification.md).

mod adapters;
mod client;
mod connection;
mod envelope;
mod error;
mod frame;
mod handler;
mod server;
mod transport;

pub use client::IpcClient;
pub use connection::{Connection, FramedConnection};
pub use envelope::{
    DEFAULT_SOCKET_PATH, EchoResponse, Envelope, EnvelopeType, ErrorCode, MAX_FRAME_BODY,
    PROTOCOL_VERSION,
};
pub use error::{IpcError, ProtocolError};
pub use frame::{FrameCodec, LeJsonCodec};
pub use handler::RequestHandler;
pub use server::IpcListener;
pub use transport::{IpcListenerTransport, IpcTransport};

pub use adapters::unix::{UnixConnection, UnixListenerTransport, UnixTransport};
