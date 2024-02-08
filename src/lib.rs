use chrono::{DateTime, Utc};

pub fn read(input: &mut impl Iterator<Item = u8>) -> Exolvl {
    Read::read(input)
}

trait Read {
    fn read(input: &mut impl Iterator<Item = u8>) -> Self;
}

trait ReadVersioned {
    fn read(input: &mut impl Iterator<Item = u8>, version: i32) -> Self;
}

trait ReadWith {
    type With;

    fn read_with(input: &mut impl Iterator<Item = u8>, with: Self::With) -> Self;
}

struct Varint(i32);

impl Read for Varint {
    fn read(input: &mut impl Iterator<Item = u8>) -> Self {
        const SEGMENT_BITS: u8 = 0x7F;
        const CONTINUE_BIT: u8 = 0x80;

        let mut value = 0;
        let mut position = 0;

        loop {
            let current_byte = u8::read(input);

            value |= i32::from(current_byte & SEGMENT_BITS) << position;

            if current_byte & CONTINUE_BIT == 0 {
                break;
            }

            position += 7;
        }

        Self(value)
    }
}

impl Read for String {
    fn read(input: &mut impl Iterator<Item = u8>) -> Self {
        let len = Varint::read(input);

        input.take(len.0 as usize).map(|b| b as char).collect()
    }
}

impl Read for DateTime<Utc> {
    fn read(input: &mut impl Iterator<Item = u8>) -> Self {
        MyDateTime(i64::read(input)).into()
    }
}

impl Read for i32 {
    fn read(input: &mut impl Iterator<Item = u8>) -> Self {
        let mut bytes = [0; 4];

        bytes.iter_mut().for_each(|byte| *byte = Read::read(input));

        i32::from_le_bytes(bytes)
    }
}

impl Read for i64 {
    fn read(input: &mut impl Iterator<Item = u8>) -> Self {
        let mut bytes = [0; 8];

        bytes.iter_mut().for_each(|byte| *byte = Read::read(input));

        i64::from_le_bytes(bytes)
    }
}

impl Read for f32 {
    fn read(input: &mut impl Iterator<Item = u8>) -> Self {
        let mut bytes = [0; 4];

        bytes.iter_mut().for_each(|byte| *byte = Read::read(input));

        f32::from_le_bytes(bytes)
    }
}

impl<T: Read> Read for Vec<T> {
    fn read(input: &mut impl Iterator<Item = u8>) -> Self {
        let len = i32::read(input) as usize;

        (0..len).map(|_| Read::read(input)).collect()
    }
}

impl<T: Read + Copy + Default, const LEN: usize> Read for [T; LEN] {
    fn read(input: &mut impl Iterator<Item = u8>) -> Self {
        let mut arr = [Default::default(); LEN];

        arr.iter_mut().for_each(|item| *item = Read::read(input));

        arr
    }
}

impl<T: Read> Read for Option<T> {
    fn read(input: &mut impl Iterator<Item = u8>) -> Self {
        if bool::read(input) {
            Some(Read::read(input))
        } else {
            None
        }
    }
}

impl Read for bool {
    fn read(input: &mut impl Iterator<Item = u8>) -> Self {
        u8::read(input) != 0
    }
}

impl Read for u8 {
    fn read(input: &mut impl Iterator<Item = u8>) -> Self {
        input.next().unwrap()
    }
}

#[derive(Debug)]
pub struct Exolvl {
    pub local_level: LocalLevel,
    pub level_data: LevelData,
    pub author_replay: AuthorReplay,
}

impl Read for Exolvl {
    fn read(input: &mut impl Iterator<Item = u8>) -> Self {
        let magic: [u8; 4] = Read::read(input);

        if &magic != b"NYA^" {
            panic!("Invalid magic");
        }

        let local_level = LocalLevel::read(input);
        let level_data = ReadVersioned::read(input, local_level.serialization_version);
        let author_replay = Read::read(input);

        Self {
            local_level,
            level_data,
            author_replay,
        }
    }
}

