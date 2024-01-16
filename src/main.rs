#![allow(dead_code)]

use base64::{prelude::BASE64_STANDARD, Engine};
use binread::{BinRead, BinReaderExt as _, BinResult, ReadOptions};
use std::io::{Read, Seek};

const SERIALIZATION_VERSION: i32 = 16;

fn main() {
    let mut file_reader = std::fs::File::open("testlvl.exolvl.bin").unwrap();

    let exolvl: Exolvl = file_reader.read_le().unwrap();

    println!("{:#?}", exolvl);
}

#[derive(Debug, BinRead)]
#[br(magic = b"NYA^")]
struct Exolvl {
    local_level: LocalLevel,
    level_data: LevelData,
    author_replay: AuthorReplay,
}

#[derive(Debug, BinRead)]
struct LocalLevel {
    serialization_version: i32,
    #[br(map = |x: MyString| x.inner)]
    level_id: String,
    level_version: i32,
    #[br(map = |x: MyString| x.inner)]
    level_name: String,
    #[br(map = |x: MyString| x.inner)]
    thumbnail: String,
    #[br(map = |x: MyDateTime| x.inner)]
    creation_date: chrono::DateTime<chrono::Utc>,
    #[br(map = |x: MyDateTime| x.inner)]
    update_date: chrono::DateTime<chrono::Utc>,
    author_time: i64,
    #[br(map = |x: MyVec<i64>| x.inner)]
    author_lap_times: Vec<i64>,
    silver_medal_time: i64,
    gold_medal_time: i64,
    laps: i32,
    #[br(map = |x: u8| x != 0)]
    #[br(pad_after = 1)]
    private: bool,
}

#[derive(Debug)]
struct MyDateTime {
    inner: chrono::DateTime<chrono::Utc>,
}

impl BinRead for MyDateTime {
    type Args = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        _options: &ReadOptions,
        _args: Self::Args,
    ) -> BinResult<Self> {
        const TICKS_TO_SECONDS: i64 = 10_000_000;
        const EPOCH_DIFFERENCE: i64 = 62_135_596_800;

        let ticks = reader.read_le::<i64>()? & 0x3FFFFFFFFFFFFFFF;
        let seconds = ticks / TICKS_TO_SECONDS - EPOCH_DIFFERENCE;

        Ok(MyDateTime {
            inner: chrono::DateTime::<chrono::Utc>::from_timestamp(seconds, 0).unwrap(),
        })
    }
}

#[derive(Debug, BinRead)]
struct LevelData {
    #[br(map = |x: MyString| x.inner)]
    level_id: String,
    level_version: i32,
    #[br(map = |x: u8| x != 0)]
    nova_level: bool,
    #[br(map = |x: MyVec<i32>| x.inner)]
    under_decoration_tiles: Vec<i32>,
    #[br(map = |x: MyVec<i32>| x.inner)]
    background_decoration_tiles_2: Vec<i32>,
    #[br(map = |x: MyVec<i32>| x.inner)]
    terrain_tiles: Vec<i32>,
    #[br(map = |x: MyVec<i32>| x.inner)]
    floating_zone_tiles: Vec<i32>,
    #[br(map = |x: MyVec<i32>| x.inner)]
    object_tiles: Vec<i32>,
    #[br(map = |x: MyVec<i32>| x.inner)]
    foreground_decoration_tiles: Vec<i32>,
    #[br(map = |x: MyVec<Object>| x.inner)]
    objects: Vec<Object>,
    #[br(map = |x: MyVec<Layer>| x.inner)]
    layers: Vec<Layer>,
    #[br(map = |x: MyVec<Prefab>| x.inner)]
    prefabs: Vec<Prefab>,
    #[br(map = |x: MyVec<Brush>| x.inner)]
    brushes: Vec<Brush>,
    #[br(map = |x: MyVec<Pattern>| x.inner)]
    patterns: Vec<Pattern>,
    author_time: i64,
    #[br(map = |x: MyVec<i64>| x.inner)]
    author_lap_times: Vec<i64>,
    silver_medal_time: i64,
    gold_medal_time: i64,
    laps: i32,
    #[br(map = |x: u8| x != 0)]
    center_camera: bool,
    #[br(map = |x: MyVec<i32>| x.inner)]
    scripts: Vec<i32>,
    #[br(map = |x: MyVec<NovaScript>| x.inner)]
    nova_scripts: Vec<NovaScript>,
    #[br(map = |x: MyVec<Variable>| x.inner)]
    global_variables: Vec<Variable>,
    #[br(map = |x: MyString| x.inner)]
    theme: String,
    custom_background_colour: Colour,

