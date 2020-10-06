use crate::one::property::PropertyType;
use crate::onestore::object::Object;

#[derive(Debug)]
pub(crate) struct Color {
    alpha: u8,
    r: u8,
    g: u8,
    b: u8,
}

impl Color {
    pub(crate) fn parse(prop_type: PropertyType, object: &Object) -> Option<Color> {
        object
            .props()
            .get(prop_type)
            .map(|value| value.to_u32().expect("color is not a u32"))
            .map(|value| value.to_le_bytes())
            .map(|value| Color {
                alpha: 255 - value[3],
                r: value[0],
                g: value[1],
                b: value[2],
            })
    }
}