#[derive(Debug)]
pub struct LocalLevel {
    pub serialization_version: i32,
    pub level_id: String,
    pub level_version: i32,
    pub level_name: String,
    pub thumbnail: String,
    pub creation_date: DateTime<Utc>,
    pub update_date: DateTime<Utc>,
    pub author_time: i64,
    pub author_lap_times: Vec<i64>,
    pub silver_medal_time: i64,
    pub gold_medal_time: i64,
    pub laps: i32,
    pub private: bool,

    unknown_1: u8,
}

impl Read for LocalLevel {
    fn read(input: &mut impl Iterator<Item = u8>) -> Self {
        Self {
            serialization_version: Read::read(input),
            level_id: Read::read(input),
            level_version: Read::read(input),
            level_name: Read::read(input),
            thumbnail: Read::read(input),
            creation_date: Read::read(input),
            update_date: Read::read(input),
            author_time: Read::read(input),
            author_lap_times: Read::read(input),
            silver_medal_time: Read::read(input),
            gold_medal_time: Read::read(input),
            laps: Read::read(input),
            private: Read::read(input),
            unknown_1: Read::read(input),
        }
    }
}

#[derive(Debug)]
pub struct MyDateTime(i64);

impl MyDateTime {
    const TICKS_TO_SECONDS: i64 = 10_000_000;
    const EPOCH_DIFFERENCE: i64 = 62_135_596_800;
}

impl From<DateTime<Utc>> for MyDateTime {
    fn from(datetime: DateTime<Utc>) -> Self {
        let ticks = (datetime.timestamp() + Self::EPOCH_DIFFERENCE) * Self::TICKS_TO_SECONDS;

        Self(ticks)
    }
}

impl From<MyDateTime> for DateTime<Utc> {
    fn from(my_datetime: MyDateTime) -> Self {
        let masked_ticks = my_datetime.0 & 0x3FFF_FFFF_FFFF_FFFF;
        let seconds = masked_ticks / MyDateTime::TICKS_TO_SECONDS - MyDateTime::EPOCH_DIFFERENCE;

        DateTime::<Utc>::from_timestamp(seconds, 0).unwrap()
    }
}

#[derive(Debug)]
pub struct LevelData {
    pub level_id: String,
    pub level_version: i32,
    pub nova_level: bool,
    pub under_decoration_tiles: Vec<i32>,
    pub background_decoration_tiles_2: Vec<i32>,
    pub terrain_tiles: Vec<i32>,
    pub floating_zone_tiles: Vec<i32>,
    pub object_tiles: Vec<i32>,
    pub foreground_decoration_tiles: Vec<i32>,
    pub objects: Vec<Object>,
    pub layers: Vec<Layer>,
    pub prefabs: Vec<Prefab>,
    pub brushes: Vec<Brush>,
    pub patterns: Vec<Pattern>,
    pub colour_palette: Option<Vec<Colour>>,
    pub author_time: i64,
    pub author_lap_times: Vec<i64>,
    pub silver_medal_time: i64,
    pub gold_medal_time: i64,
    pub laps: i32,
    pub center_camera: bool,
    pub scripts: Vec<i32>,
    pub nova_scripts: Vec<NovaScript>,
    pub global_variables: Vec<Variable>,
    pub theme: String,
    pub custom_background_colour: Colour,

    _unknown1: [u8; 24],

    pub custom_terrain_colour: Colour,

    _unknown_2: [u8; 20],

    pub custom_terrain_border_colour: Colour,
    pub custom_terrain_border_thickness: f32,
    pub custom_terrain_border_corner_radius: f32,

    _unknown_3: [u8; 6],

    pub default_music: bool,
    pub music_ids: Vec<String>,
    pub allow_direction_change: bool,
    pub disable_replays: bool,
    pub disable_revive_pads: bool,
    pub disable_start_animation: bool,
    pub gravity: Vec2,
}

