//! Length-prefixed JSON framing (4-byte LE, 16 MiB cap).

use crate::envelope::{Envelope, MAX_FRAME_BODY};
use crate::error::{IpcError, ProtocolError};

/// Encode/decode length-prefixed JSON envelope bodies.
pub trait FrameCodec: Send + Sync {
    fn encode(&self, envelope: &Envelope) -> Result<Vec<u8>, IpcError>;
    fn decode_body(&self, body: &[u8]) -> Result<Envelope, IpcError>;
    fn decode_frame(&self, frame: &[u8]) -> Result<(Envelope, usize), IpcError>;
}

/// 4-byte little-endian length + UTF-8 JSON envelope body.
#[derive(Debug, Clone, Copy, Default)]
pub struct LeJsonCodec;

impl FrameCodec for LeJsonCodec {
    fn encode(&self, envelope: &Envelope) -> Result<Vec<u8>, IpcError> {
        let body = serde_json::to_vec(envelope)?;
        if body.len() > MAX_FRAME_BODY {
            return Err(IpcError::Protocol(ProtocolError::PayloadTooLarge {
                length: body.len() as u32,
                max: MAX_FRAME_BODY,
            }));
        }
        let mut out = Vec::with_capacity(4 + body.len());
        out.extend_from_slice(&(body.len() as u32).to_le_bytes());
        out.extend_from_slice(&body);
        Ok(out)
    }

    fn decode_body(&self, body: &[u8]) -> Result<Envelope, IpcError> {
        serde_json::from_slice(body).map_err(IpcError::from)
    }

    fn decode_frame(&self, frame: &[u8]) -> Result<(Envelope, usize), IpcError> {
        if frame.len() < 4 {
            return Err(IpcError::Protocol(ProtocolError::Incomplete {
                needed: 4,
                available: frame.len(),
            }));
        }
        let length = u32::from_le_bytes([frame[0], frame[1], frame[2], frame[3]]);
        if length == 0 || length as usize > MAX_FRAME_BODY {
            return Err(IpcError::Protocol(ProtocolError::InvalidFrame(format!(
                "invalid length prefix: {length}"
            ))));
        }
        let total = 4 + length as usize;
        if frame.len() < total {
            return Err(IpcError::Protocol(ProtocolError::Incomplete {
                needed: total,
                available: frame.len(),
            }));
        }
        let body = &frame[4..total];
        let envelope = self.decode_body(body)?;
        Ok((envelope, total))
    }
}

pub(crate) fn length_prefix_len(frame: &[u8]) -> Result<usize, IpcError> {
    if frame.len() < 4 {
        return Err(IpcError::Protocol(ProtocolError::Incomplete {
            needed: 4,
            available: frame.len(),
        }));
    }
    let length = u32::from_le_bytes([frame[0], frame[1], frame[2], frame[3]]);
    if length == 0 || length as usize > MAX_FRAME_BODY {
        return Err(IpcError::Protocol(ProtocolError::InvalidFrame(format!(
            "invalid length prefix: {length}"
        ))));
    }
    Ok(4 + length as usize)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::envelope::{Envelope, EnvelopeType, PROTOCOL_VERSION};

    #[test]
    fn le_json_codec_roundtrip() {
        let codec = LeJsonCodec;
        let envelope = Envelope::request(7, EnvelopeType::Echo, serde_json::json!({}));
        let frame = codec.encode(&envelope).expect("encode");
        let (decoded, consumed) = codec.decode_frame(&frame).expect("decode");
        assert_eq!(consumed, frame.len());
        assert_eq!(decoded, envelope);
        assert_eq!(decoded.v, PROTOCOL_VERSION);
    }

    #[test]
    fn oversize_body_rejected_on_encode() {
        let codec = LeJsonCodec;
        let huge = "x".repeat(MAX_FRAME_BODY + 1);
        let envelope = Envelope::request(1, EnvelopeType::Mcp, serde_json::json!({ "data": huge }));
        let err = codec.encode(&envelope).unwrap_err();
        assert!(matches!(
            err,
            IpcError::Protocol(ProtocolError::PayloadTooLarge { .. })
        ));
    }

    #[test]
    fn bad_length_prefix() {
        let codec = LeJsonCodec;
        let frame = [0u8, 0, 0, 0];
        let err = codec.decode_frame(&frame).unwrap_err();
        assert!(matches!(
            err,
            IpcError::Protocol(ProtocolError::InvalidFrame(_))
        ));
    }

    #[test]
    fn length_exceeds_max() {
        let codec = LeJsonCodec;
        let length = (MAX_FRAME_BODY as u32).saturating_add(1);
        let frame = length.to_le_bytes().to_vec();
        let err = codec.decode_frame(&frame).unwrap_err();
        assert!(matches!(
            err,
            IpcError::Protocol(ProtocolError::InvalidFrame(_))
        ));
    }

    #[test]
    fn bad_json_body() {
        let codec = LeJsonCodec;
        let body = b"not-json";
        let mut frame = (body.len() as u32).to_le_bytes().to_vec();
        frame.extend_from_slice(body);
        let err = codec.decode_frame(&frame).unwrap_err();
        assert!(matches!(err, IpcError::Json(_)));
    }
}
