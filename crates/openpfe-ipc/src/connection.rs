//! Framed envelope read/write over a byte stream.

use std::future::Future;
use std::pin::Pin;

use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

use crate::envelope::Envelope;
use crate::error::{IpcError, ProtocolError};
use crate::frame::{FrameCodec, LeJsonCodec, length_prefix_len};

/// One framed envelope read or write per call.
pub trait Connection: Send {
    fn read_envelope(
        &mut self,
    ) -> Pin<Box<dyn Future<Output = Result<Envelope, IpcError>> + Send + '_>>;

    fn write_envelope<'a>(
        &'a mut self,
        envelope: &'a Envelope,
    ) -> Pin<Box<dyn Future<Output = Result<(), IpcError>> + Send + 'a>>;
}

/// [`Connection`] over any async read/write pair using [`LeJsonCodec`].
pub struct FramedConnection<R, W> {
    reader: R,
    writer: W,
    codec: LeJsonCodec,
    read_buf: Vec<u8>,
}

impl<R, W> FramedConnection<R, W>
where
    R: AsyncRead + Unpin + Send,
    W: AsyncWrite + Unpin + Send,
{
    pub fn new(reader: R, writer: W) -> Self {
        Self {
            reader,
            writer,
            codec: LeJsonCodec,
            read_buf: Vec::new(),
        }
    }
}

impl<R, W> Connection for FramedConnection<R, W>
where
    R: AsyncRead + Unpin + Send,
    W: AsyncWrite + Unpin + Send,
{
    fn read_envelope(
        &mut self,
    ) -> Pin<Box<dyn Future<Output = Result<Envelope, IpcError>> + Send + '_>> {
        Box::pin(async move {
            loop {
                if self.read_buf.len() >= 4 {
                    let frame_len = length_prefix_len(&self.read_buf)?;
                    if self.read_buf.len() >= frame_len {
                        let (envelope, consumed) =
                            self.codec.decode_frame(&self.read_buf[..frame_len])?;
                        self.read_buf.drain(..consumed);
                        return Ok(envelope);
                    }
                }
                let mut chunk = [0u8; 4096];
                let n = self.reader.read(&mut chunk).await?;
                if n == 0 {
                    if self.read_buf.is_empty() {
                        return Err(IpcError::Protocol(ProtocolError::InvalidFrame(
                            "connection closed".into(),
                        )));
                    }
                    return Err(IpcError::Protocol(ProtocolError::Incomplete {
                        needed: 4,
                        available: self.read_buf.len(),
                    }));
                }
                self.read_buf.extend_from_slice(&chunk[..n]);
            }
        })
    }

    fn write_envelope<'a>(
        &'a mut self,
        envelope: &'a Envelope,
    ) -> Pin<Box<dyn Future<Output = Result<(), IpcError>> + Send + 'a>> {
        Box::pin(async move {
            let frame = self.codec.encode(envelope)?;
            self.writer.write_all(&frame).await?;
            self.writer.flush().await?;
            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use std::sync::{Arc, Mutex};

    use super::*;
    use crate::envelope::{Envelope, EnvelopeType};

    struct MockConn {
        read_q: Arc<Mutex<Vec<u8>>>,
        write_q: Arc<Mutex<Vec<u8>>>,
        codec: LeJsonCodec,
        read_buf: Vec<u8>,
    }

    impl Connection for MockConn {
        fn read_envelope(
            &mut self,
        ) -> Pin<Box<dyn Future<Output = Result<Envelope, IpcError>> + Send + '_>> {
            Box::pin(async move {
                loop {
                    if self.read_buf.len() >= 4 {
                        let frame_len = length_prefix_len(&self.read_buf)?;
                        if self.read_buf.len() >= frame_len {
                            let (envelope, consumed) =
                                self.codec.decode_frame(&self.read_buf[..frame_len])?;
                            self.read_buf.drain(..consumed);
                            return Ok(envelope);
                        }
                    }
                    let mut read_q = self.read_q.lock().expect("lock");
                    if read_q.is_empty() {
                        return Err(IpcError::Protocol(ProtocolError::InvalidFrame(
                            "mock exhausted".into(),
                        )));
                    }
                    let chunk = read_q.drain(..).collect::<Vec<_>>();
                    drop(read_q);
                    self.read_buf.extend(chunk);
                }
            })
        }

        fn write_envelope<'a>(
            &'a mut self,
            envelope: &'a Envelope,
        ) -> Pin<Box<dyn Future<Output = Result<(), IpcError>> + Send + 'a>> {
            Box::pin(async move {
                let frame = self.codec.encode(envelope)?;
                self.write_q.lock().expect("lock").extend(frame);
                Ok(())
            })
        }
    }

    #[tokio::test]
    async fn mock_connection_roundtrip() {
        let codec = LeJsonCodec;
        let req = Envelope::request(3, EnvelopeType::Echo, serde_json::json!({}));
        let frame = codec.encode(&req).expect("encode");
        let read_q = Arc::new(Mutex::new(frame));
        let write_q = Arc::new(Mutex::new(Vec::new()));
        let mut conn = MockConn {
            read_q: Arc::clone(&read_q),
            write_q: Arc::clone(&write_q),
            codec: LeJsonCodec,
            read_buf: Vec::new(),
        };
        let resp = Envelope::request(3, EnvelopeType::Echo, serde_json::json!({ "ok": true }));
        read_q
            .lock()
            .expect("lock")
            .extend(codec.encode(&resp).expect("enc"));
        let got = conn.read_envelope().await.expect("read");
        assert_eq!(got.id, 3);
        conn.write_envelope(&req).await.expect("write");
        assert!(!write_q.lock().expect("lock").is_empty());
    }
}
