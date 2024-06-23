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
use types::{color::Colour, vec2::Vec2};
use uuid::Uuid;

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
pub enum ActionType {
    Repeat {
        actions: Vec<Action>,
        count: NovaValue,
    },
    RepeatWhile {
        actions: Vec<Action>,
        condition: NovaValue,
    },
    ConditionBlock {
        if_actions: Vec<Action>,
        else_actions: Vec<Action>,
        condition: NovaValue,
    },
    Wait {
        duration: NovaValue,
    },
    WaitFrames {
        frames: NovaValue,
    },
    Move {
        target_objects: NovaValue,
        position: NovaValue,
        global: NovaValue,
        duration: NovaValue,
        easing: NovaValue,
    },
    Scale {
        target_objects: NovaValue,
        scale: NovaValue,
        duration: NovaValue,
        easing: NovaValue,
    },
    Rotate {
        target_objects: NovaValue,
        rotation: NovaValue,
        shortest_path: NovaValue,
        global: NovaValue,
        duration: NovaValue,
        easing: NovaValue,
    },
    RotateAround {
        target_objects: NovaValue,
        pivot: NovaValue,
        rotation: NovaValue,
        rotate_target: NovaValue,
        duration: NovaValue,
        easing: NovaValue,
    },
    SetVariable {
        variable: i32,
        value: Option<NovaValue>,
    },
    ResetVariable {
        variable: i32,
    },
    ResetObject {
        target_objects: NovaValue,
    },
    SetColor {
        target_objects: NovaValue,
        colour: NovaValue,
        channel: NovaValue,
        duration: NovaValue,
        easing: NovaValue,
    },
    SetTransparency {
        target_objects: NovaValue,
        transparency: NovaValue,
        channel: NovaValue,
        duration: NovaValue,
        easing: NovaValue,
    },
    SetSecondaryColor {
        target_objects: NovaValue,
        colour: NovaValue,
        duration: NovaValue,
        easing: NovaValue,
    },
    SetSecondaryTransparency {
        target_objects: NovaValue,
        transparency: NovaValue,
        duration: NovaValue,
        easing: NovaValue,
    },
    SetBorderColor {
        target_objects: NovaValue,
        colour: NovaValue,
        duration: NovaValue,
        easing: NovaValue,
    },
    SetBorderTransparency {
        target_objects: NovaValue,
        transparency: NovaValue,
        duration: NovaValue,
        easing: NovaValue,
    },
    SetSprite {
        target_objects: NovaValue,
        sprite: NovaValue,
    },
    SetText {
        target_objects: NovaValue,
        text: NovaValue,
    },
    SetEnabled {
        target_objects: NovaValue,
        enabled: NovaValue,
    },
    Activate {
        target_objects: NovaValue,
    },
    Deactivate {
        target_objects: NovaValue,
    },
    Damage {
        target_objects: NovaValue,
        damage: NovaValue,
    },
    Kill {
        target_objects: NovaValue,
    },
    GameFinish,
    CameraPan {
        position: NovaValue,
        duration: NovaValue,
        easing: NovaValue,
    },
    CameraFollowPlayer,
    CameraZoom {
        viewport_size: NovaValue,
        duration: NovaValue,
        easing: NovaValue,
    },
    CameraZoomReset {
        duration: NovaValue,
        easing: NovaValue,
    },
    CameraOffset {
        offset: NovaValue,
        duration: NovaValue,
        easing: NovaValue,
    },
    CameraOffsetReset {
        duration: NovaValue,
        easing: NovaValue,
    },
    CameraShake {
        strength: NovaValue,
        roughness: NovaValue,
        fade_in: NovaValue,
        fade_out: NovaValue,
        duration: NovaValue,
    },
    PlaySound {
        sound: NovaValue,
        volume: NovaValue,
        pitch: NovaValue,
    },
    PlayMusic {
        music: NovaValue,
        volume: NovaValue,
        pitch: NovaValue,
    },
    SetDirection {
        target_objects: NovaValue,
        direction: NovaValue,
    },
    SetGravity {
        target_objects: NovaValue,
        gravity: NovaValue,
    },
    SetVelocity {
        target_objects: NovaValue,
        velocity: NovaValue,
    },
    SetCinematic {
        enabled: NovaValue,
    },
    SetInputEnabled {
        enabled: NovaValue,
    },
    SetTimerEnabled {
        enabled: NovaValue,
    },
    GameTextShow {
        text: NovaValue,
        duration: NovaValue,
    },
    DialogueShow {
        text: NovaValue,
        position: NovaValue,
        reverse_direction: NovaValue,
    },
    StopScript {
        script: NovaValue,
    },
    TransitionIn {
        type_: NovaValue,
        colour: NovaValue,
        duration: NovaValue,
        easing: NovaValue,
    },
    TransitionOut {
        type_: NovaValue,
        colour: NovaValue,
        duration: NovaValue,
        easing: NovaValue,
    },
    TimeScale {
        time_scale: NovaValue,
        duration: NovaValue,
        easing: NovaValue,
    },
    RunFunction {
        function: FunctionCall,
    },
    SetVariableOverTime {
        variable: i32,
        value: Option<NovaValue>,
        duration: NovaValue,
        easing: NovaValue,
    },
    RepeatForEachObject {
        target_objects: NovaValue,
        actions: Vec<Action>,
    },
    StopSound {
        sound_instance: NovaValue,
        fade_out: NovaValue,
    },
    PlayParticleSystem {
        target_objects: NovaValue,
    },
    StopParticleSystem {
        target_objects: NovaValue,
        clear: NovaValue,
    },
}

