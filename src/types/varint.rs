use crate::{error::Error, Read, Write};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Varint(pub u64);

impl Read for Varint {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let value = leb128::read::unsigned(input)?;

        Ok(Self(value))
    }
}

impl Write for Varint {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        leb128::write::unsigned(output, self.0)?;

        Ok(())
    }
}
