use crate::{bool_to_u8, Colour, MyString, MyVec, Vec2};
use binread::{BinRead, BinReaderExt as _, BinResult, ReadOptions};
use binwrite::BinWrite;
use std::io::{Read, Seek};

#[derive(Debug, BinRead, BinWrite)]
pub struct NovaScript {
    pub script_id: i32,
    pub script_name: MyString,
    #[br(map = |x: u8| x != 0)]
    #[binwrite(preprocessor(|x: &bool| bool_to_u8(*x)))]
    pub is_function: bool,
    pub activation_count: i32,
    pub condition: NovaValue,
    pub activation_list: MyVec<Activator>,
    pub parameters: MyVec<Parameter>,
    pub variables: MyVec<Variable>,
    pub actions: MyVec<Action>,
}

#[derive(Debug)]
pub struct Action {
    pub closed: bool,
    pub wait: bool,
    pub action_type: ActionType,
}

impl BinRead for Action {
    type Args = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        _options: &ReadOptions,
        _args: Self::Args,
    ) -> BinResult<Self> {
        let action_type = reader.read_le::<i32>()?;
        let closed = reader.read_le::<u8>()? != 0;
        let wait = reader.read_le::<u8>()? != 0;

        Ok(Self {
            closed,
            wait,
            action_type: match action_type {
                0 => {
                    let actions = reader.read_le::<MyVec<Self>>()?;
                    let count = reader.read_le::<NovaValue>()?;

                    ActionType::Repeat { actions, count }
                }
                1 => {
                    let actions = reader.read_le::<MyVec<Self>>()?;
                    let condition = reader.read_le::<NovaValue>()?;

                    ActionType::RepeatWhile { actions, condition }
                }
                2 => {
                    let if_actions = reader.read_le::<MyVec<Self>>()?;
                    let else_actions = reader.read_le::<MyVec<Self>>()?;
                    let condition = reader.read_le::<NovaValue>()?;

                    ActionType::ConditionBlock {
                        if_actions,
                        else_actions,
                        condition,
                    }
                }
                3 => {
                    let duration = reader.read_le::<NovaValue>()?;

                    ActionType::Wait { duration }
                }
                4 => {
                    let frames = reader.read_le::<NovaValue>()?;

                    ActionType::WaitFrames { frames }
                }
                5 => {
                    let target_objects = reader.read_le::<NovaValue>()?;
                    let position = reader.read_le::<NovaValue>()?;
                    let global = reader.read_le::<NovaValue>()?;
                    let duration = reader.read_le::<NovaValue>()?;
                    let easing = reader.read_le::<NovaValue>()?;

                    ActionType::Move {
                        target_objects,
                        position,
                        global,
                        duration,
                        easing,
                    }
                }
                6 => {
                    let target_objects = reader.read_le::<NovaValue>()?;
                    let scale = reader.read_le::<NovaValue>()?;
                    let duration = reader.read_le::<NovaValue>()?;
                    let easing = reader.read_le::<NovaValue>()?;

                    ActionType::Scale {
                        target_objects,
                        scale,
                        duration,
                        easing,
                    }
                }
                7 => {
                    let target_objects = reader.read_le::<NovaValue>()?;
                    let rotation = reader.read_le::<NovaValue>()?;
                    let shortest_path = reader.read_le::<NovaValue>()?;
                    let global = reader.read_le::<NovaValue>()?;
                    let duration = reader.read_le::<NovaValue>()?;
                    let easing = reader.read_le::<NovaValue>()?;

                    ActionType::Rotate {
                        target_objects,
                        rotation,
                        shortest_path,
                        global,
                        duration,
                        easing,
                    }
                }
                8 => {
                    let target_objects = reader.read_le::<NovaValue>()?;
                    let pivot = reader.read_le::<NovaValue>()?;
                    let rotation = reader.read_le::<NovaValue>()?;
                    let rotate_target = reader.read_le::<NovaValue>()?;
                    let duration = reader.read_le::<NovaValue>()?;
                    let easing = reader.read_le::<NovaValue>()?;

                    ActionType::RotateAround {
                        target_objects,
                        pivot,
                        rotation,
                        rotate_target,
                        duration,
                        easing,
                    }
                }
                9 => {
                    let variable = reader.read_le::<i32>()?;

                    let value = if reader.read_le::<u8>()? != 0 {
                        Some(reader.read_le::<NovaValue>()?)
                    } else {
                        None
                    };

                    ActionType::SetVariable { variable, value }
                }
                10 => {
                    let variable = reader.read_le::<i32>()?;

                    ActionType::ResetVariable { variable }
                }
                11 => {
                    let target_objects = reader.read_le::<NovaValue>()?;

                    ActionType::ResetObject { target_objects }
                }
                12 => {
                    let target_objects = reader.read_le::<NovaValue>()?;
                    let color = reader.read_le::<NovaValue>()?;
                    let channel = reader.read_le::<NovaValue>()?;
                    let duration = reader.read_le::<NovaValue>()?;
                    let easing = reader.read_le::<NovaValue>()?;

                    ActionType::SetColor {
                        target_objects,
                        color,
                        channel,
                        duration,
                        easing,
                    }
                }
                13 => {
                    let target_objects = reader.read_le::<NovaValue>()?;
                    let transparency = reader.read_le::<NovaValue>()?;
                    let channel = reader.read_le::<NovaValue>()?;
                    let duration = reader.read_le::<NovaValue>()?;
                    let easing = reader.read_le::<NovaValue>()?;

                    ActionType::SetTransparency {
                        target_objects,
                        transparency,
                        channel,
                        duration,
                        easing,
                    }
                }
                14 => {
                    let target_objects = reader.read_le::<NovaValue>()?;
                    let color = reader.read_le::<NovaValue>()?;
                    let duration = reader.read_le::<NovaValue>()?;
                    let easing = reader.read_le::<NovaValue>()?;

                    ActionType::SetSecondaryColor {
                        target_objects,
                        color,
                        duration,
                        easing,
                    }
                }
                15 => {
                    let target_objects = reader.read_le::<NovaValue>()?;
                    let transparency = reader.read_le::<NovaValue>()?;
                    let duration = reader.read_le::<NovaValue>()?;
                    let easing = reader.read_le::<NovaValue>()?;

                    ActionType::SetSecondaryTransparency {
                        target_objects,
                        transparency,
                        duration,
                        easing,
                    }
                }
                16 => {
                    let target_objects = reader.read_le::<NovaValue>()?;
                    let color = reader.read_le::<NovaValue>()?;
                    let duration = reader.read_le::<NovaValue>()?;
                    let easing = reader.read_le::<NovaValue>()?;

                    ActionType::SetBorderColor {
                        target_objects,
                        color,
                        duration,
                        easing,
                    }
                }
                17 => {
                    let target_objects = reader.read_le::<NovaValue>()?;
                    let transparency = reader.read_le::<NovaValue>()?;
                    let duration = reader.read_le::<NovaValue>()?;
                    let easing = reader.read_le::<NovaValue>()?;

                    ActionType::SetBorderTransparency {
                        target_objects,
                        transparency,
                        duration,
                        easing,
                    }
                }
                18 => {
                    let target_objects = reader.read_le::<NovaValue>()?;
                    let sprite = reader.read_le::<NovaValue>()?;

                    ActionType::SetSprite {
                        target_objects,
                        sprite,
                    }
                }
                19 => {
                    let target_objects = reader.read_le::<NovaValue>()?;
                    let text = reader.read_le::<NovaValue>()?;

                    ActionType::SetText {
                        target_objects,
                        text,
                    }
                }
                20 => {
                    let target_objects = reader.read_le::<NovaValue>()?;
                    let enabled = reader.read_le::<NovaValue>()?;

                    ActionType::SetEnabled {
                        target_objects,
                        enabled,
                    }
                }
                21 => {
                    let target_objects = reader.read_le::<NovaValue>()?;

                    ActionType::Activate { target_objects }
                }
                22 => {
                    let target_objects = reader.read_le::<NovaValue>()?;

                    ActionType::Deactivate { target_objects }
                }
                23 => {
                    let target_objects = reader.read_le::<NovaValue>()?;
                    let damage = reader.read_le::<NovaValue>()?;

                    ActionType::Damage {
                        target_objects,
                        damage,
                    }
                }
                24 => {
                    let target_objects = reader.read_le::<NovaValue>()?;

                    ActionType::Kill { target_objects }
                }
                25 => ActionType::GameFinish {},
                26 => {
                    let position = reader.read_le::<NovaValue>()?;
                    let duration = reader.read_le::<NovaValue>()?;
                    let easing = reader.read_le::<NovaValue>()?;

                    ActionType::CameraPan {
                        position,
                        duration,
                        easing,
                    }
                }
                27 => ActionType::CameraFollowPlayer {},
                28 => {
                    let viewport_size = reader.read_le::<NovaValue>()?;
                    let duration = reader.read_le::<NovaValue>()?;
                    let easing = reader.read_le::<NovaValue>()?;

                    ActionType::CameraZoom {
                        viewport_size,
                        duration,
                        easing,
                    }
                }
                29 => {
                    let duration = reader.read_le::<NovaValue>()?;
                    let easing = reader.read_le::<NovaValue>()?;

                    ActionType::CameraZoomReset { duration, easing }
                }
                30 => {
                    let offset = reader.read_le::<NovaValue>()?;
                    let duration = reader.read_le::<NovaValue>()?;
                    let easing = reader.read_le::<NovaValue>()?;

                    ActionType::CameraOffset {
                        offset,
                        duration,
                        easing,
                    }
                }
                31 => {
                    let duration = reader.read_le::<NovaValue>()?;
                    let easing = reader.read_le::<NovaValue>()?;

                    ActionType::CameraOffsetReset { duration, easing }
                }
                32 => {
                    let strength = reader.read_le::<NovaValue>()?;
                    let roughness = reader.read_le::<NovaValue>()?;
                    let fade_in = reader.read_le::<NovaValue>()?;
                    let fade_out = reader.read_le::<NovaValue>()?;
                    let duration = reader.read_le::<NovaValue>()?;

                    ActionType::CameraShake {
                        strength,
                        roughness,
                        fade_in,
                        fade_out,
                        duration,
                    }
                }
                33 => {
                    let sound = reader.read_le::<NovaValue>()?;
                    let volume = reader.read_le::<NovaValue>()?;
                    let pitch = reader.read_le::<NovaValue>()?;

                    ActionType::PlaySound {
                        sound,
                        volume,
                        pitch,
                    }
                }
                34 => {
                    let music = reader.read_le::<NovaValue>()?;
                    let volume = reader.read_le::<NovaValue>()?;
                    let pitch = reader.read_le::<NovaValue>()?;

                    ActionType::PlayMusic {
                        music,
                        volume,
                        pitch,
                    }
                }
                35 => {
                    let target_objects = reader.read_le::<NovaValue>()?;
                    let direction = reader.read_le::<NovaValue>()?;

                    ActionType::SetDirection {
                        target_objects,
                        direction,
                    }
                }
                36 => {
                    let target_objects = reader.read_le::<NovaValue>()?;
                    let gravity = reader.read_le::<NovaValue>()?;

                    ActionType::SetGravity {
                        target_objects,
                        gravity,
                    }
                }
                37 => {
                    let target_objects = reader.read_le::<NovaValue>()?;
                    let velocity = reader.read_le::<NovaValue>()?;

                    ActionType::SetVelocity {
                        target_objects,
                        velocity,
                    }
                }
                38 => {
                    let enabled = reader.read_le::<NovaValue>()?;

                    ActionType::SetCinematic { enabled }
                }
                39 => {
                    let enabled = reader.read_le::<NovaValue>()?;

                    ActionType::SetInputEnabled { enabled }
                }
                40 => {
                    let enabled = reader.read_le::<NovaValue>()?;

                    ActionType::SetTimerEnabled { enabled }
                }
                41 => {
                    let text = reader.read_le::<NovaValue>()?;
                    let duration = reader.read_le::<NovaValue>()?;

                    ActionType::GameTextShow { text, duration }
                }
                42 => {
                    let text = reader.read_le::<NovaValue>()?;
                    let position = reader.read_le::<NovaValue>()?;
                    let reverse_direction = reader.read_le::<NovaValue>()?;

                    ActionType::DialogueShow {
                        text,
                        position,
                        reverse_direction,
                    }
                }
                43 => {
                    let script = reader.read_le::<NovaValue>()?;

                    ActionType::StopScript { script }
                }
                44 => {
                    let type_ = reader.read_le::<NovaValue>()?;
                    let color = reader.read_le::<NovaValue>()?;
                    let duration = reader.read_le::<NovaValue>()?;
                    let easing = reader.read_le::<NovaValue>()?;

                    ActionType::TransitionIn {
                        type_,
                        color,
                        duration,
                        easing,
                    }
                }
                45 => {
                    let type_ = reader.read_le::<NovaValue>()?;
                    let color = reader.read_le::<NovaValue>()?;
                    let duration = reader.read_le::<NovaValue>()?;
                    let easing = reader.read_le::<NovaValue>()?;

                    ActionType::TransitionOut {
                        type_,
                        color,
                        duration,
                        easing,
                    }
                }
                46 => {
                    let time_scale = reader.read_le::<NovaValue>()?;
                    let duration = reader.read_le::<NovaValue>()?;
                    let easing = reader.read_le::<NovaValue>()?;

                    ActionType::TimeScale {
                        time_scale,
                        duration,
                        easing,
                    }
                }
                47 => {
                    let function = reader.read_le::<FunctionCall>()?;

                    ActionType::RunFunction { function }
                }
                48 => {
                    let variable = reader.read_le::<i32>()?;

                    let value = if reader.read_le::<u8>()? == 0 {
                        Some(reader.read_le::<NovaValue>()?)
                    } else {
                        None
                    };

                    let duration = reader.read_le::<NovaValue>()?;
                    let easing = reader.read_le::<NovaValue>()?;

                    ActionType::SetVariableOverTime {
                        variable,
                        value,
                        duration,
                        easing,
                    }
                }
                49 => {
                    let target_objects = reader.read_le::<NovaValue>()?;
                    let actions = reader.read_le::<MyVec<Self>>()?;

                    ActionType::RepeatForEachObject {
                        target_objects,
                        actions,
                    }
                }
                _ => panic!("Unknown action type: {action_type}"),
            },
        })
    }
}

