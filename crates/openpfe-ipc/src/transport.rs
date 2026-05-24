//! Transport abstraction (connect / bind / accept).

use std::future::Future;
use std::path::Path;
use std::pin::Pin;

use crate::connection::Connection;
use crate::error::IpcError;

/// Pluggable transport (Unix v1; Windows later).
pub trait IpcTransport: Send + Sync {
    type Conn: Connection;

    fn connect(path: &Path) -> Pin<Box<dyn Future<Output = Result<Self::Conn, IpcError>> + Send>>;

    fn bind(path: &Path) -> Pin<Box<dyn Future<Output = Result<Self::Listener, IpcError>> + Send>>;

    type Listener: IpcListenerTransport<Conn = Self::Conn>;
}

/// Accept side of [`IpcTransport`].
pub trait IpcListenerTransport: Send {
    type Conn: Connection;

    fn accept(&mut self)
    -> Pin<Box<dyn Future<Output = Result<Self::Conn, IpcError>> + Send + '_>>;
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;

    use super::*;
    use crate::connection::FramedConnection;
    use tokio::io::{DuplexStream, duplex};

    struct MockTransport;

    struct MockListener {
        pending: VecDeque<MockConn>,
    }

    type MockConn =
        FramedConnection<tokio::io::ReadHalf<DuplexStream>, tokio::io::WriteHalf<DuplexStream>>;

    impl IpcTransport for MockTransport {
        type Conn = MockConn;

        fn connect(
            _path: &std::path::Path,
        ) -> Pin<Box<dyn Future<Output = Result<Self::Conn, IpcError>> + Send>> {
            Box::pin(async move {
                let (client, server) = duplex(64 * 1024);
                let (cr, cw) = tokio::io::split(client);
                let _server = server;
                Ok(FramedConnection::new(cr, cw))
            })
        }

        fn bind(
            _path: &std::path::Path,
        ) -> Pin<Box<dyn Future<Output = Result<Self::Listener, IpcError>> + Send>> {
            Box::pin(async move {
                Ok(MockListener {
                    pending: VecDeque::new(),
                })
            })
        }

        type Listener = MockListener;
    }

    impl IpcListenerTransport for MockListener {
        type Conn = MockConn;

        fn accept(
            &mut self,
        ) -> Pin<Box<dyn Future<Output = Result<Self::Conn, IpcError>> + Send + '_>> {
            Box::pin(async move {
                self.pending.pop_front().ok_or_else(|| {
                    IpcError::Protocol(crate::error::ProtocolError::InvalidFrame(
                        "mock listener empty".into(),
                    ))
                })
            })
        }
    }

    #[tokio::test]
    async fn mock_transport_connect() {
        let conn = MockTransport::connect(std::path::Path::new("/noop"))
            .await
            .expect("connect");
        let _ = conn;
    }

    #[tokio::test]
    async fn mock_listener_accepts_pushed_conn() {
        let (a, b) = duplex(1024);
        let (ar, aw) = tokio::io::split(a);
        let conn = FramedConnection::new(ar, aw);
        let mut listener = MockListener {
            pending: VecDeque::from([conn]),
        };
        let accepted = listener.accept().await.expect("accept");
        let _ = accepted;
        let _ = b;
    }
}
