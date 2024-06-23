use super::{nova_value::NovaValue, static_type::StaticType};
use crate::{error::Error, Read, Write};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct Variable {
    pub variable_id: i32,
    pub name: String,
    pub static_type: StaticType,
    pub initial_value: NovaValue,
}

impl Read for Variable {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            variable_id: Read::read(input)?,
            name: Read::read(input)?,
            static_type: Read::read(input)?,
            initial_value: Read::read(input)?,
        })
    }
}

impl Write for Variable {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.variable_id.write(output)?;
        self.name.write(output)?;
        self.static_type.write(output)?;
        self.initial_value.write(output)
    }
}