impl ReadVersioned for LevelData {
    fn read(input: &mut impl Iterator<Item = u8>, version: i32) -> Self {
        Self {
            level_id: Read::read(input),
            level_version: Read::read(input),
            nova_level: Read::read(input),
            under_decoration_tiles: Read::read(input),
            background_decoration_tiles_2: Read::read(input),
            terrain_tiles: Read::read(input),
            floating_zone_tiles: Read::read(input),
            object_tiles: Read::read(input),
            foreground_decoration_tiles: Read::read(input),
            objects: Read::read(input),
            layers: Read::read(input),
            prefabs: Read::read(input),
            brushes: Read::read(input),
            patterns: Read::read(input),
            colour_palette: (version >= 17).then(|| Read::read(input)),
            author_time: Read::read(input),
            author_lap_times: Read::read(input),
            silver_medal_time: Read::read(input),
            gold_medal_time: Read::read(input),
            laps: Read::read(input),
            center_camera: Read::read(input),
            scripts: Read::read(input),
            nova_scripts: Read::read(input),
            global_variables: Read::read(input),
            theme: Read::read(input),
            custom_background_colour: Read::read(input),
            _unknown1: Read::read(input),
            custom_terrain_colour: Read::read(input),
            _unknown_2: Read::read(input),
            custom_terrain_border_colour: Read::read(input),
            custom_terrain_border_thickness: Read::read(input),
            custom_terrain_border_corner_radius: Read::read(input),
            _unknown_3: Read::read(input),
            default_music: Read::read(input),
            music_ids: Read::read(input),
            allow_direction_change: Read::read(input),
            disable_replays: Read::read(input),
            disable_revive_pads: Read::read(input),
            disable_start_animation: Read::read(input),
            gravity: Read::read(input),
        }
    }
}

#[derive(Debug)]
pub struct Pattern {
    pub pattern_id: i32,
    pub pattern_frames: Vec<Image>,
}

impl Read for Pattern {
    fn read(input: &mut impl Iterator<Item = u8>) -> Self {
        Self {
            pattern_id: Read::read(input),
            pattern_frames: Read::read(input),
        }
    }
}

#[derive(Debug)]
pub struct Prefab {
    pub prefab_id: i32,
    pub prefab_image_data: Image,
    pub items: Vec<Object>,
}

impl Read for Prefab {
    fn read(input: &mut impl Iterator<Item = u8>) -> Self {
        Self {
            prefab_id: Read::read(input),
            prefab_image_data: Read::read(input),
            items: Read::read(input),
        }
    }
}

#[derive(Debug)]
pub struct Image(pub Vec<u8>);

impl Read for Image {
    fn read(input: &mut impl Iterator<Item = u8>) -> Self {
        let data = Read::read(input);

        Self(data)
    }
}

#[derive(Debug)]
pub struct Layer {
    pub layer_id: i32,
    pub layer_name: String,
    pub selected: bool,
    pub invisible: bool,
    pub locked: bool,
    pub foreground_type: i32,
    pub parallax: Vec2,
    pub fixed_size: bool,
    pub children: Vec<i32>,
}

impl Read for Layer {
    fn read(input: &mut impl Iterator<Item = u8>) -> Self {
        Self {
            layer_id: Read::read(input),
            layer_name: Read::read(input),
            selected: Read::read(input),
            invisible: Read::read(input),
            locked: Read::read(input),
            foreground_type: Read::read(input),
            parallax: Read::read(input),
            fixed_size: Read::read(input),
            children: Read::read(input),
        }
    }
}

#[derive(Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Read for Vec2 {
    fn read(input: &mut impl Iterator<Item = u8>) -> Self {
        Self {
            x: Read::read(input),
            y: Read::read(input),
        }
    }
}

