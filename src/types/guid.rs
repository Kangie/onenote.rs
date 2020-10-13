use crate::errors::Result;
use crate::Reader;
use std::fmt;
use uuid::Uuid;

#[derive(Clone, Copy, PartialEq, Hash, Eq)]
pub struct Guid(pub Uuid);

impl Guid {
    pub(crate) fn from_str(value: &str) -> Result<Guid> {
        Uuid::parse_str(value).map(Guid).map_err(|e| e.into())
    }

    pub(crate) fn parse(reader: Reader) -> Result<Guid> {
        let v = reader.get_u128()?;

        let guid = Guid(Uuid::from_bytes([
            // Big Endian
            (v >> 24) as u8,
            (v >> 16) as u8,
            (v >> 8) as u8,
            v as u8,
            // Big Endian
            (v >> 40) as u8,
            (v >> 32) as u8,
            // Big Endian
            (v >> 56) as u8,
            (v >> 48) as u8,
            // Little Endian
            (v >> 64) as u8,
            (v >> 72) as u8,
            (v >> 80) as u8,
            (v >> 88) as u8,
            (v >> 96) as u8,
            (v >> 104) as u8,
            (v >> 112) as u8,
            (v >> 120) as u8,
        ]));

        Ok(guid)
    }

    pub(crate) fn nil() -> Guid {
        Guid(Uuid::nil())
    }

    pub(crate) fn is_nil(&self) -> bool {
        self.0.is_nil()
    }
}

impl fmt::Display for Guid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{{:X}}}", self.0)
    }
}

impl fmt::Debug for Guid {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Guid {}", self)
    }
}
