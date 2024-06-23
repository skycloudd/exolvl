use super::{image::Image, object::Object};
use crate::{error::Error, Read, Write};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct Prefab {
    pub prefab_id: i32,
    pub prefab_image_data: Image,
    pub items: Vec<Object>,
}

impl Read for Prefab {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            prefab_id: Read::read(input)?,
            prefab_image_data: Read::read(input)?,
            items: Read::read(input)?,
        })
    }
}

impl Write for Prefab {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.prefab_id.write(output)?;
        self.prefab_image_data.write(output)?;
        self.items.write(output)
    }
}