#[derive(Debug)]
pub struct Colour {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Read for Colour {
    fn read(input: &mut impl Iterator<Item = u8>) -> Self {
        Self {
            r: Read::read(input),
            g: Read::read(input),
            b: Read::read(input),
            a: Read::read(input),
        }
    }
}

#[derive(Debug)]
pub struct AuthorReplay {
    pub replay_data: Vec<u8>,
}

impl Read for AuthorReplay {
    fn read(input: &mut impl Iterator<Item = u8>) -> Self {
        Self {
            replay_data: Read::read(input),
        }
    }
}

#[derive(Debug)]
pub struct Object {
    pub entity_id: i32,
    pub tile_id: i32,
    pub prefab_entity_id: i32,
    pub prefab_id: i32,
    pub position: Vec2,
    pub scale: Vec2,
    pub rotation: f32,
    pub tag: String,
    pub properties: Vec<ObjectProperty>,
    pub in_layer: i32,
    pub in_group: i32,
    pub group_members: Vec<i32>,
}

impl Read for Object {
    fn read(input: &mut impl Iterator<Item = u8>) -> Self {
        Self {
            entity_id: Read::read(input),
            tile_id: Read::read(input),
            prefab_entity_id: Read::read(input),
            prefab_id: Read::read(input),
            position: Read::read(input),
            scale: Read::read(input),
            rotation: Read::read(input),
            tag: Read::read(input),
            properties: Read::read(input),
            in_layer: Read::read(input),
            in_group: Read::read(input),
            group_members: Read::read(input),
        }
    }
}

#[derive(Debug)]
pub enum ObjectProperty {
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
    MoonInnerRadius(f32),
    MoonOffset(f32),
}

impl Read for ObjectProperty {
    fn read(input: &mut impl Iterator<Item = u8>) -> Self {
        let property_type = Read::read(input);

        match property_type {
            0 => ObjectProperty::Colour(Read::read(input)),
            1 => ObjectProperty::Resolution(Read::read(input)),
            2 => ObjectProperty::FillMode(Read::read(input)),
            3 => ObjectProperty::SecondaryColour(Read::read(input)),
            4 => ObjectProperty::Thickness(Read::read(input)),
            5 => ObjectProperty::TotalAngle(Read::read(input)),
            6 => ObjectProperty::Corners(Read::read(input)),
            7 => ObjectProperty::Blending(Read::read(input)),
            8 => ObjectProperty::GridOffset(Read::read(input)),
            9 => ObjectProperty::CornerRadius(Read::read(input)),
            10 => ObjectProperty::Width(Read::read(input)),
            11 => ObjectProperty::Height(Read::read(input)),
            12 => ObjectProperty::BorderColour(Read::read(input)),
            13 => ObjectProperty::BorderThickness(Read::read(input)),
            14 => ObjectProperty::PhysicsType(Read::read(input)),
            15 => ObjectProperty::Friction(Read::read(input)),
            16 => ObjectProperty::TerrainCorners(Read::read(input)),
            17 => ObjectProperty::Direction(Read::read(input)),
            18 => ObjectProperty::Impulse(Read::read(input)),
            19 => ObjectProperty::Killer(Read::read(input)),
            20 => ObjectProperty::RoundReflexAngles(Read::read(input)),
            21 => ObjectProperty::RoundCollider(Read::read(input)),
            22 => ObjectProperty::Radius(Read::read(input)),
            23 => ObjectProperty::Size(Read::read(input)),
            24 => ObjectProperty::ReverseDirection(Read::read(input)),
            25 => ObjectProperty::CollisionDetector(Read::read(input)),
            26 => ObjectProperty::Pattern(Read::read(input)),
            27 => ObjectProperty::PatternTiling(Read::read(input)),
            28 => ObjectProperty::PatternOffset(Read::read(input)),
            35 => ObjectProperty::Sprite(Read::read(input)),
            36 => ObjectProperty::Trigger(Read::read(input)),
            37 => ObjectProperty::Health(Read::read(input)),
            38 => ObjectProperty::DamageFromJump(Read::read(input)),
            39 => ObjectProperty::DamageFromDash(Read::read(input)),
            40 => ObjectProperty::ReverseDirOnDamage(Read::read(input)),
            41 => ObjectProperty::Floating(Read::read(input)),
            43 => ObjectProperty::FlipX(Read::read(input)),
            44 => ObjectProperty::FlipY(Read::read(input)),
            45 => ObjectProperty::Text(Read::read(input)),
            46 => ObjectProperty::FontSize(Read::read(input)),
            47 => ObjectProperty::EditorColour(Read::read(input)),
            83 => ObjectProperty::MoonInnerRadius(Read::read(input)),
            84 => ObjectProperty::MoonOffset(Read::read(input)),
            _ => panic!("Unknown object property type: {}", property_type),
        }
    }
}

#[derive(Debug)]
pub struct Brush {
    pub brush_id: i32,
    pub spread: Vec2,
    pub frequency: f32,
    pub grid: BrushGrid,
    pub objects: Vec<BrushObject>,
}

impl Read for Brush {
    fn read(input: &mut impl Iterator<Item = u8>) -> Self {
        Self {
            brush_id: Read::read(input),
            spread: Read::read(input),
            frequency: Read::read(input),
            grid: Read::read(input),
            objects: Read::read(input),
        }
    }
}

#[derive(Debug)]
pub struct BrushObject {
    pub entity_id: i32,
    pub properties: Vec<ObjectProperty>,
    pub weight: f32,
    pub scale: f32,
    pub rotation: f32,
    pub flip_x: bool,
    pub flip_y: bool,
}

impl Read for BrushObject {
    fn read(input: &mut impl Iterator<Item = u8>) -> Self {
        Self {
            entity_id: Read::read(input),
            properties: Read::read(input),
            weight: Read::read(input),
            scale: Read::read(input),
            rotation: Read::read(input),
            flip_x: Read::read(input),
            flip_y: Read::read(input),
        }
    }
}

#[derive(Debug)]
pub struct BrushGrid {
    pub x: i32,
    pub y: i32,
}

impl Read for BrushGrid {
    fn read(input: &mut impl Iterator<Item = u8>) -> Self {
        Self {
            x: Read::read(input),
            y: Read::read(input),
        }
    }
}

#[derive(Debug)]
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
    fn read(input: &mut impl Iterator<Item = u8>) -> Self {
        Self {
            script_id: Read::read(input),
            script_name: Read::read(input),
            is_function: Read::read(input),
            activation_count: Read::read(input),
            condition: Read::read(input),
            activation_list: Read::read(input),
            parameters: Read::read(input),
            variables: Read::read(input),
            actions: Read::read(input),
        }
    }
}

