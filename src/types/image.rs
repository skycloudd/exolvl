use crate::{error::Error, Read, Write};
#[cfg(feature = "image")]
use image::{DynamicImage, ImageFormat, RgbaImage};

#[cfg(feature = "image")]
#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub struct Image(pub RgbaImage);

#[cfg(all(feature = "image", feature = "serde"))]
impl serde::Serialize for Image {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.to_vec().serialize(serializer)
    }
}

#[cfg(all(feature = "image", feature = "serde"))]
impl<'de> serde::Deserialize<'de> for Image {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let buffer = serde::Deserialize::deserialize(deserializer)?;

        let img = image::load_from_memory(buffer).map_err(serde::de::Error::custom)?;

        let img = img.to_rgba8();

        Ok(Self(img))
    }
}

#[cfg(not(feature = "image"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Image(pub Vec<u8>);

impl Read for Image {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let data = Read::read(input)?;

        Ok(Self(data))
    }
}

impl Write for Image {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.0.write(output)
    }
}

#[cfg(feature = "image")]
impl Read for DynamicImage {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let vec = Vec::<u8>::read(input)?;

        image::load_from_memory(&vec).map_err(Error::from)
    }
}

#[cfg(feature = "image")]
impl Write for DynamicImage {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        let mut vec = std::io::Cursor::new(Vec::new());
        self.write_to(&mut vec, ImageFormat::Png)?;

        output.write_all(&vec.into_inner())?;

        Ok(())
    }
}

#[cfg(feature = "image")]
impl Read for RgbaImage {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let vec = Vec::<u8>::read(input)?;

        image::load_from_memory(&vec)
            .map_err(Error::from)
            .map(|img| img.to_rgba8())
    }
}

#[cfg(feature = "image")]
impl Write for RgbaImage {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        let mut vec = std::io::Cursor::new(Vec::new());
        self.write_to(&mut vec, ImageFormat::Png)?;

        output.write_all(&vec.into_inner())?;

        Ok(())
    }
}