impl From<&ActionType> for i32 {
    fn from(action_type: &ActionType) -> Self {
        match action_type {
            ActionType::Repeat { .. } => 0,
            ActionType::RepeatWhile { .. } => 1,
            ActionType::ConditionBlock { .. } => 2,
            ActionType::Wait { .. } => 3,
            ActionType::WaitFrames { .. } => 4,
            ActionType::Move { .. } => 5,
            ActionType::Scale { .. } => 6,
            ActionType::Rotate { .. } => 7,
            ActionType::RotateAround { .. } => 8,
            ActionType::SetVariable { .. } => 9,
            ActionType::ResetVariable { .. } => 10,
            ActionType::ResetObject { .. } => 11,
            ActionType::SetColor { .. } => 12,
            ActionType::SetTransparency { .. } => 13,
            ActionType::SetSecondaryColor { .. } => 14,
            ActionType::SetSecondaryTransparency { .. } => 15,
            ActionType::SetBorderColor { .. } => 16,
            ActionType::SetBorderTransparency { .. } => 17,
            ActionType::SetSprite { .. } => 18,
            ActionType::SetText { .. } => 19,
            ActionType::SetEnabled { .. } => 20,
            ActionType::Activate { .. } => 21,
            ActionType::Deactivate { .. } => 22,
            ActionType::Damage { .. } => 23,
            ActionType::Kill { .. } => 24,
            ActionType::GameFinish => 25,
            ActionType::CameraPan { .. } => 26,
            ActionType::CameraFollowPlayer => 27,
            ActionType::CameraZoom { .. } => 28,
            ActionType::CameraZoomReset { .. } => 29,
            ActionType::CameraOffset { .. } => 30,
            ActionType::CameraOffsetReset { .. } => 31,
            ActionType::CameraShake { .. } => 32,
            ActionType::PlaySound { .. } => 33,
            ActionType::PlayMusic { .. } => 34,
            ActionType::SetDirection { .. } => 35,
            ActionType::SetGravity { .. } => 36,
            ActionType::SetVelocity { .. } => 37,
            ActionType::SetCinematic { .. } => 38,
            ActionType::SetInputEnabled { .. } => 39,
            ActionType::SetTimerEnabled { .. } => 40,
            ActionType::GameTextShow { .. } => 41,
            ActionType::DialogueShow { .. } => 42,
            ActionType::StopScript { .. } => 43,
            ActionType::TransitionIn { .. } => 44,
            ActionType::TransitionOut { .. } => 45,
            ActionType::TimeScale { .. } => 46,
            ActionType::RunFunction { .. } => 47,
            ActionType::SetVariableOverTime { .. } => 48,
            ActionType::RepeatForEachObject { .. } => 49,
            ActionType::StopSound { .. } => 50,
            ActionType::PlayParticleSystem { .. } => 51,
            ActionType::StopParticleSystem { .. } => 52,
        }
    }
}