#[derive(Debug)]
pub struct Action {
    pub closed: bool,
    pub wait: bool,
    pub action_type: ActionType,
}

impl Read for Action {
    fn read(input: &mut impl Iterator<Item = u8>) -> Self {
        let action_type = Read::read(input);

        Self {
            closed: Read::read(input),
            wait: Read::read(input),
            action_type: ReadWith::read_with(input, action_type),
        }
    }
}

#[derive(Debug)]
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
}

impl ReadWith for ActionType {
    type With = i32;

    fn read_with(input: &mut impl Iterator<Item = u8>, with: Self::With) -> Self {
        match with {
            0 => ActionType::Repeat {
                actions: Read::read(input),
                count: Read::read(input),
            },
            1 => ActionType::RepeatWhile {
                actions: Read::read(input),
                condition: Read::read(input),
            },
            2 => ActionType::ConditionBlock {
                if_actions: Read::read(input),
                else_actions: Read::read(input),
                condition: Read::read(input),
            },
            3 => ActionType::Wait {
                duration: Read::read(input),
            },
            4 => ActionType::WaitFrames {
                frames: Read::read(input),
            },
            5 => ActionType::Move {
                target_objects: Read::read(input),
                position: Read::read(input),
                global: Read::read(input),
                duration: Read::read(input),
                easing: Read::read(input),
            },
            6 => ActionType::Scale {
                target_objects: Read::read(input),
                scale: Read::read(input),
                duration: Read::read(input),
                easing: Read::read(input),
            },
            7 => ActionType::Rotate {
                target_objects: Read::read(input),
                rotation: Read::read(input),
                shortest_path: Read::read(input),
                global: Read::read(input),
                duration: Read::read(input),
                easing: Read::read(input),
            },
            8 => ActionType::RotateAround {
                target_objects: Read::read(input),
                pivot: Read::read(input),
                rotation: Read::read(input),
                rotate_target: Read::read(input),
                duration: Read::read(input),
                easing: Read::read(input),
            },
            9 => ActionType::SetVariable {
                variable: Read::read(input),
                value: Read::read(input),
            },
            10 => ActionType::ResetVariable {
                variable: Read::read(input),
            },
            11 => ActionType::ResetObject {
                target_objects: Read::read(input),
            },
            12 => ActionType::SetColor {
                target_objects: Read::read(input),
                color: Read::read(input),
                channel: Read::read(input),
                duration: Read::read(input),
                easing: Read::read(input),
            },
            13 => ActionType::SetTransparency {
                target_objects: Read::read(input),
                transparency: Read::read(input),
                channel: Read::read(input),
                duration: Read::read(input),
                easing: Read::read(input),
            },
            14 => ActionType::SetSecondaryColor {
                target_objects: Read::read(input),
                color: Read::read(input),
                duration: Read::read(input),
                easing: Read::read(input),
            },
            15 => ActionType::SetSecondaryTransparency {
                target_objects: Read::read(input),
                transparency: Read::read(input),
                duration: Read::read(input),
                easing: Read::read(input),
            },
            16 => ActionType::SetBorderColor {
                target_objects: Read::read(input),
                color: Read::read(input),
                duration: Read::read(input),
                easing: Read::read(input),
            },
            17 => ActionType::SetBorderTransparency {
                target_objects: Read::read(input),
                transparency: Read::read(input),
                duration: Read::read(input),
                easing: Read::read(input),
            },
            18 => ActionType::SetSprite {
                target_objects: Read::read(input),
                sprite: Read::read(input),
            },
            19 => ActionType::SetText {
                target_objects: Read::read(input),
                text: Read::read(input),
            },
            20 => ActionType::SetEnabled {
                target_objects: Read::read(input),
                enabled: Read::read(input),
            },
            21 => ActionType::Activate {
                target_objects: Read::read(input),
            },
            22 => ActionType::Deactivate {
                target_objects: Read::read(input),
            },
            23 => ActionType::Damage {
                target_objects: Read::read(input),
                damage: Read::read(input),
            },
            24 => ActionType::Kill {
                target_objects: Read::read(input),
            },
            25 => ActionType::GameFinish,
            26 => ActionType::CameraPan {
                position: Read::read(input),
                duration: Read::read(input),
                easing: Read::read(input),
            },
            27 => ActionType::CameraFollowPlayer,
            28 => ActionType::CameraZoom {
                viewport_size: Read::read(input),
                duration: Read::read(input),
                easing: Read::read(input),
            },
            29 => ActionType::CameraZoomReset {
                duration: Read::read(input),
                easing: Read::read(input),
            },
            30 => ActionType::CameraOffset {
                offset: Read::read(input),
                duration: Read::read(input),
                easing: Read::read(input),
            },
            31 => ActionType::CameraOffsetReset {
                duration: Read::read(input),
                easing: Read::read(input),
            },
            32 => ActionType::CameraShake {
                strength: Read::read(input),
                roughness: Read::read(input),
                fade_in: Read::read(input),
                fade_out: Read::read(input),
                duration: Read::read(input),
            },
            33 => ActionType::PlaySound {
                sound: Read::read(input),
                volume: Read::read(input),
                pitch: Read::read(input),
            },
            34 => ActionType::PlayMusic {
                music: Read::read(input),
                volume: Read::read(input),
                pitch: Read::read(input),
            },
            35 => ActionType::SetDirection {
                target_objects: Read::read(input),
                direction: Read::read(input),
            },
            36 => ActionType::SetGravity {
                target_objects: Read::read(input),
                gravity: Read::read(input),
            },
            37 => ActionType::SetVelocity {
                target_objects: Read::read(input),
                velocity: Read::read(input),
            },
            38 => ActionType::SetCinematic {
                enabled: Read::read(input),
            },
            39 => ActionType::SetInputEnabled {
                enabled: Read::read(input),
            },
            40 => ActionType::SetTimerEnabled {
                enabled: Read::read(input),
            },
            41 => ActionType::GameTextShow {
                text: Read::read(input),
                duration: Read::read(input),
            },
            42 => ActionType::DialogueShow {
                text: Read::read(input),
                position: Read::read(input),
                reverse_direction: Read::read(input),
            },
            43 => ActionType::StopScript {
                script: Read::read(input),
            },
            44 => ActionType::TransitionIn {
                type_: Read::read(input),
                color: Read::read(input),
                duration: Read::read(input),
                easing: Read::read(input),
            },
            45 => ActionType::TransitionOut {
                type_: Read::read(input),
                color: Read::read(input),
                duration: Read::read(input),
                easing: Read::read(input),
            },
            46 => ActionType::TimeScale {
                time_scale: Read::read(input),
                duration: Read::read(input),
                easing: Read::read(input),
            },
            47 => ActionType::RunFunction {
                function: Read::read(input),
            },
            48 => ActionType::SetVariableOverTime {
                variable: Read::read(input),
                value: Read::read(input),
                duration: Read::read(input),
                easing: Read::read(input),
            },
            49 => ActionType::RepeatForEachObject {
                target_objects: Read::read(input),
                actions: Read::read(input),
            },
            _ => panic!("Unknown action type: {with}"),
        }
    }
}

