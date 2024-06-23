use crate::{error::Error, Read, Write};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Default)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Read for Color {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            r: Read::read(input)?,
            g: Read::read(input)?,
            b: Read::read(input)?,
            a: Read::read(input)?,
        })
    }
}

impl Write for Color {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.r.write(output)?;
        self.g.write(output)?;
        self.b.write(output)?;
        self.a.write(output)
    }
}
