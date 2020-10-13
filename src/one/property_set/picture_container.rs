use crate::errors::Result;
use crate::one::property::{simple, PropertyType};
use crate::one::property_set::PropertySetId;
use crate::onestore::object::Object;
use crate::ErrorKind;

#[derive(Debug)]
pub(crate) struct Data {
    pub(crate) data: Vec<u8>,
    pub(crate) extension: Option<String>,
}

pub(crate) fn parse(object: &Object) -> Result<Data> {
    if object.id() != PropertySetId::PictureContainer.as_jcid()
        && object.id() != PropertySetId::XpsContainer.as_jcid()
    {
        return Err(ErrorKind::MalformedOneNoteFileData(
            format!("unexpected object type: 0x{:X}", object.id().0).into(),
        )
        .into());
    }

    let data = object.file_data().map(|v| v.to_vec()).unwrap_or_default();
    let extension = simple::parse_string(PropertyType::PictureFileExtension, object)?;

    Ok(Data { data, extension })
}
