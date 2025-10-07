use {
    crate::error::HeaderError,
    zerocopy::{error::CastError, IntoBytes, Ref},
    zerocopy_derive::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned},
};

/// Protocol wire-format constants.
pub const MAGIC: [u8; 4] = *b"A2A!";
pub const VERSION: u16 = 1;

#[repr(u8)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kind {
    Control = 0,
    Game = 1,
    Error = 2,
}

impl TryFrom<u8> for Kind {
    type Error = HeaderError;

    #[inline]
    fn try_from(value: u8) -> Result<Self, HeaderError> {
        match value {
            0 => Ok(Self::Control),
            1 => Ok(Self::Game),
            2 => Ok(Self::Error),
            other => Err(HeaderError::InvalidKind(other)),
        }
    }
}

impl From<Kind> for u8 {
    #[inline]
    fn from(kind: Kind) -> Self {
        kind as u8
    }
}

#[repr(C)]
#[derive(
    FromBytes,
    IntoBytes,
    KnownLayout,
    Immutable,
    Unaligned,
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
)]
struct RawHeader {
    magic: [u8; 4],
    version: [u8; 2],
    kind: u8,
    flags: u8,
    payload_len: [u8; 4],
}

impl RawHeader {
    const SIZE: usize = core::mem::size_of::<Self>();

    #[inline]
    fn new(kind: Kind, flags: u8, payload_len: u32) -> Self {
        Self {
            magic: MAGIC,
            version: VERSION.to_be_bytes(),
            kind: kind.into(),
            flags,
            payload_len: payload_len.to_be_bytes(),
        }
    }

    #[inline]
    fn version(&self) -> u16 {
        u16::from_be_bytes(self.version)
    }

    #[inline]
    fn payload_len(&self) -> u32 {
        u32::from_be_bytes(self.payload_len)
    }
}

/// Header metadata for a protocol frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Header {
    raw: RawHeader,
}

impl Header {
    pub const SIZE: usize = RawHeader::SIZE;

    #[inline]
    pub fn new(kind: Kind, flags: u8, payload_len: u32) -> Self {
        Self { raw: RawHeader::new(kind, flags, payload_len) }
    }

    #[inline]
    pub fn kind(&self) -> Kind {
        Kind::try_from(self.raw.kind).expect("stored enum discriminant is valid")
    }

    #[inline]
    pub fn flags(&self) -> u8 {
        self.raw.flags
    }

    #[inline]
    pub fn payload_len(&self) -> u32 {
        self.raw.payload_len()
    }

    #[inline]
    pub fn version(&self) -> u16 {
        self.raw.version()
    }

    #[inline]
    pub fn with_flags(mut self, flags: u8) -> Self {
        self.raw.flags = flags;
        self
    }

    #[inline]
    pub fn with_payload_len(mut self, payload_len: u32) -> Self {
        self.raw.payload_len = payload_len.to_be_bytes();
        self
    }

    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        self.raw.as_bytes()
    }

    pub fn parse(bytes: &[u8]) -> Result<Self, HeaderError> {
        if bytes.len() < Self::SIZE {
            return Err(HeaderError::Truncated);
        }

        let raw_ref = Ref::<_, RawHeader>::from_bytes(&bytes[..Self::SIZE])
            .map_err(map_cast_error)?;
        let raw = *raw_ref;

        if raw.magic != MAGIC {
            return Err(HeaderError::InvalidMagic(raw.magic));
        }

        if raw.version() != VERSION {
            return Err(HeaderError::UnsupportedVersion {
                found: raw.version(),
                expected: VERSION,
            });
        }

        Kind::try_from(raw.kind)?;

        Ok(Self { raw })
    }
}

fn map_cast_error(err: CastError<&[u8], RawHeader>) -> HeaderError {
    match err {
        CastError::Alignment(_) => HeaderError::Misaligned,
        CastError::Size(_) => HeaderError::Truncated,
        CastError::Validity(_) => unreachable!("RawHeader has no invalid states"),
    }
}
