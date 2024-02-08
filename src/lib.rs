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
        if Read::read(input) {
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
        let serialization_version = Read::read(input);
        let level_id = Read::read(input);
        let level_version = Read::read(input);
        let level_name = Read::read(input);
        let thumbnail = Read::read(input);
        let creation_date = Read::read(input);
        let update_date = Read::read(input);
        let author_time = Read::read(input);
        let author_lap_times = Read::read(input);
        let silver_medal_time = Read::read(input);
        let gold_medal_time = Read::read(input);
        let laps = Read::read(input);
        let private = Read::read(input);

        let unknown_1 = Read::read(input);

        Self {
            serialization_version,
            level_id,
            level_version,
            level_name,
            thumbnail,
            creation_date,
            update_date,
            author_time,
            author_lap_times,
            silver_medal_time,
            gold_medal_time,
            laps,
            private,
            unknown_1,
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
        let level_id = Read::read(input);
        let level_version = Read::read(input);
        let nova_level = Read::read(input);
        let under_decoration_tiles = Read::read(input);
        let background_decoration_tiles_2 = Read::read(input);
        let terrain_tiles = Read::read(input);
        let floating_zone_tiles = Read::read(input);
        let object_tiles = Read::read(input);
        let foreground_decoration_tiles = Read::read(input);
        let objects = Read::read(input);
        let layers = Read::read(input);
        let prefabs = Read::read(input);
        let brushes = Read::read(input);
        let patterns = Read::read(input);
        let colour_palette = (version >= 17).then(|| Read::read(input));
        let author_time = Read::read(input);
        let author_lap_times = Read::read(input);
        let silver_medal_time = Read::read(input);
        let gold_medal_time = Read::read(input);
        let laps = Read::read(input);
        let center_camera = Read::read(input);
        let scripts = Read::read(input);
        let nova_scripts = Read::read(input);
        let global_variables = Read::read(input);
        let theme = Read::read(input);
        let custom_background_colour = Read::read(input);

        let _unknown1 = Read::read(input);

        let custom_terrain_colour = Read::read(input);

        let _unknown_2 = Read::read(input);

        let custom_terrain_border_colour = Read::read(input);
        let custom_terrain_border_thickness = Read::read(input);
        let custom_terrain_border_corner_radius = Read::read(input);

        let _unknown_3 = Read::read(input);

        let default_music = Read::read(input);
        let music_ids = Read::read(input);
        let allow_direction_change = Read::read(input);
        let disable_replays = Read::read(input);
        let disable_revive_pads = Read::read(input);
        let disable_start_animation = Read::read(input);
        let gravity = Read::read(input);

        Self {
            level_id,
            level_version,
            nova_level,
            under_decoration_tiles,
            background_decoration_tiles_2,
            terrain_tiles,
            floating_zone_tiles,
            object_tiles,
            foreground_decoration_tiles,
            objects,
            layers,
            prefabs,
            brushes,
            patterns,
            colour_palette,
            author_time,
            author_lap_times,
            silver_medal_time,
            gold_medal_time,
            laps,
            center_camera,
            scripts,
            nova_scripts,
            global_variables,
            theme,
            custom_background_colour,
            _unknown1,
            custom_terrain_colour,
            _unknown_2,
            custom_terrain_border_colour,
            custom_terrain_border_thickness,
            custom_terrain_border_corner_radius,
            _unknown_3,
            default_music,
            music_ids,
            allow_direction_change,
            disable_replays,
            disable_revive_pads,
            disable_start_animation,
            gravity,
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
        let pattern_id = Read::read(input);
        let pattern_frames = Read::read(input);

        Self {
            pattern_id,
            pattern_frames,
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
        let prefab_id = Read::read(input);
        let prefab_image_data = Read::read(input);
        let items = Read::read(input);

        Self {
            prefab_id,
            prefab_image_data,
            items,
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
        let layer_id = Read::read(input);
        let layer_name = Read::read(input);
        let selected = Read::read(input);
        let invisible = Read::read(input);
        let locked = Read::read(input);
        let foreground_type = Read::read(input);
        let parallax = Read::read(input);
        let fixed_size = Read::read(input);
        let children = Read::read(input);

        Self {
            layer_id,
            layer_name,
            selected,
            invisible,
            locked,
            foreground_type,
            parallax,
            fixed_size,
            children,
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
        let x = Read::read(input);
        let y = Read::read(input);

        Self { x, y }
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
        let r = Read::read(input);
        let g = Read::read(input);
        let b = Read::read(input);
        let a = Read::read(input);

        Self { r, g, b, a }
    }
}

#[derive(Debug)]
pub struct AuthorReplay {
    pub replay_data: Vec<u8>,
}

impl Read for AuthorReplay {
    fn read(input: &mut impl Iterator<Item = u8>) -> Self {
        let replay_data = Read::read(input);

        Self { replay_data }
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
        let entity_id = Read::read(input);
        let tile_id = Read::read(input);
        let prefab_entity_id = Read::read(input);
        let prefab_id = Read::read(input);
        let position = Read::read(input);
        let scale = Read::read(input);
        let rotation = Read::read(input);
        let tag = Read::read(input);
        let properties = Read::read(input);
        let in_layer = Read::read(input);
        let in_group = Read::read(input);
        let group_members = Read::read(input);

        Self {
            entity_id,
            tile_id,
            prefab_entity_id,
            prefab_id,
            position,
            scale,
            rotation,
            tag,
            properties,
            in_layer,
            in_group,
            group_members,
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
        let brush_id = Read::read(input);
        let spread = Read::read(input);
        let frequency = Read::read(input);
        let grid = Read::read(input);
        let objects = Read::read(input);

        Self {
            brush_id,
            spread,
            frequency,
            grid,
            objects,
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
        let entity_id = Read::read(input);
        let properties = Read::read(input);
        let weight = Read::read(input);
        let scale = Read::read(input);
        let rotation = Read::read(input);
        let flip_x = Read::read(input);
        let flip_y = Read::read(input);

        Self {
            entity_id,
            properties,
            weight,
            scale,
            rotation,
            flip_x,
            flip_y,
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
        let x = Read::read(input);
        let y = Read::read(input);

        Self { x, y }
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
        let script_id = Read::read(input);
        let script_name = Read::read(input);
        let is_function = Read::read(input);
        let activation_count = Read::read(input);
        let condition = Read::read(input);
        let activation_list = Read::read(input);
        let parameters = Read::read(input);
        let variables = Read::read(input);
        let actions = Read::read(input);

        Self {
            script_id,
            script_name,
            is_function,
            activation_count,
            condition,
            activation_list,
            parameters,
            variables,
            actions,
        }
    }
}

#[derive(Debug)]
pub struct Action {
    pub action_type: ActionType,
    pub closed: bool,
    pub wait: bool,
}

impl Read for Action {
    fn read(input: &mut impl Iterator<Item = u8>) -> Self {
        let action_type = Read::read(input);
        let closed = Read::read(input);
        let wait = Read::read(input);
        let action_type = ReadWith::read_with(input, action_type);

        Self {
            action_type,
            closed,
            wait,
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
            0 => {
                let actions = Read::read(input);
                let count = Read::read(input);

                ActionType::Repeat { actions, count }
            }
            1 => {
                let actions = Read::read(input);
                let condition = Read::read(input);

                ActionType::RepeatWhile { actions, condition }
            }
            2 => {
                let if_actions = Read::read(input);
                let else_actions = Read::read(input);
                let condition = Read::read(input);

                ActionType::ConditionBlock {
                    if_actions,
                    else_actions,
                    condition,
                }
            }
            3 => {
                let duration = Read::read(input);

                ActionType::Wait { duration }
            }
            4 => {
                let frames = Read::read(input);

                ActionType::WaitFrames { frames }
            }
            5 => {
                let target_objects = Read::read(input);
                let position = Read::read(input);
                let global = Read::read(input);
                let duration = Read::read(input);
                let easing = Read::read(input);

                ActionType::Move {
                    target_objects,
                    position,
                    global,
                    duration,
                    easing,
                }
            }
            6 => {
                let target_objects = Read::read(input);
                let scale = Read::read(input);
                let duration = Read::read(input);
                let easing = Read::read(input);

                ActionType::Scale {
                    target_objects,
                    scale,
                    duration,
                    easing,
                }
            }
            7 => {
                let target_objects = Read::read(input);
                let rotation = Read::read(input);
                let shortest_path = Read::read(input);
                let global = Read::read(input);
                let duration = Read::read(input);
                let easing = Read::read(input);

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
                let target_objects = Read::read(input);
                let pivot = Read::read(input);
                let rotation = Read::read(input);
                let rotate_target = Read::read(input);
                let duration = Read::read(input);
                let easing = Read::read(input);

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
                let variable = Read::read(input);
                let value = Read::read(input);

                ActionType::SetVariable { variable, value }
            }
            10 => {
                let variable = Read::read(input);

                ActionType::ResetVariable { variable }
            }
            11 => {
                let target_objects = Read::read(input);

                ActionType::ResetObject { target_objects }
            }
            12 => {
                let target_objects = Read::read(input);
                let color = Read::read(input);
                let channel = Read::read(input);
                let duration = Read::read(input);
                let easing = Read::read(input);

                ActionType::SetColor {
                    target_objects,
                    color,
                    channel,
                    duration,
                    easing,
                }
            }
            13 => {
                let target_objects = Read::read(input);
                let transparency = Read::read(input);
                let channel = Read::read(input);
                let duration = Read::read(input);
                let easing = Read::read(input);

                ActionType::SetTransparency {
                    target_objects,
                    transparency,
                    channel,
                    duration,
                    easing,
                }
            }
            14 => {
                let target_objects = Read::read(input);
                let color = Read::read(input);
                let duration = Read::read(input);
                let easing = Read::read(input);

                ActionType::SetSecondaryColor {
                    target_objects,
                    color,
                    duration,
                    easing,
                }
            }
            15 => {
                let target_objects = Read::read(input);
                let transparency = Read::read(input);
                let duration = Read::read(input);
                let easing = Read::read(input);

                ActionType::SetSecondaryTransparency {
                    target_objects,
                    transparency,
                    duration,
                    easing,
                }
            }
            16 => {
                let target_objects = Read::read(input);
                let color = Read::read(input);
                let duration = Read::read(input);
                let easing = Read::read(input);

                ActionType::SetBorderColor {
                    target_objects,
                    color,
                    duration,
                    easing,
                }
            }
            17 => {
                let target_objects = Read::read(input);
                let transparency = Read::read(input);
                let duration = Read::read(input);
                let easing = Read::read(input);

                ActionType::SetBorderTransparency {
                    target_objects,
                    transparency,
                    duration,
                    easing,
                }
            }
            18 => {
                let target_objects = Read::read(input);
                let sprite = Read::read(input);

                ActionType::SetSprite {
                    target_objects,
                    sprite,
                }
            }
            19 => {
                let target_objects = Read::read(input);
                let text = Read::read(input);

                ActionType::SetText {
                    target_objects,
                    text,
                }
            }
            20 => {
                let target_objects = Read::read(input);
                let enabled = Read::read(input);

                ActionType::SetEnabled {
                    target_objects,
                    enabled,
                }
            }
            21 => {
                let target_objects = Read::read(input);

                ActionType::Activate { target_objects }
            }
            22 => {
                let target_objects = Read::read(input);

                ActionType::Deactivate { target_objects }
            }
            23 => {
                let target_objects = Read::read(input);
                let damage = Read::read(input);

                ActionType::Damage {
                    target_objects,
                    damage,
                }
            }
            24 => {
                let target_objects = Read::read(input);

                ActionType::Kill { target_objects }
            }
            25 => ActionType::GameFinish,
            26 => {
                let position = Read::read(input);
                let duration = Read::read(input);
                let easing = Read::read(input);

                ActionType::CameraPan {
                    position,
                    duration,
                    easing,
                }
            }
            27 => ActionType::CameraFollowPlayer,
            28 => {
                let viewport_size = Read::read(input);
                let duration = Read::read(input);
                let easing = Read::read(input);

                ActionType::CameraZoom {
                    viewport_size,
                    duration,
                    easing,
                }
            }
            29 => {
                let duration = Read::read(input);
                let easing = Read::read(input);

                ActionType::CameraZoomReset { duration, easing }
            }
            30 => {
                let offset = Read::read(input);
                let duration = Read::read(input);
                let easing = Read::read(input);

                ActionType::CameraOffset {
                    offset,
                    duration,
                    easing,
                }
            }
            31 => {
                let duration = Read::read(input);
                let easing = Read::read(input);

                ActionType::CameraOffsetReset { duration, easing }
            }
            32 => {
                let strength = Read::read(input);
                let roughness = Read::read(input);
                let fade_in = Read::read(input);
                let fade_out = Read::read(input);
                let duration = Read::read(input);

                ActionType::CameraShake {
                    strength,
                    roughness,
                    fade_in,
                    fade_out,
                    duration,
                }
            }
            33 => {
                let sound = Read::read(input);
                let volume = Read::read(input);
                let pitch = Read::read(input);

                ActionType::PlaySound {
                    sound,
                    volume,
                    pitch,
                }
            }
            34 => {
                let music = Read::read(input);
                let volume = Read::read(input);
                let pitch = Read::read(input);

                ActionType::PlayMusic {
                    music,
                    volume,
                    pitch,
                }
            }
            35 => {
                let target_objects = Read::read(input);
                let direction = Read::read(input);

                ActionType::SetDirection {
                    target_objects,
                    direction,
                }
            }
            36 => {
                let target_objects = Read::read(input);
                let gravity = Read::read(input);

                ActionType::SetGravity {
                    target_objects,
                    gravity,
                }
            }
            37 => {
                let target_objects = Read::read(input);
                let velocity = Read::read(input);

                ActionType::SetVelocity {
                    target_objects,
                    velocity,
                }
            }
            38 => {
                let enabled = Read::read(input);

                ActionType::SetCinematic { enabled }
            }
            39 => {
                let enabled = Read::read(input);

                ActionType::SetInputEnabled { enabled }
            }
            40 => {
                let enabled = Read::read(input);

                ActionType::SetTimerEnabled { enabled }
            }
            41 => {
                let text = Read::read(input);
                let duration = Read::read(input);

                ActionType::GameTextShow { text, duration }
            }
            42 => {
                let text = Read::read(input);
                let position = Read::read(input);
                let reverse_direction = Read::read(input);

                ActionType::DialogueShow {
                    text,
                    position,
                    reverse_direction,
                }
            }
            43 => {
                let script = Read::read(input);

                ActionType::StopScript { script }
            }
            44 => {
                let type_ = Read::read(input);
                let color = Read::read(input);
                let duration = Read::read(input);
                let easing = Read::read(input);

                ActionType::TransitionIn {
                    type_,
                    color,
                    duration,
                    easing,
                }
            }
            45 => {
                let type_ = Read::read(input);
                let color = Read::read(input);
                let duration = Read::read(input);
                let easing = Read::read(input);

                ActionType::TransitionOut {
                    type_,
                    color,
                    duration,
                    easing,
                }
            }
            46 => {
                let time_scale = Read::read(input);
                let duration = Read::read(input);
                let easing = Read::read(input);

                ActionType::TimeScale {
                    time_scale,
                    duration,
                    easing,
                }
            }
            47 => {
                let function = Read::read(input);

                ActionType::RunFunction { function }
            }
            48 => {
                let variable = Read::read(input);
                let value = Read::read(input);
                let duration = Read::read(input);
                let easing = Read::read(input);

                ActionType::SetVariableOverTime {
                    variable,
                    value,
                    duration,
                    easing,
                }
            }
            49 => {
                let target_objects = Read::read(input);
                let actions = Read::read(input);

                ActionType::RepeatForEachObject {
                    target_objects,
                    actions,
                }
            }
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
        let dynamic_type = DynamicType::read(input);

        let bool_value = Read::read(input);
        let int_value = Read::read(input);
        let float_value = Read::read(input);
        let string_value = Read::read(input);
        let color_value = Read::read(input);
        let vector_value = Read::read(input);
        let int_list_value = Read::read(input);
        let sub_values = Read::read(input);

        Self {
            dynamic_type,
            bool_value,
            int_value,
            float_value,
            string_value,
            color_value,
            vector_value,
            int_list_value,
            sub_values,
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
        let value = i32::read(input);

        Self::try_from(value).unwrap()
    }
}

#[derive(Debug)]
pub struct FunctionCall {
    pub id: i32,
    pub parameters: Vec<CallParameter>,
}

impl Read for FunctionCall {
    fn read(input: &mut impl Iterator<Item = u8>) -> Self {
        let id = Read::read(input);
        let parameters = Read::read(input);

        Self { id, parameters }
    }
}

#[derive(Debug)]
pub struct CallParameter {
    pub parameter_id: i32,
    pub value: NovaValue,
}

impl Read for CallParameter {
    fn read(input: &mut impl Iterator<Item = u8>) -> Self {
        let parameter_id = Read::read(input);
        let value = Read::read(input);

        Self {
            parameter_id,
            value,
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
        let variable_id = Read::read(input);
        let name = Read::read(input);
        let static_type = Read::read(input);
        let initial_value = Read::read(input);

        Self {
            variable_id,
            name,
            static_type,
            initial_value,
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
        let value = i32::read(input);

        Self::try_from(value).unwrap()
    }
}

#[derive(Debug)]
pub struct Activator {
    pub activator_type: i32,
    pub parameters: Vec<NovaValue>,
}

impl Read for Activator {
    fn read(input: &mut impl Iterator<Item = u8>) -> Self {
        let activator_type = Read::read(input);
        let parameters = Read::read(input);

        Self {
            activator_type,
            parameters,
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
        let parameter_id = Read::read(input);
        let name = Read::read(input);
        let static_type = Read::read(input);
        let default_value = Read::read(input);

        Self {
            parameter_id,
            name,
            static_type,
            default_value,
        }
    }
}
