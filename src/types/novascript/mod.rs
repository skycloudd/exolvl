use crate::{error::Error, Read, Write};
use action::Action;
use activator::Activator;
use nova_value::NovaValue;
use parameter::Parameter;
use variable::Variable;

pub mod action;
pub mod activator;
pub mod nova_value;
pub mod parameter;
pub mod variable;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct NovaScript {
    pub script_id: i32,
    pub script_name: String,
    pub is_function: bool,
    pub activation_count: i32,
    pub condition: NovaValue,
    pub activation_list: Vec<Activator>,
    pub parameters: Vec<Parameter>,
    pub variables: Vec<Variable>,
    pub actions: Vec<Action>,
}

impl Read for NovaScript {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            script_id: Read::read(input)?,
            script_name: Read::read(input)?,
            is_function: Read::read(input)?,
            activation_count: Read::read(input)?,
            condition: Read::read(input)?,
            activation_list: Read::read(input)?,
            parameters: Read::read(input)?,
            variables: Read::read(input)?,
            actions: Read::read(input)?,
        })
    }
}

impl Write for NovaScript {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.script_id.write(output)?;
        self.script_name.write(output)?;
        self.is_function.write(output)?;
        self.activation_count.write(output)?;
        self.condition.write(output)?;
        self.activation_list.write(output)?;
        self.parameters.write(output)?;
        self.variables.write(output)?;
        self.actions.write(output)
    }
}