    #[br(pad_before = 24)]
    _unknown1: (),

    custom_terrain_colour: Colour,

    #[br(pad_before = 20)]
    _unknown_2: (),

    custom_terrain_border_colour: Colour,
    custom_terrain_border_thickness: f32,
    custom_terrain_border_corner_radius: f32,

    #[br(pad_before = 6)]
    _unknown_3: (),

    #[br(map = |x: u8| x != 0)]
    default_music: bool,
    #[br(map = |x: MyVec<MyString>| x.inner.into_iter().map(|x| x.inner).collect())]
    music_ids: Vec<String>,
    #[br(map = |x: u8| x != 0)]
    allow_direction_change: bool,
    #[br(map = |x: u8| x != 0)]
    disable_replays: bool,
    #[br(map = |x: u8| x != 0)]
    disable_revive_pads: bool,
    #[br(map = |x: u8| x != 0)]
    disable_start_animation: bool,
    gravity: Vec2,
}

#[derive(Debug, BinRead)]
struct NovaScript {
    script_id: i32,
    #[br(map = |x: MyString| x.inner)]
    script_name: String,
    #[br(map = |x: u8| x != 0)]
    is_function: bool,
    activation_count: i32,
    condition: NovaValue,
    #[br(map = |x: MyVec<Activator>| x.inner)]
    activation_list: Vec<Activator>,
    #[br(map = |x: MyVec<Parameter>| x.inner)]
    parameters: Vec<Parameter>,
    #[br(map = |x: MyVec<Variable>| x.inner)]
    variables: Vec<Variable>,
    #[br(map = |x: MyVec<Action>| x.inner)]
    actions: Vec<Action>,
}

#[derive(Debug)]
struct Action {
    closed: bool,
    wait: bool,
    action_type: ActionType,
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

        Ok(Action {
            closed,
            wait,
            action_type: match action_type {
                0 => {
                    let actions = reader.read_le::<MyVec<Action>>()?.inner;
                    let count = reader.read_le::<NovaValue>()?;

                    ActionType::Repeat { actions, count }
                }
                1 => {
                    let actions = reader.read_le::<MyVec<Action>>()?.inner;
                    let condition = reader.read_le::<NovaValue>()?;

                    ActionType::RepeatWhile { actions, condition }
                }
                2 => {
                    let if_actions = reader.read_le::<MyVec<Action>>()?.inner;
                    let else_actions = reader.read_le::<MyVec<Action>>()?.inner;
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
                    let actions = reader.read_le::<MyVec<Action>>()?.inner;

                    ActionType::RepeatForEachObject {
                        target_objects,
                        actions,
                    }
                }
                _ => panic!("Unknown action type: {}", action_type),
            },
        })
    }
}

#[derive(Debug)]
enum ActionType {
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
        actions: Vec<Action>,
    },
}

#[derive(Debug, BinRead)]
struct FunctionCall {
    id: i32,
    #[br(map = |x: MyVec<CallParameter>| x.inner)]
    parameters: Vec<CallParameter>,
}

#[derive(Debug, BinRead)]
struct CallParameter {
    parameter_id: i32,
    value: NovaValue,
}

