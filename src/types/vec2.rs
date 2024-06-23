use ordered_float::OrderedFloat;

use crate::{error::Error, Read, Write};

/// A 2D vector.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Vec2 {
    /// The x-coordinate.
    pub x: OrderedFloat<f32>,
    /// The y-coordinate.
    pub y: OrderedFloat<f32>,
}

impl Read for Vec2 {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            x: OrderedFloat(Read::read(input)?),
            y: OrderedFloat(Read::read(input)?),
        })
    }
}

impl Write for Vec2 {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.x.write(output)?;
        self.y.write(output)
    }
}