#[derive(Debug)]
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
    fn read(input: &mut impl Iterator<Item = u8>) -> Self {
        Self {
            dynamic_type: DynamicType::read(input),
            bool_value: Read::read(input),
            int_value: Read::read(input),
            float_value: Read::read(input),
            string_value: Read::read(input),
            color_value: Read::read(input),
            vector_value: Read::read(input),
            int_list_value: Read::read(input),
            sub_values: Read::read(input),
        }
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
    LayerParameter = 179,
    PointerHeld = 189
);

impl Read for DynamicType {
    fn read(input: &mut impl Iterator<Item = u8>) -> Self {
        Self::try_from(i32::read(input)).unwrap()
    }
}

#[derive(Debug)]
pub struct FunctionCall {
    pub id: i32,
    pub parameters: Vec<CallParameter>,
}

impl Read for FunctionCall {
    fn read(input: &mut impl Iterator<Item = u8>) -> Self {
        Self {
            id: Read::read(input),
            parameters: Read::read(input),
        }
    }
}

#[derive(Debug)]
pub struct CallParameter {
    pub parameter_id: i32,
    pub value: NovaValue,
}

impl Read for CallParameter {
    fn read(input: &mut impl Iterator<Item = u8>) -> Self {
        Self {
            parameter_id: Read::read(input),
            value: Read::read(input),
        }
    }
}

