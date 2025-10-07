use thiserror::Error;

/// Parsing failure for the protocol header.
#[derive(Debug, Error, Clone, PartialEq, Eq)]
pub enum HeaderError {
    #[error("incomplete header")]
    Truncated,
    #[error("header pointer misaligned")]
    Misaligned,
    #[error("invalid magic value: {0:?}")]
    InvalidMagic([u8; 4]),
    #[error("unsupported protocol version {found} (expected {expected})")]
    UnsupportedVersion { found: u16, expected: u16 },
    #[error("unknown message kind {0}")]
    InvalidKind(u8),
}

/// High-level protocol errors surfaced by the codec.
#[derive(Debug, Error)]
pub enum ProtoError {
    #[error(transparent)]
    Header(#[from] HeaderError),
    #[error("payload length {len} exceeds maximum {max}")]
    PayloadTooLarge { len: u32, max: u32 },
    #[error("payload length mismatch (header {expected}, actual {actual})")]
    LengthMismatch { expected: u32, actual: usize },
    #[error(transparent)]
    Io(#[from] std::io::Error),
}
