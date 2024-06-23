use super::image::Image;
use crate::{error::Error, Read, Write};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, PartialEq)]
pub struct Pattern {
    pub pattern_id: i32,
    pub pattern_frames: Vec<Image>,
}

impl Read for Pattern {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            pattern_id: Read::read(input)?,
            pattern_frames: Read::read(input)?,
        })
    }
}

impl Write for Pattern {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.pattern_id.write(output)?;
        self.pattern_frames.write(output)
    }
}
