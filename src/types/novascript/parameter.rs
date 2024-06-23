use super::{nova_value::NovaValue, static_type::StaticType};
use crate::{error::Error, Read, Write};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Parameter {
    pub parameter_id: i32,
    pub name: String,
    pub static_type: StaticType,
    pub default_value: NovaValue,
}

impl Read for Parameter {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            parameter_id: Read::read(input)?,
            name: Read::read(input)?,
            static_type: Read::read(input)?,
            default_value: Read::read(input)?,
        })
    }
}

impl Write for Parameter {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.parameter_id.write(output)?;
        self.name.write(output)?;
        self.static_type.write(output)?;
        self.default_value.write(output)
    }
}