impl BinWrite for Action {
    fn write_options<W: std::io::prelude::Write>(
        &self,
        writer: &mut W,
        options: &binwrite::WriterOption,
    ) -> std::io::Result<()> {
        let type_: i32 = match &self.action_type {
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
            ActionType::GameFinish {} => 25,
            ActionType::CameraPan { .. } => 26,
            ActionType::CameraFollowPlayer {} => 27,
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
        };

        writer.write_all(&type_.to_le_bytes())?;

        writer.write_all(&bool_to_u8(self.closed).to_le_bytes())?;
        writer.write_all(&bool_to_u8(self.wait).to_le_bytes())?;

        match &self.action_type {
            ActionType::Repeat { actions, count } => {
                actions.write_options(writer, options)?;
                count.write_options(writer, options)?;
            }
            ActionType::RepeatWhile { actions, condition } => {
                actions.write_options(writer, options)?;
                condition.write_options(writer, options)?;
            }
            ActionType::ConditionBlock {
                if_actions,
                else_actions,
                condition,
            } => {
                if_actions.write_options(writer, options)?;
                else_actions.write_options(writer, options)?;
                condition.write_options(writer, options)?;
            }
            ActionType::Wait { duration } => {
                duration.write_options(writer, options)?;
            }
            ActionType::WaitFrames { frames } => {
                frames.write_options(writer, options)?;
            }
            ActionType::Move {
                target_objects,
                position,
                global,
                duration,
                easing,
            } => {
                target_objects.write_options(writer, options)?;
                position.write_options(writer, options)?;
                global.write_options(writer, options)?;
                duration.write_options(writer, options)?;
                easing.write_options(writer, options)?;
            }
            ActionType::Scale {
                target_objects,
                scale,
                duration,
                easing,
            } => {
                target_objects.write_options(writer, options)?;
                scale.write_options(writer, options)?;
                duration.write_options(writer, options)?;
                easing.write_options(writer, options)?;
            }
            ActionType::Rotate {
                target_objects,
                rotation,
                shortest_path,
                global,
                duration,
                easing,
            } => {
                target_objects.write_options(writer, options)?;
                rotation.write_options(writer, options)?;
                shortest_path.write_options(writer, options)?;
                global.write_options(writer, options)?;
                duration.write_options(writer, options)?;
                easing.write_options(writer, options)?;
            }
            ActionType::RotateAround {
                target_objects,
                pivot,
                rotation,
                rotate_target,
                duration,
                easing,
            } => {
                target_objects.write_options(writer, options)?;
                pivot.write_options(writer, options)?;
                rotation.write_options(writer, options)?;
                rotate_target.write_options(writer, options)?;
                duration.write_options(writer, options)?;
                easing.write_options(writer, options)?;
            }
            ActionType::SetVariable { variable, value } => {
                variable.write_options(writer, options)?;

                if let Some(value) = value {
                    writer.write_all(&[1])?;
                    value.write_options(writer, options)?;
                } else {
                    writer.write_all(&[0])?;
                }
            }
            ActionType::ResetVariable { variable } => {
                variable.write_options(writer, options)?;
            }
            ActionType::ResetObject { target_objects } => {
                target_objects.write_options(writer, options)?;
            }
            ActionType::SetColor {
                target_objects,
                color,
                channel,
                duration,
                easing,
            } => {
                target_objects.write_options(writer, options)?;
                color.write_options(writer, options)?;
                channel.write_options(writer, options)?;
                duration.write_options(writer, options)?;
                easing.write_options(writer, options)?;
            }
            ActionType::SetTransparency {
                target_objects,
                transparency,
                channel,
                duration,
                easing,
            } => {
                target_objects.write_options(writer, options)?;
                transparency.write_options(writer, options)?;
                channel.write_options(writer, options)?;
                duration.write_options(writer, options)?;
                easing.write_options(writer, options)?;
            }
            ActionType::SetSecondaryColor {
                target_objects,
                color,
                duration,
                easing,
            } => {
                target_objects.write_options(writer, options)?;
                color.write_options(writer, options)?;
                duration.write_options(writer, options)?;
                easing.write_options(writer, options)?;
            }
            ActionType::SetSecondaryTransparency {
                target_objects,
                transparency,
                duration,
                easing,
            } => {
                target_objects.write_options(writer, options)?;
                transparency.write_options(writer, options)?;
                duration.write_options(writer, options)?;
                easing.write_options(writer, options)?;
            }
            ActionType::SetBorderColor {
                target_objects,
                color,
                duration,
                easing,
            } => {
                target_objects.write_options(writer, options)?;
                color.write_options(writer, options)?;
                duration.write_options(writer, options)?;
                easing.write_options(writer, options)?;
            }
            ActionType::SetBorderTransparency {
                target_objects,
                transparency,
                duration,
                easing,
            } => {
                target_objects.write_options(writer, options)?;
                transparency.write_options(writer, options)?;
                duration.write_options(writer, options)?;
                easing.write_options(writer, options)?;
            }
            ActionType::SetSprite {
                target_objects,
                sprite,
            } => {
                target_objects.write_options(writer, options)?;
                sprite.write_options(writer, options)?;
            }
            ActionType::SetText {
                target_objects,
                text,
            } => {
                target_objects.write_options(writer, options)?;
                text.write_options(writer, options)?;
            }
            ActionType::SetEnabled {
                target_objects,
                enabled,
            } => {
                target_objects.write_options(writer, options)?;
                enabled.write_options(writer, options)?;
            }
            ActionType::Activate { target_objects } => {
                target_objects.write_options(writer, options)?;
            }
            ActionType::Deactivate { target_objects } => {
                target_objects.write_options(writer, options)?;
            }
            ActionType::Damage {
                target_objects,
                damage,
            } => {
                target_objects.write_options(writer, options)?;
                damage.write_options(writer, options)?;
            }
            ActionType::Kill { target_objects } => {
                target_objects.write_options(writer, options)?;
            }
            ActionType::GameFinish {} => {}
            ActionType::CameraPan {
                position,
                duration,
                easing,
            } => {
                position.write_options(writer, options)?;
                duration.write_options(writer, options)?;
                easing.write_options(writer, options)?;
            }
            ActionType::CameraFollowPlayer {} => {}
            ActionType::CameraZoom {
                viewport_size,
                duration,
                easing,
            } => {
                viewport_size.write_options(writer, options)?;
                duration.write_options(writer, options)?;
                easing.write_options(writer, options)?;
            }
            ActionType::CameraZoomReset { duration, easing } => {
                duration.write_options(writer, options)?;
                easing.write_options(writer, options)?;
            }
            ActionType::CameraOffset {
                offset,
                duration,
                easing,
            } => {
                offset.write_options(writer, options)?;
                duration.write_options(writer, options)?;
                easing.write_options(writer, options)?;
            }
            ActionType::CameraOffsetReset { duration, easing } => {
                duration.write_options(writer, options)?;
                easing.write_options(writer, options)?;
            }
            ActionType::CameraShake {
                strength,
                roughness,
                fade_in,
                fade_out,
                duration,
            } => {
                strength.write_options(writer, options)?;
                roughness.write_options(writer, options)?;
                fade_in.write_options(writer, options)?;
                fade_out.write_options(writer, options)?;
                duration.write_options(writer, options)?;
            }
            ActionType::PlaySound {
                sound,
                volume,
                pitch,
            } => {
                sound.write_options(writer, options)?;
                volume.write_options(writer, options)?;
                pitch.write_options(writer, options)?;
            }
            ActionType::PlayMusic {
                music,
                volume,
                pitch,
            } => {
                music.write_options(writer, options)?;
                volume.write_options(writer, options)?;
                pitch.write_options(writer, options)?;
            }
            ActionType::SetDirection {
                target_objects,
                direction,
            } => {
                target_objects.write_options(writer, options)?;
                direction.write_options(writer, options)?;
            }
            ActionType::SetGravity {
                target_objects,
                gravity,
            } => {
                target_objects.write_options(writer, options)?;
                gravity.write_options(writer, options)?;
            }
            ActionType::SetVelocity {
                target_objects,
                velocity,
            } => {
                target_objects.write_options(writer, options)?;
                velocity.write_options(writer, options)?;
            }
            ActionType::SetCinematic { enabled } => {
                enabled.write_options(writer, options)?;
            }
            ActionType::SetInputEnabled { enabled } => {
                enabled.write_options(writer, options)?;
            }
            ActionType::SetTimerEnabled { enabled } => {
                enabled.write_options(writer, options)?;
            }
            ActionType::GameTextShow { text, duration } => {
                text.write_options(writer, options)?;
                duration.write_options(writer, options)?;
            }
            ActionType::DialogueShow {
                text,
                position,
                reverse_direction,
            } => {
                text.write_options(writer, options)?;
                position.write_options(writer, options)?;
                reverse_direction.write_options(writer, options)?;
            }
            ActionType::StopScript { script } => {
                script.write_options(writer, options)?;
            }
            ActionType::TransitionIn {
                type_,
                color,
                duration,
                easing,
            } => {
                type_.write_options(writer, options)?;
                color.write_options(writer, options)?;
                duration.write_options(writer, options)?;
                easing.write_options(writer, options)?;
            }
            ActionType::TransitionOut {
                type_,
                color,
                duration,
                easing,
            } => {
                type_.write_options(writer, options)?;
                color.write_options(writer, options)?;
                duration.write_options(writer, options)?;
                easing.write_options(writer, options)?;
            }
            ActionType::TimeScale {
                time_scale,
                duration,
                easing,
            } => {
                time_scale.write_options(writer, options)?;
                duration.write_options(writer, options)?;
                easing.write_options(writer, options)?;
            }
            ActionType::RunFunction { function } => {
                function.write_options(writer, options)?;
            }
            ActionType::SetVariableOverTime {
                variable,
                value,
                duration,
                easing,
            } => {
                variable.write_options(writer, options)?;

                if let Some(value) = value {
                    writer.write_all(&[1])?;
                    value.write_options(writer, options)?;
                } else {
                    writer.write_all(&[0])?;
                }

                duration.write_options(writer, options)?;
                easing.write_options(writer, options)?;
            }
            ActionType::RepeatForEachObject {
                target_objects,
                actions,
            } => {
                target_objects.write_options(writer, options)?;
                actions.write_options(writer, options)?;
            }
        };

        Ok(())
    }
}

