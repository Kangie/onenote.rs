use crate::errors::Result;
use crate::fsshttpb::data_element::DataElement;
use crate::types::compact_u64::CompactU64;
use crate::types::exguid::ExGuid;
use crate::types::object_types::ObjectType;
use crate::types::stream_object::ObjectHeader;
use crate::Reader;

#[derive(Debug)]
pub(crate) struct DataElementFragment {
    pub(crate) id: ExGuid,
    pub(crate) size: u64,
    pub(crate) chunk_reference: DataElementFragmentChunkReference,
    pub(crate) data: Vec<u8>,
}

#[derive(Debug)]
pub(crate) struct DataElementFragmentChunkReference {
    pub(crate) offset: u64,
    pub(crate) length: u64,
}

impl DataElement {
    pub(crate) fn parse_data_element_fragment(reader: Reader) -> Result<DataElementFragment> {
        ObjectHeader::try_parse(reader, ObjectType::DataElementFragment)?;

        let id = ExGuid::parse(reader)?;
        let size = CompactU64::parse(reader)?.value();
        let offset = CompactU64::parse(reader)?.value();
        let length = CompactU64::parse(reader)?.value();

        let data = reader.read(size as usize)?.to_vec();

        let chunk_reference = DataElementFragmentChunkReference { offset, length };
        let fragment = DataElementFragment {
            id,
            size,
            chunk_reference,
            data,
        };

        Ok(fragment)
    }
}
