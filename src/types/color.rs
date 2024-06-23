use crate::{error::Error, Read, Write};
use ordered_float::OrderedFloat;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Color {
    pub r: OrderedFloat<f32>,
    pub g: OrderedFloat<f32>,
    pub b: OrderedFloat<f32>,
    pub a: OrderedFloat<f32>,
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
