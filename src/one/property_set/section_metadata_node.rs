use crate::errors::{ErrorKind, Result};
use crate::one::property::color::Color;
use crate::one::property::{simple, PropertyType};
use crate::one::property_set::PropertySetId;
use crate::onestore::object::Object;

#[derive(Debug)]
pub(crate) struct Data {
    pub(crate) schema_revision_in_order_to_read: u32,
    pub(crate) schema_revision_in_order_to_write: u32,
    pub(crate) display_name: Option<String>,
    pub(crate) color: Option<Color>,
}

pub(crate) fn parse(object: &Object) -> Result<Data> {
    if object.id() != PropertySetId::SectionMetadata.as_jcid() {
        return Err(ErrorKind::MalformedOneNoteFileData(
            format!("unexpected object type: 0x{:X}", object.id().0).into(),
        )
        .into());
    }

    let schema_revision_in_order_to_read =
        simple::parse_u32(PropertyType::SchemaRevisionInOrderToRead, object)?.ok_or_else(|| {
            ErrorKind::MalformedOneNoteFileData(
                "section metadata has no schema revision in order to read".into(),
            )
        })?;
    let schema_revision_in_order_to_write =
        simple::parse_u32(PropertyType::SchemaRevisionInOrderToWrite, object)?.ok_or_else(
            || {
                ErrorKind::MalformedOneNoteFileData(
                    "section metadata has no schema revision in order to write".into(),
                )
            },
        )?;
    let display_name = simple::parse_string(PropertyType::SectionDisplayName, object)?;
    let color = Color::parse(PropertyType::SectionColor, object)?;

    let data = Data {
        schema_revision_in_order_to_read,
        schema_revision_in_order_to_write,
        display_name,
        color,
    };

    Ok(data)
}
