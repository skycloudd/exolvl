use super::nova_value::NovaValue;
use crate::{error::Error, Read, Write};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Activator {
    pub activator_type: i32,
    pub parameters: Vec<NovaValue>,
}

impl Read for Activator {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            activator_type: Read::read(input)?,
            parameters: Read::read(input)?,
        })
    }
}

impl Write for Activator {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.activator_type.write(output)?;
        self.parameters.write(output)
    }
}
