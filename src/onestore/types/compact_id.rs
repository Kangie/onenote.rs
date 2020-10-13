use crate::errors::Result;
use crate::Reader;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
pub(crate) struct CompactId {
    n: u8,
    guid_index: u32,
}

impl CompactId {
    pub(crate) fn parse(reader: Reader) -> Result<CompactId> {
        let data = reader.get_u32()?;

        let n = (data & 0xFF) as u8;
        let guid_index = data >> 8;

        Ok(CompactId { n, guid_index })
    }
}
