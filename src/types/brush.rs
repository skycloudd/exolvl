use ordered_float::OrderedFloat;

use super::{object_property::ObjectProperty, vec2::Vec2};
use crate::{error::Error, Read, Write};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Brush {
    pub brush_id: i32,
    pub spread: Vec2,
    pub frequency: OrderedFloat<f32>,
    pub grid: BrushGrid,
    pub objects: Vec<BrushObject>,
}

impl Read for Brush {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            brush_id: Read::read(input)?,
            spread: Read::read(input)?,
            frequency: Read::read(input)?,
            grid: Read::read(input)?,
            objects: Read::read(input)?,
        })
    }
}

impl Write for Brush {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.brush_id.write(output)?;
        self.spread.write(output)?;
        self.frequency.write(output)?;
        self.grid.write(output)?;
        self.objects.write(output)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::module_name_repetitions)]
pub struct BrushObject {
    pub entity_id: i32,
    pub properties: Vec<ObjectProperty>,
    pub weight: OrderedFloat<f32>,
    pub scale: OrderedFloat<f32>,
    pub rotation: OrderedFloat<f32>,
    pub flip_x: bool,
    pub flip_y: bool,
}

impl Read for BrushObject {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            entity_id: Read::read(input)?,
            properties: Read::read(input)?,
            weight: Read::read(input)?,
            scale: Read::read(input)?,
            rotation: Read::read(input)?,
            flip_x: Read::read(input)?,
            flip_y: Read::read(input)?,
        })
    }
}

impl Write for BrushObject {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.entity_id.write(output)?;
        self.properties.write(output)?;
        self.weight.write(output)?;
        self.scale.write(output)?;
        self.rotation.write(output)?;
        self.flip_x.write(output)?;
        self.flip_y.write(output)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[allow(clippy::module_name_repetitions)]
pub struct BrushGrid {
    pub x: i32,
    pub y: i32,
}

impl Read for BrushGrid {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            x: Read::read(input)?,
            y: Read::read(input)?,
        })
    }
}

impl Write for BrushGrid {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.x.write(output)?;
        self.y.write(output)
    }
}
