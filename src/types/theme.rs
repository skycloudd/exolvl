use crate::{error::Error, Read, Write};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Theme {
    #[default]
    Mountains,
    Halloween,
    Christmas,
    Custom,
}

impl Read for Theme {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error>
    where
        Self: Sized,
    {
        match String::read(input)?.as_str() {
            "mountains" => Ok(Self::Mountains),
            "halloween" => Ok(Self::Halloween),
            "christmas" => Ok(Self::Christmas),
            "custom" => Ok(Self::Custom),
            other => Err(Error::InvalidTheme(other.to_owned())),
        }
    }
}

impl Write for Theme {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        match self {
            Self::Mountains => "mountains",
            Self::Halloween => "halloween",
            Self::Christmas => "christmas",
            Self::Custom => "custom",
        }
        .write(output)
    }
}
