use super::vec2::Vec2;
use crate::{error::Error, Read, Write};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct Layer {
    pub layer_id: i32,
    pub layer_name: String,
    pub selected: bool,
    pub invisible: bool,
    pub locked: bool,
    pub foreground_type: i32,
    pub parallax: Vec2,
    pub fixed_size: bool,
    pub children: Vec<i32>,
}

impl Read for Layer {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            layer_id: Read::read(input)?,
            layer_name: Read::read(input)?,
            selected: Read::read(input)?,
            invisible: Read::read(input)?,
            locked: Read::read(input)?,
            foreground_type: Read::read(input)?,
            parallax: Read::read(input)?,
            fixed_size: Read::read(input)?,
            children: Read::read(input)?,
        })
    }
}

impl Write for Layer {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.layer_id.write(output)?;
        self.layer_name.write(output)?;
        self.selected.write(output)?;
        self.invisible.write(output)?;
        self.locked.write(output)?;
        self.foreground_type.write(output)?;
        self.parallax.write(output)?;
        self.fixed_size.write(output)?;
        self.children.write(output)
    }
}