#[derive(Debug, BinRead)]
struct Variable {
    variable_id: i32,
    #[br(map = |x: MyString| x.inner)]
    name: String,
    #[br(map = |x: i32| x.try_into().unwrap())]
    static_type: StaticType,
    dynamic_type: NovaValue,
}

#[derive(Debug, BinRead)]
struct Parameter {
    parameter_id: i32,
    #[br(map = |x: MyString| x.inner)]
    name: String,
    #[br(map = |x: i32| x.try_into().unwrap())]
    static_type: StaticType,
    dynamic_type: NovaValue,
}

macro_rules! define_static_type {
    ($($name:ident = $number:expr),*) => {
        #[derive(Debug)]
        enum StaticType {
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

#[derive(Debug, BinRead)]
struct Activator {
    activator_type: i32,
    #[br(map = |x: MyVec<NovaValue>| x.inner)]
    parameters: Vec<NovaValue>,
}

#[binread::derive_binread]
#[derive(Debug)]
struct NovaValue {
    #[br(map = |x: i32| x.try_into().unwrap())]
    dynamic_type: DynamicType,
    #[br(map = |x: u8| x != 0)]
    bool_value: bool,
    int_value: i32,
    float_value: f32,

    #[br(temp)]
    #[br(map = |x: u8| x != 0)]
    has_string_value: bool,

    #[br(if(has_string_value))]
    #[br(map = |x: MyString| Some(x.inner))]
    string_value: Option<String>,

    color_value: Colour,
    vector_value: Vec2,

    #[br(temp)]
    #[br(map = |x: u8| x != 0)]
    has_int_list: bool,

    #[br(if(has_int_list))]
    #[br(map = |x: MyVec<i32>| Some(x.inner))]
    int_list_value: Option<Vec<i32>>,

    #[br(temp)]
    #[br(map = |x: u8| x != 0)]
    has_sub_values: bool,

    #[br(if(has_sub_values))]
    #[br(map = |x: MyVec<NovaValue>| Some(x.inner))]
    sub_values: Option<Vec<NovaValue>>,
}

macro_rules! define_dynamic_type {
    ($($name:ident = $number:expr),*) => {
        #[derive(Debug)]
        enum DynamicType {
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

#[derive(Debug, BinRead)]
struct Pattern {
    pattern_id: i32,
    #[br(map = |x: MyVec<Image>| x.inner)]
    pattern_frames: Vec<Image>,
}

#[derive(Debug, BinRead)]
struct Brush {
    brush_id: i32,
    spread: Vec2,
    frequency: f32,
    grid: BrushGrid,
    #[br(map = |x: MyVec<BrushObject>| x.inner)]
    objects: Vec<BrushObject>,
}

#[derive(Debug, BinRead)]
struct BrushObject {
    entity_id: i32,
    #[br(map = |x: MyVec<ObjectProperty>| x.inner)]
    properties: Vec<ObjectProperty>,
    weight: f32,
    scale: f32,
    rotation: f32,
    #[br(map = |x: u8| x != 0)]
    flip_x: bool,
    #[br(map = |x: u8| x != 0)]
    flip_y: bool,
}

#[derive(Debug, BinRead)]
struct BrushGrid {
    x: i32,
    y: i32,
}

#[derive(Debug, BinRead)]
struct Prefab {
    prefab_id: i32,
    prefab_image_data: Image,
    #[br(map = |x: MyVec<Object>| x.inner)]
    items: Vec<Object>,
}

#[derive(Debug, BinRead)]
struct Image(#[br(map = |x: MyVec<u8>| BASE64_STANDARD.encode(x.inner))] String);

#[derive(Debug, BinRead)]
struct Layer {
    layer_id: i32,
    #[br(map = |x: MyString| x.inner)]
    layer_name: String,
    #[br(map = |x: u8| x != 0)]
    selected: bool,
    #[br(map = |x: u8| x != 0)]
    invisible: bool,
    #[br(map = |x: u8| x != 0)]
    locked: bool,
    foreground_type: i32,
    parallax: Vec2,
    #[br(map = |x: u8| x != 0)]
    fixed_size: bool,
    #[br(map = |x: MyVec<i32>| x.inner)]
    children: Vec<i32>,
}

#[derive(Debug, BinRead)]
struct Object {
    entity_id: i32,
    tile_id: i32,
    #[br(if(SERIALIZATION_VERSION >= 12))]
    prefab_entity_id: Option<i32>,
    #[br(if(SERIALIZATION_VERSION >= 12))]
    prefab_id: Option<i32>,
    position: Vec2,
    scale: Vec2,
    rotation: f32,
    #[br(map = |x: MyString| x.inner)]
    tag: String,
    #[br(map = |x: MyVec<ObjectProperty>| x.inner)]
    properties: Vec<ObjectProperty>,
    in_layer: i32,
    in_group: i32,
    #[br(map = |x: MyVec<i32>| x.inner)]
    group_members: Vec<i32>,
}

#[derive(Debug)]
enum ObjectProperty {
    Colour(Colour),
    Resolution(i32),
    FillMode(i32),
    SecondaryColour(Colour),
    Thickness(f32),
    TotalAngle(i32),
    Corners(i32),
    Blending(i32),
    GridOffset(Vec2),
    CornerRadius(f32),
    Width(f32),
    Height(f32),
    BorderColour(Colour),
    BorderThickness(f32),
    PhysicsType(i32),
    Friction(f32),
    TerrainCorners(Vec<Vec<Vec2>>),
    Direction(i32),
    Impulse(i32),
    Killer(bool),
    RoundReflexAngles(bool),
    RoundCollider(bool),
    Radius(f32),
    Size(f32),
    ReverseDirection(bool),
    CollisionDetector(bool),
    Pattern(i32),
    PatternTiling(Vec2),
    PatternOffset(Vec2),
    Sprite(String),
    Trigger(bool),
    Health(f32),
    DamageFromJump(bool),
    DamageFromDash(bool),
    ReverseDirOnDamage(bool),
    Floating(bool),
    FlipX(bool),
    FlipY(bool),
    Text(String),
    FontSize(f32),
    EditorColour(Colour),
}

impl BinRead for ObjectProperty {
    type Args = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        _options: &ReadOptions,
        _args: Self::Args,
    ) -> BinResult<Self> {
        match reader.read_le::<i32>()? {
            0 => Ok(ObjectProperty::Colour(reader.read_le()?)),
            1 => Ok(ObjectProperty::Resolution(reader.read_le()?)),
            2 => Ok(ObjectProperty::FillMode(reader.read_le()?)),
            3 => Ok(ObjectProperty::SecondaryColour(reader.read_le()?)),
            4 => Ok(ObjectProperty::Thickness(reader.read_le()?)),
            5 => Ok(ObjectProperty::TotalAngle(reader.read_le()?)),
            6 => Ok(ObjectProperty::Corners(reader.read_le()?)),
            7 => Ok(ObjectProperty::Blending(reader.read_le()?)),
            8 => Ok(ObjectProperty::GridOffset(reader.read_le()?)),
            9 => Ok(ObjectProperty::CornerRadius(reader.read_le()?)),
            10 => Ok(ObjectProperty::Width(reader.read_le()?)),
            11 => Ok(ObjectProperty::Height(reader.read_le()?)),
            12 => Ok(ObjectProperty::BorderColour(reader.read_le()?)),
            13 => Ok(ObjectProperty::BorderThickness(reader.read_le()?)),
            14 => Ok(ObjectProperty::PhysicsType(reader.read_le()?)),
            15 => Ok(ObjectProperty::Friction(reader.read_le()?)),
            16 => Ok(ObjectProperty::TerrainCorners(
                reader
                    .read_le::<MyVec<MyVec<_>>>()?
                    .inner
                    .into_iter()
                    .map(|x| x.inner)
                    .collect(),
            )),
            17 => Ok(ObjectProperty::Direction(reader.read_le()?)),
            18 => Ok(ObjectProperty::Impulse(reader.read_le()?)),
            19 => Ok(ObjectProperty::Killer(reader.read_le::<u8>()? != 0)),
            20 => Ok(ObjectProperty::RoundReflexAngles(
                reader.read_le::<u8>()? != 0,
            )),
            21 => Ok(ObjectProperty::RoundCollider(reader.read_le::<u8>()? != 0)),
            22 => Ok(ObjectProperty::Radius(reader.read_le()?)),
            23 => Ok(ObjectProperty::Size(reader.read_le()?)),
            24 => Ok(ObjectProperty::ReverseDirection(
                reader.read_le::<u8>()? != 0,
            )),
            25 => Ok(ObjectProperty::CollisionDetector(
                reader.read_le::<u8>()? != 0,
            )),
            26 => Ok(ObjectProperty::Pattern(reader.read_le()?)),
            27 => Ok(ObjectProperty::PatternTiling(reader.read_le()?)),
            28 => Ok(ObjectProperty::PatternOffset(reader.read_le()?)),
            35 => Ok(ObjectProperty::Sprite(reader.read_le::<MyString>()?.inner)),
            36 => Ok(ObjectProperty::Trigger(reader.read_le::<u8>()? != 0)),
            37 => Ok(ObjectProperty::Health(reader.read_le()?)),
            38 => Ok(ObjectProperty::DamageFromJump(reader.read_le::<u8>()? != 0)),
            39 => Ok(ObjectProperty::DamageFromDash(reader.read_le::<u8>()? != 0)),
            40 => Ok(ObjectProperty::ReverseDirOnDamage(
                reader.read_le::<u8>()? != 0,
            )),
            41 => Ok(ObjectProperty::Floating(reader.read_le::<u8>()? != 0)),
            43 => Ok(ObjectProperty::FlipX(reader.read_le::<u8>()? != 0)),
            44 => Ok(ObjectProperty::FlipY(reader.read_le::<u8>()? != 0)),
            45 => Ok(ObjectProperty::Text(reader.read_le::<MyString>()?.inner)),
            46 => Ok(ObjectProperty::FontSize(reader.read_le()?)),
            47 => Ok(ObjectProperty::EditorColour(reader.read_le()?)),
            other => unreachable!("Unknown property id: {}", other),
        }
    }
}

#[derive(Debug, BinRead)]
struct Vec2 {
    x: f32,
    y: f32,
}

#[derive(Debug, BinRead)]
struct Colour {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

#[derive(Debug, BinRead)]
struct AuthorReplay {
    #[br(map = |x: MyVec<u8>| BASE64_STANDARD.encode(x.inner))]
    replay_data: String,
}

#[derive(Debug, BinRead)]
struct MyString {
    #[br(map = |x: VarInt| x.inner as u32)]
    len: u32,
    #[br(count = len)]
    #[br(map = |x: Vec<char>| x.into_iter().collect())]
    inner: String,
}

#[derive(Debug, BinRead)]
struct MyVec<T: BinRead<Args = ()>> {
    len: u32,
    #[br(count = len)]
    inner: Vec<T>,
}

#[derive(Debug, BinRead)]
struct VarInt {
    #[br(parse_with = parse_var_int)]
    inner: i32,
}

fn parse_var_int<R: Read + Seek>(
    reader: &mut R,
    _options: &ReadOptions,
    _args: (),
) -> BinResult<i32> {
    const SEGMENT_BITS: u8 = 0x7F;
    const CONTINUE_BIT: u8 = 0x80;

    let mut value = 0;
    let mut position = 0;

    loop {
        let current_byte = reader.read_le::<u8>()?;

        value |= ((current_byte & SEGMENT_BITS) << position) as i32;

        if current_byte & CONTINUE_BIT == 0 {
            break;
        }

        position += 7;
    }

    Ok(value)
}