#[derive(Debug)]
pub enum ActionType {
    Repeat {
        actions: MyVec<Action>,
        count: NovaValue,
    },
    RepeatWhile {
        actions: MyVec<Action>,
        condition: NovaValue,
    },
    ConditionBlock {
        if_actions: MyVec<Action>,
        else_actions: MyVec<Action>,
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
    GameFinish {},
    CameraPan {
        position: NovaValue,
        duration: NovaValue,
        easing: NovaValue,
    },
    CameraFollowPlayer {},
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
        actions: MyVec<Action>,
    },
}

#[binread::derive_binread]
#[derive(Debug)]
pub struct NovaValue {
    #[br(map = |x: i32| x.try_into().unwrap())]
    pub dynamic_type: DynamicType,
    #[br(map = |x: u8| x != 0)]
    pub bool_value: bool,
    pub int_value: i32,
    pub float_value: f32,

    #[br(temp)]
    #[br(map = |x: u8| x != 0)]
    has_string_value: bool,

    #[br(if(has_string_value))]
    pub string_value: Option<MyString>,

    pub color_value: Colour,
    pub vector_value: Vec2,

    #[br(temp)]
    #[br(map = |x: u8| x != 0)]
    has_int_list: bool,

    #[br(if(has_int_list))]
    pub int_list_value: Option<MyVec<i32>>,