impl ReadContext for ActionType {
    type Context = i32;

    fn read_ctx(input: &mut impl std::io::Read, with: Self::Context) -> Result<Self, Error> {
        Ok(match with {
            0 => Self::Repeat {
                actions: Read::read(input)?,
                count: Read::read(input)?,
            },
            1 => Self::RepeatWhile {
                actions: Read::read(input)?,
                condition: Read::read(input)?,
            },
            2 => Self::ConditionBlock {
                if_actions: Read::read(input)?,
                else_actions: Read::read(input)?,
                condition: Read::read(input)?,
            },
            3 => Self::Wait {
                duration: Read::read(input)?,
            },
            4 => Self::WaitFrames {
                frames: Read::read(input)?,
            },
            5 => Self::Move {
                target_objects: Read::read(input)?,
                position: Read::read(input)?,
                global: Read::read(input)?,
                duration: Read::read(input)?,
                easing: Read::read(input)?,
            },
            6 => Self::Scale {
                target_objects: Read::read(input)?,
                scale: Read::read(input)?,
                duration: Read::read(input)?,
                easing: Read::read(input)?,
            },
            7 => Self::Rotate {
                target_objects: Read::read(input)?,
                rotation: Read::read(input)?,
                shortest_path: Read::read(input)?,
                global: Read::read(input)?,
                duration: Read::read(input)?,
                easing: Read::read(input)?,
            },
            8 => Self::RotateAround {
                target_objects: Read::read(input)?,
                pivot: Read::read(input)?,
                rotation: Read::read(input)?,
                rotate_target: Read::read(input)?,
                duration: Read::read(input)?,
                easing: Read::read(input)?,
            },
            9 => Self::SetVariable {
                variable: Read::read(input)?,
                value: Read::read(input)?,
            },
            10 => Self::ResetVariable {
                variable: Read::read(input)?,
            },
            11 => Self::ResetObject {
                target_objects: Read::read(input)?,
            },
            12 => Self::SetColor {
                target_objects: Read::read(input)?,
                colour: Read::read(input)?,
                channel: Read::read(input)?,
                duration: Read::read(input)?,
                easing: Read::read(input)?,
            },
            13 => Self::SetTransparency {
                target_objects: Read::read(input)?,
                transparency: Read::read(input)?,
                channel: Read::read(input)?,
                duration: Read::read(input)?,
                easing: Read::read(input)?,
            },
            14 => Self::SetSecondaryColor {
                target_objects: Read::read(input)?,
                colour: Read::read(input)?,
                duration: Read::read(input)?,
                easing: Read::read(input)?,
            },
            15 => Self::SetSecondaryTransparency {
                target_objects: Read::read(input)?,
                transparency: Read::read(input)?,
                duration: Read::read(input)?,
                easing: Read::read(input)?,
            },
            16 => Self::SetBorderColor {
                target_objects: Read::read(input)?,
                colour: Read::read(input)?,
                duration: Read::read(input)?,
                easing: Read::read(input)?,
            },
            17 => Self::SetBorderTransparency {
                target_objects: Read::read(input)?,
                transparency: Read::read(input)?,
                duration: Read::read(input)?,
                easing: Read::read(input)?,
            },
            18 => Self::SetSprite {
                target_objects: Read::read(input)?,
                sprite: Read::read(input)?,
            },
            19 => Self::SetText {
                target_objects: Read::read(input)?,
                text: Read::read(input)?,
            },
            20 => Self::SetEnabled {
                target_objects: Read::read(input)?,
                enabled: Read::read(input)?,
            },
            21 => Self::Activate {
                target_objects: Read::read(input)?,
            },
            22 => Self::Deactivate {
                target_objects: Read::read(input)?,
            },
            23 => Self::Damage {
                target_objects: Read::read(input)?,
                damage: Read::read(input)?,
            },
            24 => Self::Kill {
                target_objects: Read::read(input)?,
            },
            25 => Self::GameFinish,
            26 => Self::CameraPan {
                position: Read::read(input)?,
                duration: Read::read(input)?,
                easing: Read::read(input)?,
            },
            27 => Self::CameraFollowPlayer,
            28 => Self::CameraZoom {
                viewport_size: Read::read(input)?,
                duration: Read::read(input)?,
                easing: Read::read(input)?,
            },
            29 => Self::CameraZoomReset {
                duration: Read::read(input)?,
                easing: Read::read(input)?,
            },
            30 => Self::CameraOffset {
                offset: Read::read(input)?,
                duration: Read::read(input)?,
                easing: Read::read(input)?,
            },
            31 => Self::CameraOffsetReset {
                duration: Read::read(input)?,
                easing: Read::read(input)?,
            },
            32 => Self::CameraShake {
                strength: Read::read(input)?,
                roughness: Read::read(input)?,
                fade_in: Read::read(input)?,
                fade_out: Read::read(input)?,
                duration: Read::read(input)?,
            },
            33 => Self::PlaySound {
                sound: Read::read(input)?,
                volume: Read::read(input)?,
                pitch: Read::read(input)?,
            },
            34 => Self::PlayMusic {
                music: Read::read(input)?,
                volume: Read::read(input)?,
                pitch: Read::read(input)?,
            },
            35 => Self::SetDirection {
                target_objects: Read::read(input)?,
                direction: Read::read(input)?,
            },
            36 => Self::SetGravity {
                target_objects: Read::read(input)?,
                gravity: Read::read(input)?,
            },
            37 => Self::SetVelocity {
                target_objects: Read::read(input)?,
                velocity: Read::read(input)?,
            },
            38 => Self::SetCinematic {
                enabled: Read::read(input)?,
            },
            39 => Self::SetInputEnabled {
                enabled: Read::read(input)?,
            },
            40 => Self::SetTimerEnabled {
                enabled: Read::read(input)?,
            },
            41 => Self::GameTextShow {
                text: Read::read(input)?,
                duration: Read::read(input)?,
            },
            42 => Self::DialogueShow {
                text: Read::read(input)?,
                position: Read::read(input)?,
                reverse_direction: Read::read(input)?,
            },
            43 => Self::StopScript {
                script: Read::read(input)?,
            },
            44 => Self::TransitionIn {
                type_: Read::read(input)?,
                colour: Read::read(input)?,
                duration: Read::read(input)?,
                easing: Read::read(input)?,
            },
            45 => Self::TransitionOut {
                type_: Read::read(input)?,
                colour: Read::read(input)?,
                duration: Read::read(input)?,
                easing: Read::read(input)?,
            },
            46 => Self::TimeScale {
                time_scale: Read::read(input)?,
                duration: Read::read(input)?,
                easing: Read::read(input)?,
            },
            47 => Self::RunFunction {
                function: Read::read(input)?,
            },
            48 => Self::SetVariableOverTime {
                variable: Read::read(input)?,
                value: Read::read(input)?,
                duration: Read::read(input)?,
                easing: Read::read(input)?,
            },
            49 => Self::RepeatForEachObject {
                target_objects: Read::read(input)?,
                actions: Read::read(input)?,
            },
            50 => Self::StopSound {
                sound_instance: Read::read(input)?,
                fade_out: Read::read(input)?,
            },
            51 => Self::PlayParticleSystem {
                target_objects: Read::read(input)?,
            },
            52 => Self::StopParticleSystem {
                target_objects: Read::read(input)?,
                clear: Read::read(input)?,
            },

            n => return Err(Error::InvalidActionType(n)),
        })
    }
}

