#![forbid(unsafe_code)]
#![warn(clippy::pedantic)]
#![warn(clippy::nursery)]
#![warn(missing_copy_implementations)]
#![warn(missing_debug_implementations)]
// #![warn(missing_docs)] // uncomment when writing docs
#![allow(clippy::struct_excessive_bools)]
#![allow(clippy::too_many_lines)]
#![cfg_attr(target_os = "windows", doc=include_str!("..\\README.md"))]
#![cfg_attr(not(target_os = "windows"), doc=include_str!("../README.md"))]

pub mod error;
mod primitive_impls;
mod private;
pub mod traits;
pub mod types;

use error::Error;
pub use traits::{Read, ReadContext, ReadVersioned, Write};
use types::{action_type::ActionType, color::Colour, dynamic_type::DynamicType, vec2::Vec2};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct Script {
    pub script_id: uuid::Uuid,
    pub name: String,
    pub creation_date: chrono::DateTime<chrono::Utc>,
    pub actions: Vec<OldAction>,
}

impl Read for Script {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            script_id: Read::read(input)?,
            name: Read::read(input)?,
            creation_date: Read::read(input)?,
            actions: Read::read(input)?,
        })
    }
}

impl Write for Script {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.script_id.write(output)?;
        self.name.write(output)?;
        self.creation_date.write(output)?;
        self.actions.write(output)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct OldAction {
    pub action_type: OldActionType,
    pub wait: bool,
    pub properties: Vec<OldActionProperty>,
}

impl Read for OldAction {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            action_type: Read::read(input)?,
            wait: Read::read(input)?,
            properties: Read::read(input)?,
        })
    }
}

impl Write for OldAction {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.action_type.write(output)?;
        self.wait.write(output)?;
        self.properties.write(output)
    }
}

macro_rules! define_old_action_type {
    ($($name:ident = $number:expr),*) => {
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
        pub enum OldActionType {
            $($name = $number),*
        }

        impl TryFrom<i32> for OldActionType {
            type Error = ();

            fn try_from(value: i32) -> Result<Self, Self::Error> {
                match value {
                    $($number => Ok(OldActionType::$name),)*
                    _ => Err(())
                }
            }
        }

        impl From<&OldActionType> for i32 {
            fn from(value: &OldActionType) -> Self {
                match value {
                    $(OldActionType::$name => $number,)*
                }
            }
        }
    };
}

define_old_action_type!(
    RunScript = 0,
    StopScripts = 1,
    Wait = 2,
    WaitFrames = 3,
    Move = 4,
    Jump = 5,
    Slam = 6,
    Charge = 7,
    Scale = 8,
    Rotate = 9,
    RotateAround = 10,
    SetDirection = 11,
    Activate = 12,
    Deactivate = 13,
    PlaySound = 14,
    PlayMusic = 15,
    SetCinematic = 16,
    SetInputEnabled = 17,
    PanCameraToObject = 18,
    CameraFollowPlayer = 19,
    ShowGameText = 20,
    SetVulnerable = 21,
    Color = 22,
    Damage = 23,
    Kill = 24,
    Finish = 25,
    SetGravity = 26,
    SetVelocity = 27
);

impl Read for OldActionType {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let value = i32::read(input)?;

        Self::try_from(value).map_err(|()| Error::InvalidDynamicType(value))
    }
}

impl Write for OldActionType {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        i32::from(self).write(output)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct OldActionProperty {
    pub name: String,
    pub value: String,
}

impl Read for OldActionProperty {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self {
            name: Read::read(input)?,
            value: Read::read(input)?,
        })
    }
}

impl Write for OldActionProperty {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.name.write(output)?;
        self.value.write(output)
    }
}

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

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct Action {
    pub closed: bool,
    pub wait: bool,
    pub action_type: ActionType,
}

impl Read for Action {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let action_type = Read::read(input)?;

        Ok(Self {
            closed: Read::read(input)?,
            wait: Read::read(input)?,
            action_type: ReadContext::read_ctx(input, action_type)?,
        })
    }
}

impl Write for Action {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        let action_type = i32::from(&self.action_type);

        action_type.write(output)?;
        self.closed.write(output)?;
        self.wait.write(output)?;
        self.action_type.write(output)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct NovaValue {
    pub dynamic_type: DynamicType,

    pub bool_value: bool,
    pub int_value: i32,
    pub float_value: f32,
    pub string_value: Option<String>,
    pub color_value: Colour,
    pub vector_value: Vec2,
    pub int_list_value: Option<Vec<i32>>,
    pub sub_values: Option<Vec<NovaValue>>,
}

impl Read for NovaValue {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            dynamic_type: Read::read(input)?,
            bool_value: Read::read(input)?,
            int_value: Read::read(input)?,
            float_value: Read::read(input)?,
            string_value: Read::read(input)?,
            color_value: Read::read(input)?,
            vector_value: Read::read(input)?,
            int_list_value: Read::read(input)?,
            sub_values: Read::read(input)?,
        })
    }
}

impl Write for NovaValue {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.dynamic_type.write(output)?;
        self.bool_value.write(output)?;
        self.int_value.write(output)?;
        self.float_value.write(output)?;
        self.string_value.write(output)?;
        self.color_value.write(output)?;
        self.vector_value.write(output)?;
        self.int_list_value.write(output)?;
        self.sub_values.write(output)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct FunctionCall {
    pub id: i32,
    pub parameters: Vec<CallParameter>,
}

impl Read for FunctionCall {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            id: Read::read(input)?,
            parameters: Read::read(input)?,
        })
    }
}

impl Write for FunctionCall {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.id.write(output)?;
        self.parameters.write(output)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct CallParameter {
    pub parameter_id: i32,
    pub value: NovaValue,
}

impl Read for CallParameter {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            parameter_id: Read::read(input)?,
            value: Read::read(input)?,
        })
    }
}

impl Write for CallParameter {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.parameter_id.write(output)?;
        self.value.write(output)
    }
}

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

macro_rules! define_static_type {
    ($($name:ident = $number:expr),*) => {
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
        pub enum StaticType {
            $($name = $number),*
        }

        impl TryFrom<i32> for StaticType {
            type Error = ();

            fn try_from(value: i32) -> Result<Self, Self::Error> {
                match value {
                    $($number => Ok(StaticType::$name),)*
                    _ => Err(())
                }
            }
        }

        impl From<&StaticType> for i32 {
            fn from(value: &StaticType) -> Self {
                match value {
                    $(StaticType::$name => $number,)*
                }
            }
        }
    };
}

define_static_type!(
    Bool = 0,
    Int = 1,
    Float = 2,
    String = 3,
    Colour = 4,
    Vector = 5,
    Sound = 6,
    Music = 7,
    Object = 8,
    ObjectSet = 9,
    Transition = 10,
    Easing = 11,
    Sprite = 12,
    Script = 13,
    Layer = 14
);

impl Read for StaticType {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let value = i32::read(input)?;

        Self::try_from(value).map_err(|()| Error::InvalidStaticType(value))
    }
}

impl Write for StaticType {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        i32::from(self).write(output)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
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

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
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