#[derive(Debug)]
pub struct Variable {
    pub variable_id: i32,
    pub name: String,
    pub static_type: StaticType,
    pub initial_value: NovaValue,
}

impl Read for Variable {
    fn read(input: &mut impl Iterator<Item = u8>) -> Self {
        Self {
            variable_id: Read::read(input),
            name: Read::read(input),
            static_type: Read::read(input),
            initial_value: Read::read(input),
        }
    }
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

impl Read for StaticType {
    fn read(input: &mut impl Iterator<Item = u8>) -> Self {
        Self::try_from(i32::read(input)).unwrap()
    }
}

#[derive(Debug)]
pub struct Activator {
    pub activator_type: i32,
    pub parameters: Vec<NovaValue>,
}

impl Read for Activator {
    fn read(input: &mut impl Iterator<Item = u8>) -> Self {
        Self {
            activator_type: Read::read(input),
            parameters: Read::read(input),
        }
    }
}

#[derive(Debug)]
pub struct Parameter {
    pub parameter_id: i32,
    pub name: String,
    pub static_type: StaticType,
    pub default_value: NovaValue,
}

impl Read for Parameter {
    fn read(input: &mut impl Iterator<Item = u8>) -> Self {
        Self {
            parameter_id: Read::read(input),
            name: Read::read(input),
            static_type: Read::read(input),
            default_value: Read::read(input),
        }
    }
}