    #[br(temp)]
    #[br(map = |x: u8| x != 0)]
    has_sub_values: bool,

    #[br(if(has_sub_values))]
    pub sub_values: Option<MyVec<NovaValue>>,
}

impl BinWrite for NovaValue {
    fn write_options<W: std::io::prelude::Write>(
        &self,
        writer: &mut W,
        options: &binwrite::WriterOption,
    ) -> std::io::Result<()> {
        writer.write_all(&Into::<i32>::into(&self.dynamic_type).to_le_bytes())?;

        writer.write_all(&[bool_to_u8(self.bool_value)])?;

        writer.write_all(&self.int_value.to_le_bytes())?;

        writer.write_all(&self.float_value.to_le_bytes())?;

        if let Some(string_value) = &self.string_value {
            writer.write_all(&[1])?;
            string_value.write_options(writer, options)?;
        } else {
            writer.write_all(&[0])?;
        }

        self.color_value.write_options(writer, options)?;

        self.vector_value.write_options(writer, options)?;

        if let Some(int_list_value) = &self.int_list_value {
            writer.write_all(&[1])?;
            int_list_value.write_options(writer, options)?;
        } else {
            writer.write_all(&[0])?;
        }

        if let Some(sub_values) = &self.sub_values {
            writer.write_all(&[1])?;
            sub_values.write_options(writer, options)?;
        } else {
            writer.write_all(&[0])?;
        }

        Ok(())
    }
}