impl Write for ActionType {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        match self {
            Self::Repeat { actions, count } => {
                actions.write(output)?;
                count.write(output)
            }
            Self::RepeatWhile { actions, condition } => {
                actions.write(output)?;
                condition.write(output)
            }
            Self::ConditionBlock {
                if_actions,
                else_actions,
                condition,
            } => {
                if_actions.write(output)?;
                else_actions.write(output)?;
                condition.write(output)
            }
            Self::Wait { duration } => duration.write(output),
            Self::WaitFrames { frames } => frames.write(output),
            Self::Move {
                target_objects,
                position,
                global,
                duration,
                easing,
            } => {
                target_objects.write(output)?;
                position.write(output)?;
                global.write(output)?;
                duration.write(output)?;
                easing.write(output)
            }
            Self::Scale {
                target_objects,
                scale,
                duration,
                easing,
            } => {
                target_objects.write(output)?;
                scale.write(output)?;
                duration.write(output)?;
                easing.write(output)
            }
            Self::Rotate {
                target_objects,
                rotation,
                shortest_path,
                global,
                duration,
                easing,
            } => {
                target_objects.write(output)?;
                rotation.write(output)?;
                shortest_path.write(output)?;
                global.write(output)?;
                duration.write(output)?;
                easing.write(output)
            }
            Self::RotateAround {
                target_objects,
                pivot,
                rotation,
                rotate_target,
                duration,
                easing,
            } => {
                target_objects.write(output)?;
                pivot.write(output)?;
                rotation.write(output)?;
                rotate_target.write(output)?;
                duration.write(output)?;
                easing.write(output)
            }
            Self::SetVariable { variable, value } => {
                variable.write(output)?;
                value.write(output)
            }
            Self::ResetVariable { variable } => variable.write(output),
            Self::ResetObject { target_objects }
            | Self::Activate { target_objects }
            | Self::Deactivate { target_objects }
            | Self::Kill { target_objects }
            | Self::PlayParticleSystem { target_objects } => target_objects.write(output),
            Self::SetColor {
                target_objects,
                colour,
                channel,
                duration,
                easing,
            } => {
                target_objects.write(output)?;
                colour.write(output)?;
                channel.write(output)?;
                duration.write(output)?;
                easing.write(output)
            }
            Self::SetTransparency {
                target_objects,
                transparency,
                channel,
                duration,
                easing,
            } => {
                target_objects.write(output)?;
                transparency.write(output)?;
                channel.write(output)?;
                duration.write(output)?;
                easing.write(output)
            }
            Self::SetSecondaryColor {
                target_objects,
                colour,
                duration,
                easing,
            } => {
                target_objects.write(output)?;
                colour.write(output)?;
                duration.write(output)?;
                easing.write(output)
            }
            Self::SetSecondaryTransparency {
                target_objects,
                transparency,
                duration,
                easing,
            } => {
                target_objects.write(output)?;
                transparency.write(output)?;
                duration.write(output)?;
                easing.write(output)
            }
            Self::SetBorderColor {
                target_objects,
                colour,
                duration,
                easing,
            } => {
                target_objects.write(output)?;
                colour.write(output)?;
                duration.write(output)?;
                easing.write(output)
            }
            Self::SetBorderTransparency {
                target_objects,
                transparency,
                duration,
                easing,
            } => {
                target_objects.write(output)?;
                transparency.write(output)?;
                duration.write(output)?;
                easing.write(output)
            }
            Self::SetSprite {
                target_objects,
                sprite,
            } => {
                target_objects.write(output)?;
                sprite.write(output)
            }
            Self::SetText {
                target_objects,
                text,
            } => {
                target_objects.write(output)?;
                text.write(output)
            }
            Self::SetEnabled {
                target_objects,
                enabled,
            } => {
                target_objects.write(output)?;
                enabled.write(output)
            }
            Self::Damage {
                target_objects,
                damage,
            } => {
                target_objects.write(output)?;
                damage.write(output)
            }
            Self::CameraPan {
                position,
                duration,
                easing,
            } => {
                position.write(output)?;
                duration.write(output)?;
                easing.write(output)
            }
            Self::GameFinish | Self::CameraFollowPlayer => Ok(()),
            Self::CameraZoom {
                viewport_size,
                duration,
                easing,
            } => {
                viewport_size.write(output)?;
                duration.write(output)?;
                easing.write(output)
            }
            Self::CameraZoomReset { duration, easing } => {
                duration.write(output)?;
                easing.write(output)
            }
            Self::CameraOffset {
                offset,
                duration,
                easing,
            } => {
                offset.write(output)?;
                duration.write(output)?;
                easing.write(output)
            }
            Self::CameraOffsetReset { duration, easing } => {
                duration.write(output)?;
                easing.write(output)
            }
            Self::CameraShake {
                strength,
                roughness,
                fade_in,
                fade_out,
                duration,
            } => {
                strength.write(output)?;
                roughness.write(output)?;
                fade_in.write(output)?;
                fade_out.write(output)?;
                duration.write(output)
            }
            Self::PlaySound {
                sound,
                volume,
                pitch,
            } => {
                sound.write(output)?;
                volume.write(output)?;
                pitch.write(output)
            }
            Self::PlayMusic {
                music,
                volume,
                pitch,
            } => {
                music.write(output)?;
                volume.write(output)?;
                pitch.write(output)
            }
            Self::SetDirection {
                target_objects,
                direction,
            } => {
                target_objects.write(output)?;
                direction.write(output)
            }
            Self::SetGravity {
                target_objects,
                gravity,
            } => {
                target_objects.write(output)?;
                gravity.write(output)
            }
            Self::SetVelocity {
                target_objects,
                velocity,
            } => {
                target_objects.write(output)?;
                velocity.write(output)
            }
            Self::SetCinematic { enabled }
            | Self::SetInputEnabled { enabled }
            | Self::SetTimerEnabled { enabled } => enabled.write(output),
            Self::GameTextShow { text, duration } => {
                text.write(output)?;
                duration.write(output)
            }
            Self::DialogueShow {
                text,
                position,
                reverse_direction,
            } => {
                text.write(output)?;
                position.write(output)?;
                reverse_direction.write(output)
            }
            Self::StopScript { script } => script.write(output),
            Self::TransitionIn {
                type_,
                colour,
                duration,
                easing,
            } => {
                type_.write(output)?;
                colour.write(output)?;
                duration.write(output)?;
                easing.write(output)
            }
            Self::TransitionOut {
                type_,
                colour,
                duration,
                easing,
            } => {
                type_.write(output)?;
                colour.write(output)?;
                duration.write(output)?;
                easing.write(output)
            }
            Self::TimeScale {
                time_scale,
                duration,
                easing,
            } => {
                time_scale.write(output)?;
                duration.write(output)?;
                easing.write(output)
            }
            Self::RunFunction { function } => function.write(output),
            Self::SetVariableOverTime {
                variable,
                value,
                duration,
                easing,
            } => {
                variable.write(output)?;
                value.write(output)?;
                duration.write(output)?;
                easing.write(output)
            }
            Self::RepeatForEachObject {
                target_objects,
                actions,
            } => {
                target_objects.write(output)?;
                actions.write(output)
            }
            Self::StopSound {
                sound_instance,
                fade_out,
            } => {
                sound_instance.write(output)?;
                fade_out.write(output)
            }
            Self::StopParticleSystem {
                target_objects,
                clear,
            } => {
                target_objects.write(output)?;
                clear.write(output)
            }
        }
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

macro_rules! define_dynamic_type {
    ($($name:ident = $number:expr),*) => {
        #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
        #[derive(Clone, Copy, Debug,   Hash, PartialEq, Eq, PartialOrd, Ord)]
        pub enum DynamicType {
            $($name = $number),*
        }

        impl TryFrom<i32> for DynamicType {
            type Error = ();

            fn try_from(value: i32) -> Result<Self, Self::Error> {
                match value {
                    $($number => Ok(DynamicType::$name),)*
                    _ => Err(())
                }
            }
        }

        impl From<&DynamicType> for i32 {
            fn from(value: &DynamicType) -> Self {
                match value {
                    $(DynamicType::$name => $number,)*
                }
            }
        }
    };
}

define_dynamic_type!(
    BoolConstant = 0,
    BoolVariable = 1,
    BoolNot = 2,
    BoolAnd = 3,
    BoolOr = 4,
    BoolEqualBool = 5,
    BoolEqualNumber = 6,
    BoolEqualString = 7,
    BoolEqualColor = 8,
    BoolEqualVector = 9,
    BoolEqualObject = 10,
    BoolNotEqualBool = 11,
    BoolNotEqualNumber = 12,
    BoolNotEqualString = 13,
    BoolNotEqualColor = 14,
    BoolNotEqualVector = 15,
    BoolNotEqualObject = 16,
    BoolLess = 17,
    BoolLessOrEqual = 18,
    BoolGreater = 19,
    BoolGreaterOrEqual = 20,
    BoolObjectDead = 21,
    BoolPlayerOnGround = 22,
    BoolPlayerOnWalljump = 23,
    BoolPlayerOnBooster = 24,
    BoolPlayerOnSwing = 25,
    BoolPlayerInFloatingZone = 26,
    BoolPlayerUsingGlider = 27,
    BoolObjectsColliding = 28,
    BoolInputPressed = 29,
    BoolInputPressedLeft = 30,
    BoolInputPressedRight = 31,
    BoolInputHeld = 32,
    BoolInputHeldLeft = 33,
    BoolInputHeldRight = 34,
    BoolInputReleased = 35,
    BoolInputReleasedLeft = 36,
    BoolInputReleasedRight = 37,
    IntConstant = 38,
    IntVariable = 39,
    IntAdd = 40,
    IntSubtract = 41,
    IntMultiply = 42,
    IntDivide = 43,
    IntModulo = 44,
    IntMin = 45,
    IntMax = 46,
    IntAbs = 47,
    IntSign = 48,
    IntRound = 49,
    IntCeil = 50,
    IntFloor = 51,
    IntRandom = 52,
    IntRepeatCount = 53,
    IntObjectDirection = 54,
    IntObjectSetCount = 55,
    FloatConstant = 56,
    FloatVariable = 57,
    FloatAdd = 58,
    FloatSubtract = 59,
    FloatMultiply = 60,
    FloatDivide = 61,
    FloatModulo = 62,
    FloatMin = 63,
    FloatMax = 64,
    FloatAbs = 65,
    FloatSign = 66,
    FloatRound = 67,
    FloatCeil = 68,
    FloatFloor = 69,
    FloatCos = 70,
    FloatSin = 71,
    FloatTan = 72,
    FloatAcos = 73,
    FloatAsin = 74,
    FloatAtan = 75,
    FloatSqrt = 76,
    FloatPow = 77,
    FloatRandom = 78,
    FloatTime = 79,
    FloatSemitones = 80,
    FloatVectorX = 81,
    FloatVectorY = 82,
    FloatVectorLength = 83,
    FloatVectorLengthSqr = 84,
    FloatVectorDistance = 85,
    FloatVectorDistanceSqr = 86,
    FloatVectorDot = 87,
    FloatVectorAngle = 88,
    FloatVectorAngleBetween = 89,
    FloatObjectRotation = 90,
    FloatObjectGlobalRotation = 91,
    FloatCameraViewportSize = 92,
    FloatDamageAmount = 93,
    StringConstant = 94,
    StringVariable = 95,
    StringFromInt = 96,
    StringFromFloat = 97,
    StringConcat = 98,
    ColorConstant = 99,
    ColorValues = 100,
    ColorVariable = 101,
    ColorObjectColor = 102,
    VectorConstant = 103,
    VectorValues = 104,
    VectorVariable = 105,
    VectorAdd = 106,
    VectorSubtract = 107,
    VectorMultiply = 108,
    VectorDivide = 109,
    VectorNormalize = 110,
    VectorPerpendicular = 111,
    VectorReflect = 112,
    VectorObjectPos = 113,
    VectorObjectGlobalPos = 114,
    VectorObjectScale = 115,
    VectorObjectGlobalScale = 116,
    VectorObjectVelocity = 117,
    VectorCameraPos = 118,
    SoundConstant = 119,
    SoundVariable = 120,
    MusicConstant = 121,
    MusicVariable = 122,
    ObjectConstant = 123,
    ObjectVariable = 124,
    ObjectAnyObject = 125,
    ObjectFirstFromSet = 126,
    ObjectRandomFromSet = 127,
    ObjectElementFromSet = 128,
    ObjectSourceObject = 129,
    ObjectCollidedObject = 130,
    ObjectTargetObject = 131,
    ObjectPlayer = 132,
    ObjectParent = 133,
    ObjectSetConstant = 134,
    ObjectSetVariable = 135,
    ObjectSetConcat = 136,
    ObjectSetPlayers = 137,
    ObjectSetObjectsWithTag = 138,
    TransitionConstant = 139,
    TransitionVariable = 140,
    EasingConstant = 141,
    EasingVariable = 142,
    ObjectSetChildren = 143,
    BoolObjectActivated = 144,
    FloatLevelTime = 145,
    BoolPlayerJumpLocked = 146,
    StringObjectTag = 147,
    SpriteConstant = 148,
    SpriteVariable = 149,
    ScriptConstant = 150,
    ScriptVariable = 151,
    BoolParameter = 152,
    IntParameter = 153,
    FloatParameter = 154,
    StringParameter = 155,
    ColorParameter = 156,
    VectorParameter = 157,
    SoundParameter = 158,
    MusicParameter = 159,
    ObjectParameter = 160,
    ObjectSetParameter = 161,
    TransitionParameter = 162,
    EasingParameter = 163,
    SpriteParameter = 164,
    ScriptParameter = 165,
    BoolObjectsCollidingWithPoint = 166,
    FloatRoundDecimals = 167,
    VectorPointerPositionDeprecated = 168,
    VectorPointerWorldPositionDeprecated = 169,
    VectorCollisionPoint = 170,
    VectorCollisionNormal = 171,
    ObjectRepeatObject = 172,
    VectorClosestFromPoint = 173,
    ObjectSetAllObjects = 174,
    ObjectSetObjectsInLayer = 175,
    ObjectSetObjectsInCircle = 176,
    LayerConstant = 177,
    LayerVariable = 178,
    LayerParameter = 179,
    VectorRotate = 180,
    IntLastSoundInstance = 181,
    ObjectSetUnion = 182,
    ObjectSetIntersection = 183,
    ObjectSetDifference = 184,
    ObjectSetRemoveAtIndex = 185,
    VectorPointerPosition = 186,
    VectorPointerWorldPosition = 187,
    BoolPointerDown = 188,
    BoolPointerHeld = 189,
    BoolPointerReleased = 190,
    FloatColourR = 191,
    FloatColourG = 192,
    FloatColourB = 193,
    FloatColourA = 194,
    StringSubstring = 195,
    IntStringLength = 196
);

impl Read for DynamicType {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let value = i32::read(input)?;

        Self::try_from(value).map_err(|()| Error::InvalidDynamicType(value))
    }
}

impl Write for DynamicType {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        i32::from(self).write(output)
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

impl Read for Uuid {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error>
    where
        Self: Sized,
    {
        Ok(Self::parse_str(&String::read(input)?).unwrap())
    }
}

impl Write for Uuid {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.to_string().write(output)
    }
}
