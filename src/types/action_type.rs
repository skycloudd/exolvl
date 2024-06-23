use super::{
    function_call::FunctionCall,
    novascript::{action::Action, nova_value::NovaValue},
};
use crate::{error::Error, Read, ReadContext, Write};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
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
        color: NovaValue,
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
        color: NovaValue,
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
        color: NovaValue,
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
        color: NovaValue,
        duration: NovaValue,
        easing: NovaValue,
    },
    TransitionOut {
        type_: NovaValue,
        color: NovaValue,
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

    #[allow(clippy::too_many_lines)]
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
                color: Read::read(input)?,
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
                color: Read::read(input)?,
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
                color: Read::read(input)?,
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
                color: Read::read(input)?,
                duration: Read::read(input)?,
                easing: Read::read(input)?,
            },
            45 => Self::TransitionOut {
                type_: Read::read(input)?,
                color: Read::read(input)?,
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
    #[allow(clippy::too_many_lines)]
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
                color,
                channel,
                duration,
                easing,
            } => {
                target_objects.write(output)?;
                color.write(output)?;
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
                color,
                duration,
                easing,
            } => {
                target_objects.write(output)?;
                color.write(output)?;
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
                color,
                duration,
                easing,
            } => {
                target_objects.write(output)?;
                color.write(output)?;
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
                color,
                duration,
                easing,
            } => {
                type_.write(output)?;
                color.write(output)?;
                duration.write(output)?;
                easing.write(output)
            }
            Self::TransitionOut {
                type_,
                color,
                duration,
                easing,
            } => {
                type_.write(output)?;
                color.write(output)?;
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