macro_rules! define_dynamic_type {
    ($($name:ident = $number:expr),*) => {
        #[derive(Debug)]
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
    VectorPointerPosition = 168,
    VectorPointerWorldPosition = 169,
    VectorCollisionPoint = 170,
    VectorCollisionNormal = 171,
    ObjectRepeatObject = 172,
    VectorClosestFromPoint = 173,
    ObjectSetAllObjects = 174,
    ObjectSetObjectsInLayer = 175,
    ObjectSetObjectsInCircle = 176,
    LayerConstant = 177,
    LayerVariable = 178,
    LayerParameter = 179
);

#[derive(Debug, BinRead, BinWrite)]
pub struct FunctionCall {
    pub id: i32,
    pub parameters: MyVec<CallParameter>,
}

#[derive(Debug, BinRead, BinWrite)]
pub struct CallParameter {
    pub parameter_id: i32,
    pub value: NovaValue,
}

#[derive(Debug, BinRead, BinWrite)]
pub struct Variable {
    pub variable_id: i32,
    pub name: MyString,
    #[br(map = |x: i32| x.try_into().unwrap())]
    pub static_type: StaticType,
    pub initial_value: NovaValue,
}

macro_rules! define_static_type {
    ($($name:ident = $number:expr),*) => {
        #[derive(Debug)]
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
    Color = 4,
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

impl BinWrite for StaticType {
    fn write_options<W: std::io::prelude::Write>(
        &self,
        writer: &mut W,
        _options: &binwrite::WriterOption,
    ) -> std::io::Result<()> {
        writer.write_all(&Into::<i32>::into(self).to_le_bytes())
    }
}

#[derive(Debug, BinRead, BinWrite)]
pub struct Activator {
    pub activator_type: i32,
    pub parameters: MyVec<NovaValue>,
}

#[derive(Debug, BinRead, BinWrite)]
pub struct Parameter {
    pub parameter_id: i32,
    pub name: MyString,
    #[br(map = |x: i32| x.try_into().unwrap())]
    pub static_type: StaticType,
    pub default_value: NovaValue,
}
