use {
    crate::{
        error::ProtoError,
        header::{Header, Kind},
    },
    bytes::{Bytes, BytesMut},
    tokio_util::codec::{Decoder, Encoder},
};

/// Zero-copy frame consisting of a validated header and a payload slice.
#[derive(Debug, Clone)]
pub struct Frame {
    header: Header,
    payload: Bytes,
}

impl Frame {
    /// Creates a frame using the provided kind, flags and payload.
    #[inline]
    pub fn new(kind: Kind, flags: u8, payload: Bytes) -> Self {
        debug_assert!(payload.len() <= u32::MAX as usize);
        let header = Header::new(kind, flags, payload.len() as u32);
        Self { header, payload }
    }

    /// Reconstructs a frame from raw parts, ensuring the payload length matches the header.
    pub fn from_parts(header: Header, payload: Bytes) -> Result<Self, ProtoError> {
        if header.payload_len() as usize != payload.len() {
            return Err(ProtoError::LengthMismatch {
                expected: header.payload_len(),
                actual: payload.len(),
            });
        }
        Ok(Self { header, payload })
    }

    #[inline]
    pub fn header(&self) -> Header {
        self.header
    }

    #[inline]
    pub fn payload(&self) -> &Bytes {
        &self.payload
    }

    #[inline]
    pub fn into_payload(self) -> Bytes {
        self.payload
    }
}

/// Tokio codec that performs zero-copy framing for Apples-to-Apples protocol packets.
#[derive(Debug, Clone)]
pub struct FrameCodec {
    max_payload_len: u32,
}

impl FrameCodec {
    /// Creates a codec with the provided maximum payload length in bytes.
    #[inline]
    pub const fn new(max_payload_len: u32) -> Self {
        Self { max_payload_len }
    }

    /// Creates a codec with a conservative default payload ceiling (1 MiB).
    #[inline]
    pub const fn with_default_limit() -> Self {
        Self::new(1 << 20)
    }

    #[inline]
    fn ensure_within_limit(&self, length: u32) -> Result<(), ProtoError> {
        if length > self.max_payload_len {
            return Err(ProtoError::PayloadTooLarge {
                len: length,
                max: self.max_payload_len,
            });
        }
        Ok(())
    }
}

impl Default for FrameCodec {
    fn default() -> Self {
        Self::with_default_limit()
    }
}

impl Decoder for FrameCodec {
    type Item = Frame;
    type Error = ProtoError;

    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Self::Error> {
        const HEADER_SIZE: usize = Header::SIZE;

        if src.len() < HEADER_SIZE {
            return Ok(None);
        }

        let header = Header::parse(&src[..HEADER_SIZE])?;
        self.ensure_within_limit(header.payload_len())?;

        let payload_len = header.payload_len() as usize;
        let total_len =
            HEADER_SIZE
                .checked_add(payload_len)
                .ok_or(ProtoError::PayloadTooLarge {
                    len: header.payload_len(),
                    max: self.max_payload_len,
                })?;

        if src.len() < total_len {
            return Ok(None);
        }

        let mut frame = src.split_to(total_len);
        let payload = frame.split_off(HEADER_SIZE).freeze();

        Frame::from_parts(header, payload).map(Some)
    }
}

impl Encoder<Frame> for FrameCodec {
    type Error = ProtoError;

    fn encode(&mut self, item: Frame, dst: &mut BytesMut) -> Result<(), Self::Error> {
        let payload_len = item.payload.len();
        let payload_len_u32 =
            u32::try_from(payload_len).map_err(|_| ProtoError::PayloadTooLarge {
                len: u32::MAX,
                max: self.max_payload_len,
            })?;

        self.ensure_within_limit(payload_len_u32)?;

        let header = Header::new(item.header.kind(), item.header.flags(), payload_len_u32);

        dst.reserve(Header::SIZE + payload_len);
        dst.extend_from_slice(header.as_bytes());
        dst.extend_from_slice(&item.payload);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use {
        super::{Frame, FrameCodec},
        crate::header::{Header, Kind, MAGIC, VERSION},
        bytes::{Bytes, BytesMut},
        tokio_util::codec::{Decoder, Encoder},
    };

    #[test]
    fn roundtrip_zero_copy() {
        let mut codec = FrameCodec::default();
        let payload = Bytes::from_static(b"payload");
        let frame = Frame::new(Kind::Game, 0, payload.clone());

        let mut buffer = BytesMut::new();
        codec.encode(frame.clone(), &mut buffer).expect("encode");

        let expected_header = Header::new(Kind::Game, 0, payload.len() as u32);
        let mut expected = BytesMut::from(expected_header.as_bytes());
        expected.extend_from_slice(&payload);

        assert_eq!(buffer, expected);

        let decoded = codec.decode(&mut buffer).expect("decode").expect("frame");
        assert_eq!(decoded.payload(), &payload);
        assert_eq!(decoded.header().kind(), Kind::Game);
        assert_eq!(decoded.header().flags(), 0);
        assert_eq!(decoded.header().version(), VERSION);
        assert_eq!(decoded.header().as_bytes()[..4], MAGIC);

        assert!(buffer.is_empty());
    }
}
