use chrono::{DateTime, Utc};

pub trait Read {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error>
    where
        Self: Sized;
}

trait ReadVersioned {
    fn read(input: &mut impl std::io::Read, version: i32) -> Result<Self, Error>
    where
        Self: Sized;
}

trait ReadWith {
    type With;

    fn read_with(input: &mut impl std::io::Read, with: Self::With) -> Result<Self, Error>
    where
        Self: Sized;
}

#[derive(Debug)]
pub enum Error {
    WrongMagic,
    InvalidDynamicType(i32),
    InvalidStaticType(i32),
    Eof,
    InvalidObjectPropertyType(i32),
    InvalidActionType(i32),
}

pub trait Write {
    fn write(&self, output: &mut impl std::io::Write) -> std::io::Result<()>;
}

const SEGMENT_BITS: i32 = 0x7F;
const CONTINUE_BIT: i32 = 0x80;

struct Varint(i32);

impl Read for Varint {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let value = leb128::read::unsigned(input).map_err(|_| Error::Eof)?;

        Ok(Self(value as i32))
    }
}

impl Write for Varint {
    fn write(&self, output: &mut impl std::io::Write) -> std::io::Result<()> {
        let mut value = self.0;

        loop {
            if (value & !SEGMENT_BITS) == 0 {
                output.write_all(&[value as u8])?;
                return Ok(());
            }

            ((value & SEGMENT_BITS) as u8 | CONTINUE_BIT as u8).write(output)?;

            value >>= 7;
        }
    }
}

impl Read for String {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let len = Varint::read(input)?;

        let mut string = String::with_capacity(len.0 as usize);

        for _ in 0..len.0 {
            let c = u8::read(input)? as char;
            string.push(c);
        }

        Ok(string)
    }
}

impl Write for String {
    fn write(&self, output: &mut impl std::io::Write) -> std::io::Result<()> {
        Varint(self.len() as i32).write(output)?;

        for c in self.chars() {
            (c as u8).write(output)?;
        }

        Ok(())
    }
}

impl Write for u32 {
    fn write(&self, output: &mut impl std::io::Write) -> std::io::Result<()> {
        output.write_all(&self.to_le_bytes())
    }
}

impl Read for i32 {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let mut bytes = [0; 4];

        for byte in bytes.iter_mut() {
            *byte = Read::read(input)?;
        }

        Ok(i32::from_le_bytes(bytes))
    }
}

impl Write for i32 {
    fn write(&self, output: &mut impl std::io::Write) -> std::io::Result<()> {
        output.write_all(&self.to_le_bytes())
    }
}

impl Read for i64 {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let mut bytes = [0; 8];

        for byte in bytes.iter_mut() {
            *byte = Read::read(input)?;
        }

        Ok(i64::from_le_bytes(bytes))
    }
}

impl Write for i64 {
    fn write(&self, output: &mut impl std::io::Write) -> std::io::Result<()> {
        output.write_all(&self.to_le_bytes())
    }
}

impl Read for f32 {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let mut bytes = [0; 4];

        for byte in bytes.iter_mut() {
            *byte = Read::read(input)?;
        }

        Ok(f32::from_le_bytes(bytes))
    }
}

impl Write for f32 {
    fn write(&self, output: &mut impl std::io::Write) -> std::io::Result<()> {
        output.write_all(&self.to_le_bytes())
    }
}

impl<T: Read> Read for Vec<T> {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let len = i32::read(input)? as usize;

        let mut vec = Vec::with_capacity(len);

        for _ in 0..len {
            vec.push(Read::read(input)?);
        }

        Ok(vec)
    }
}

impl<T: Write> Write for Vec<T> {
    fn write(&self, output: &mut impl std::io::Write) -> std::io::Result<()> {
        (self.len() as i32).write(output)?;

        for item in self.iter() {
            item.write(output)?;
        }

        Ok(())
    }
}

impl<T: Read + Copy + Default, const LEN: usize> Read for [T; LEN] {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let mut arr = [Default::default(); LEN];

        for item in arr.iter_mut() {
            *item = Read::read(input)?;
        }

        Ok(arr)
    }
}

impl<T: Write, const LEN: usize> Write for [T; LEN] {
    fn write(&self, output: &mut impl std::io::Write) -> std::io::Result<()> {
        for item in self.iter() {
            item.write(output)?;
        }

        Ok(())
    }
}

impl<T: Read> Read for Option<T> {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        if bool::read(input)? {
            Ok(Some(Read::read(input)?))
        } else {
            Ok(None)
        }
    }
}

impl<T: Write> Write for Option<T> {
    fn write(&self, output: &mut impl std::io::Write) -> std::io::Result<()> {
        self.is_some().write(output)?;

        if let Some(value) = self {
            value.write(output)?;
        }

        Ok(())
    }
}

impl Read for bool {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(u8::read(input)? != 0)
    }
}

impl Write for bool {
    fn write(&self, output: &mut impl std::io::Write) -> std::io::Result<()> {
        (*self as u8).write(output)
    }
}

impl Read for u8 {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let mut buf = [0; 1];
        input.read_exact(&mut buf).map_err(|_| Error::Eof)?;
        Ok(buf[0])
    }
}

impl Write for u8 {
    fn write(&self, output: &mut impl std::io::Write) -> std::io::Result<()> {
        output.write_all(&[*self])
    }
}

#[derive(Debug)]
pub struct Exolvl {
    pub local_level: LocalLevel,
    pub level_data: LevelData,
    pub author_replay: AuthorReplay,
}

const MAGIC: &[u8; 4] = b"NYA^";

impl Read for Exolvl {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let magic: [u8; 4] = Read::read(input)?;

        if &magic != MAGIC {
            return Err(Error::WrongMagic);
        }

        let local_level = LocalLevel::read(input)?;
        let level_data = ReadVersioned::read(input, local_level.serialization_version)?;
        let author_replay = Read::read(input)?;

        Ok(Self {
            local_level,
            level_data,
            author_replay,
        })
    }
}

impl Write for Exolvl {
    fn write(&self, output: &mut impl std::io::Write) -> std::io::Result<()> {
        MAGIC.write(output)?;
        self.local_level.write(output)?;
        self.level_data.write(output)?;
        self.author_replay.write(output)
    }
}

#[derive(Debug)]
pub struct LocalLevel {
    pub serialization_version: i32,
    pub level_id: String,
    pub level_version: i32,
    pub level_name: String,
    pub thumbnail: String,
    pub creation_date: MyDateTime,
    pub update_date: MyDateTime,
    pub author_time: i64,
    pub author_lap_times: Vec<i64>,
    pub silver_medal_time: i64,
    pub gold_medal_time: i64,
    pub laps: i32,
    pub private: bool,

    unknown_1: u8,
}

impl Read for LocalLevel {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            serialization_version: Read::read(input)?,
            level_id: Read::read(input)?,
            level_version: Read::read(input)?,
            level_name: Read::read(input)?,
            thumbnail: Read::read(input)?,
            creation_date: Read::read(input)?,
            update_date: Read::read(input)?,
            author_time: Read::read(input)?,
            author_lap_times: Read::read(input)?,
            silver_medal_time: Read::read(input)?,
            gold_medal_time: Read::read(input)?,
            laps: Read::read(input)?,
            private: Read::read(input)?,
            unknown_1: Read::read(input)?,
        })
    }
}

impl Write for LocalLevel {
    fn write(&self, output: &mut impl std::io::Write) -> std::io::Result<()> {
        self.serialization_version.write(output)?;
        self.level_id.write(output)?;
        self.level_version.write(output)?;
        self.level_name.write(output)?;
        self.thumbnail.write(output)?;
        self.creation_date.write(output)?;
        self.update_date.write(output)?;
        self.author_time.write(output)?;
        self.author_lap_times.write(output)?;
        self.silver_medal_time.write(output)?;
        self.gold_medal_time.write(output)?;
        self.laps.write(output)?;
        self.private.write(output)?;
        self.unknown_1.write(output)
    }
}

#[derive(Debug)]
pub struct MyDateTime(i64);

impl MyDateTime {
    const TICKS_TO_SECONDS: i64 = 10_000_000;
    const EPOCH_DIFFERENCE: i64 = 62_135_596_800;
}

impl From<&DateTime<Utc>> for MyDateTime {
    fn from(datetime: &DateTime<Utc>) -> Self {
        let ticks = (datetime.timestamp() + Self::EPOCH_DIFFERENCE) * Self::TICKS_TO_SECONDS;

        Self(ticks)
    }
}

impl From<MyDateTime> for DateTime<Utc> {
    fn from(my_datetime: MyDateTime) -> Self {
        let masked_ticks = my_datetime.0;
        let seconds = masked_ticks / MyDateTime::TICKS_TO_SECONDS - MyDateTime::EPOCH_DIFFERENCE;

        DateTime::<Utc>::from_timestamp(seconds, 0).unwrap()
    }
}

impl Read for MyDateTime {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(MyDateTime(i64::read(input)?))
    }
}

impl Write for MyDateTime {
    fn write(&self, output: &mut impl std::io::Write) -> std::io::Result<()> {
        self.0.write(output)
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
    fn read(input: &mut impl std::io::Read, version: i32) -> Result<Self, Error> {
        Ok(Self {
            level_id: Read::read(input)?,
            level_version: Read::read(input)?,
            nova_level: Read::read(input)?,
            under_decoration_tiles: Read::read(input)?,
            background_decoration_tiles_2: Read::read(input)?,
            terrain_tiles: Read::read(input)?,
            floating_zone_tiles: Read::read(input)?,
            object_tiles: Read::read(input)?,
            foreground_decoration_tiles: Read::read(input)?,
            objects: Read::read(input)?,
            layers: Read::read(input)?,
            prefabs: Read::read(input)?,
            brushes: Read::read(input)?,
            patterns: Read::read(input)?,
            colour_palette: if version >= 17 {
                Some(Read::read(input)?)
            } else {
                None
            },
            author_time: Read::read(input)?,
            author_lap_times: Read::read(input)?,
            silver_medal_time: Read::read(input)?,
            gold_medal_time: Read::read(input)?,
            laps: Read::read(input)?,
            center_camera: Read::read(input)?,
            scripts: Read::read(input)?,
            nova_scripts: Read::read(input)?,
            global_variables: Read::read(input)?,
            theme: Read::read(input)?,
            custom_background_colour: Read::read(input)?,
            _unknown1: Read::read(input)?,
            custom_terrain_colour: Read::read(input)?,
            _unknown_2: Read::read(input)?,
            custom_terrain_border_colour: Read::read(input)?,
            custom_terrain_border_thickness: Read::read(input)?,
            custom_terrain_border_corner_radius: Read::read(input)?,
            _unknown_3: Read::read(input)?,
            default_music: Read::read(input)?,
            music_ids: Read::read(input)?,
            allow_direction_change: Read::read(input)?,
            disable_replays: Read::read(input)?,
            disable_revive_pads: Read::read(input)?,
            disable_start_animation: Read::read(input)?,
            gravity: Read::read(input)?,
        })
    }
}

impl Write for LevelData {
    fn write(&self, output: &mut impl std::io::Write) -> std::io::Result<()> {
        self.level_id.write(output)?;
        self.level_version.write(output)?;
        self.nova_level.write(output)?;
        self.under_decoration_tiles.write(output)?;
        self.background_decoration_tiles_2.write(output)?;
        self.terrain_tiles.write(output)?;
        self.floating_zone_tiles.write(output)?;
        self.object_tiles.write(output)?;
        self.foreground_decoration_tiles.write(output)?;
        self.objects.write(output)?;
        self.layers.write(output)?;
        self.prefabs.write(output)?;
        self.brushes.write(output)?;
        self.patterns.write(output)?;
        if let Some(colour_palette) = &self.colour_palette {
            colour_palette.write(output)?;
        }
        self.author_time.write(output)?;
        self.author_lap_times.write(output)?;
        self.silver_medal_time.write(output)?;
        self.gold_medal_time.write(output)?;
        self.laps.write(output)?;
        self.center_camera.write(output)?;
        self.scripts.write(output)?;
        self.nova_scripts.write(output)?;
        self.global_variables.write(output)?;
        self.theme.write(output)?;
        self.custom_background_colour.write(output)?;
        self._unknown1.write(output)?;
        self.custom_terrain_colour.write(output)?;
        self._unknown_2.write(output)?;
        self.custom_terrain_border_colour.write(output)?;
        self.custom_terrain_border_thickness.write(output)?;
        self.custom_terrain_border_corner_radius.write(output)?;
        self._unknown_3.write(output)?;
        self.default_music.write(output)?;
        self.music_ids.write(output)?;
        self.allow_direction_change.write(output)?;
        self.disable_replays.write(output)?;
        self.disable_revive_pads.write(output)?;
        self.disable_start_animation.write(output)?;
        self.gravity.write(output)
    }
}

#[derive(Debug)]
pub struct Pattern {
    pub pattern_id: i32,
    pub pattern_frames: Vec<Image>,
}

impl Read for Pattern {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            pattern_id: Read::read(input)?,
            pattern_frames: Read::read(input)?,
        })
    }
}

impl Write for Pattern {
    fn write(&self, output: &mut impl std::io::Write) -> std::io::Result<()> {
        self.pattern_id.write(output)?;
        self.pattern_frames.write(output)
    }
}

#[derive(Debug)]
pub struct Prefab {
    pub prefab_id: i32,
    pub prefab_image_data: Image,
    pub items: Vec<Object>,
}

impl Read for Prefab {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            prefab_id: Read::read(input)?,
            prefab_image_data: Read::read(input)?,
            items: Read::read(input)?,
        })
    }
}

impl Write for Prefab {
    fn write(&self, output: &mut impl std::io::Write) -> std::io::Result<()> {
        self.prefab_id.write(output)?;
        self.prefab_image_data.write(output)?;
        self.items.write(output)
    }
}

#[derive(Debug)]
pub struct Image(pub Vec<u8>);

impl Read for Image {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let data = Read::read(input)?;

        Ok(Self(data))
    }
}

impl Write for Image {
    fn write(&self, output: &mut impl std::io::Write) -> std::io::Result<()> {
        self.0.write(output)
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
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            layer_id: Read::read(input)?,
            layer_name: Read::read(input)?,
            selected: Read::read(input)?,
            invisible: Read::read(input)?,
            locked: Read::read(input)?,
            foreground_type: Read::read(input)?,
            parallax: Read::read(input)?,
            fixed_size: Read::read(input)?,
            children: Read::read(input)?,
        })
    }
}

impl Write for Layer {
    fn write(&self, output: &mut impl std::io::Write) -> std::io::Result<()> {
        self.layer_id.write(output)?;
        self.layer_name.write(output)?;
        self.selected.write(output)?;
        self.invisible.write(output)?;
        self.locked.write(output)?;
        self.foreground_type.write(output)?;
        self.parallax.write(output)?;
        self.fixed_size.write(output)?;
        self.children.write(output)
    }
}

#[derive(Debug)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Read for Vec2 {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            x: Read::read(input)?,
            y: Read::read(input)?,
        })
    }
}

impl Write for Vec2 {
    fn write(&self, output: &mut impl std::io::Write) -> std::io::Result<()> {
        self.x.write(output)?;
        self.y.write(output)
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
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            r: Read::read(input)?,
            g: Read::read(input)?,
            b: Read::read(input)?,
            a: Read::read(input)?,
        })
    }
}

impl Write for Colour {
    fn write(&self, output: &mut impl std::io::Write) -> std::io::Result<()> {
        self.r.write(output)?;
        self.g.write(output)?;
        self.b.write(output)?;
        self.a.write(output)
    }
}

#[derive(Debug)]
pub struct AuthorReplay(pub Vec<u8>);

impl Read for AuthorReplay {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(Self(Read::read(input)?))
    }
}

impl Write for AuthorReplay {
    fn write(&self, output: &mut impl std::io::Write) -> std::io::Result<()> {
        self.0.write(output)
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
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            entity_id: Read::read(input)?,
            tile_id: Read::read(input)?,
            prefab_entity_id: Read::read(input)?,
            prefab_id: Read::read(input)?,
            position: Read::read(input)?,
            scale: Read::read(input)?,
            rotation: Read::read(input)?,
            tag: Read::read(input)?,
            properties: Read::read(input)?,
            in_layer: Read::read(input)?,
            in_group: Read::read(input)?,
            group_members: Read::read(input)?,
        })
    }
}

impl Write for Object {
    fn write(&self, output: &mut impl std::io::Write) -> std::io::Result<()> {
        self.entity_id.write(output)?;
        self.tile_id.write(output)?;
        self.prefab_entity_id.write(output)?;
        self.prefab_id.write(output)?;
        self.position.write(output)?;
        self.scale.write(output)?;
        self.rotation.write(output)?;
        self.tag.write(output)?;
        self.properties.write(output)?;
        self.in_layer.write(output)?;
        self.in_group.write(output)?;
        self.group_members.write(output)
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
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let property_type = Read::read(input)?;

        Ok(match property_type {
            0 => ObjectProperty::Colour(Read::read(input)?),
            1 => ObjectProperty::Resolution(Read::read(input)?),
            2 => ObjectProperty::FillMode(Read::read(input)?),
            3 => ObjectProperty::SecondaryColour(Read::read(input)?),
            4 => ObjectProperty::Thickness(Read::read(input)?),
            5 => ObjectProperty::TotalAngle(Read::read(input)?),
            6 => ObjectProperty::Corners(Read::read(input)?),
            7 => ObjectProperty::Blending(Read::read(input)?),
            8 => ObjectProperty::GridOffset(Read::read(input)?),
            9 => ObjectProperty::CornerRadius(Read::read(input)?),
            10 => ObjectProperty::Width(Read::read(input)?),
            11 => ObjectProperty::Height(Read::read(input)?),
            12 => ObjectProperty::BorderColour(Read::read(input)?),
            13 => ObjectProperty::BorderThickness(Read::read(input)?),
            14 => ObjectProperty::PhysicsType(Read::read(input)?),
            15 => ObjectProperty::Friction(Read::read(input)?),
            16 => ObjectProperty::TerrainCorners(Read::read(input)?),
            17 => ObjectProperty::Direction(Read::read(input)?),
            18 => ObjectProperty::Impulse(Read::read(input)?),
            19 => ObjectProperty::Killer(Read::read(input)?),
            20 => ObjectProperty::RoundReflexAngles(Read::read(input)?),
            21 => ObjectProperty::RoundCollider(Read::read(input)?),
            22 => ObjectProperty::Radius(Read::read(input)?),
            23 => ObjectProperty::Size(Read::read(input)?),
            24 => ObjectProperty::ReverseDirection(Read::read(input)?),
            25 => ObjectProperty::CollisionDetector(Read::read(input)?),
            26 => ObjectProperty::Pattern(Read::read(input)?),
            27 => ObjectProperty::PatternTiling(Read::read(input)?),
            28 => ObjectProperty::PatternOffset(Read::read(input)?),
            35 => ObjectProperty::Sprite(Read::read(input)?),
            36 => ObjectProperty::Trigger(Read::read(input)?),
            37 => ObjectProperty::Health(Read::read(input)?),
            38 => ObjectProperty::DamageFromJump(Read::read(input)?),
            39 => ObjectProperty::DamageFromDash(Read::read(input)?),
            40 => ObjectProperty::ReverseDirOnDamage(Read::read(input)?),
            41 => ObjectProperty::Floating(Read::read(input)?),
            43 => ObjectProperty::FlipX(Read::read(input)?),
            44 => ObjectProperty::FlipY(Read::read(input)?),
            45 => ObjectProperty::Text(Read::read(input)?),
            46 => ObjectProperty::FontSize(Read::read(input)?),
            47 => ObjectProperty::EditorColour(Read::read(input)?),
            83 => ObjectProperty::MoonInnerRadius(Read::read(input)?),
            84 => ObjectProperty::MoonOffset(Read::read(input)?),
            n => return Err(Error::InvalidObjectPropertyType(n)),
        })
    }
}

impl Write for ObjectProperty {
    fn write(&self, output: &mut impl std::io::Write) -> std::io::Result<()> {
        match self {
            ObjectProperty::Colour(value) => {
                0.write(output)?;
                value.write(output)
            }
            ObjectProperty::Resolution(value) => {
                1.write(output)?;
                value.write(output)
            }
            ObjectProperty::FillMode(value) => {
                2.write(output)?;
                value.write(output)
            }
            ObjectProperty::SecondaryColour(value) => {
                3.write(output)?;
                value.write(output)
            }
            ObjectProperty::Thickness(value) => {
                4.write(output)?;
                value.write(output)
            }
            ObjectProperty::TotalAngle(value) => {
                5.write(output)?;
                value.write(output)
            }
            ObjectProperty::Corners(value) => {
                6.write(output)?;
                value.write(output)
            }
            ObjectProperty::Blending(value) => {
                7.write(output)?;
                value.write(output)
            }
            ObjectProperty::GridOffset(value) => {
                8.write(output)?;
                value.write(output)
            }
            ObjectProperty::CornerRadius(value) => {
                9.write(output)?;
                value.write(output)
            }
            ObjectProperty::Width(value) => {
                10.write(output)?;
                value.write(output)
            }
            ObjectProperty::Height(value) => {
                11.write(output)?;
                value.write(output)
            }
            ObjectProperty::BorderColour(value) => {
                12.write(output)?;
                value.write(output)
            }
            ObjectProperty::BorderThickness(value) => {
                13.write(output)?;
                value.write(output)
            }
            ObjectProperty::PhysicsType(value) => {
                14.write(output)?;
                value.write(output)
            }
            ObjectProperty::Friction(value) => {
                15.write(output)?;
                value.write(output)
            }
            ObjectProperty::TerrainCorners(value) => {
                16.write(output)?;
                value.write(output)
            }
            ObjectProperty::Direction(value) => {
                17.write(output)?;
                value.write(output)
            }
            ObjectProperty::Impulse(value) => {
                18.write(output)?;
                value.write(output)
            }
            ObjectProperty::Killer(value) => {
                19.write(output)?;
                value.write(output)
            }
            ObjectProperty::RoundReflexAngles(value) => {
                20.write(output)?;
                value.write(output)
            }
            ObjectProperty::RoundCollider(value) => {
                21.write(output)?;
                value.write(output)
            }
            ObjectProperty::Radius(value) => {
                22.write(output)?;
                value.write(output)
            }
            ObjectProperty::Size(value) => {
                23.write(output)?;
                value.write(output)
            }
            ObjectProperty::ReverseDirection(value) => {
                24.write(output)?;
                value.write(output)
            }
            ObjectProperty::CollisionDetector(value) => {
                25.write(output)?;
                value.write(output)
            }
            ObjectProperty::Pattern(value) => {
                26.write(output)?;
                value.write(output)
            }
            ObjectProperty::PatternTiling(value) => {
                27.write(output)?;
                value.write(output)
            }
            ObjectProperty::PatternOffset(value) => {
                28.write(output)?;
                value.write(output)
            }
            ObjectProperty::Sprite(value) => {
                35.write(output)?;
                value.write(output)
            }
            ObjectProperty::Trigger(value) => {
                36.write(output)?;
                value.write(output)
            }
            ObjectProperty::Health(value) => {
                37.write(output)?;
                value.write(output)
            }
            ObjectProperty::DamageFromJump(value) => {
                38.write(output)?;
                value.write(output)
            }
            ObjectProperty::DamageFromDash(value) => {
                39.write(output)?;
                value.write(output)
            }
            ObjectProperty::ReverseDirOnDamage(value) => {
                40.write(output)?;
                value.write(output)
            }
            ObjectProperty::Floating(value) => {
                41.write(output)?;
                value.write(output)
            }
            ObjectProperty::FlipX(value) => {
                43.write(output)?;
                value.write(output)
            }
            ObjectProperty::FlipY(value) => {
                44.write(output)?;
                value.write(output)
            }
            ObjectProperty::Text(value) => {
                45.write(output)?;
                value.write(output)
            }
            ObjectProperty::FontSize(value) => {
                46.write(output)?;
                value.write(output)
            }
            ObjectProperty::EditorColour(value) => {
                47.write(output)?;
                value.write(output)
            }
            ObjectProperty::MoonInnerRadius(value) => {
                83.write(output)?;
                value.write(output)
            }
            ObjectProperty::MoonOffset(value) => {
                84.write(output)?;
                value.write(output)
            }
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
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            brush_id: Read::read(input)?,
            spread: Read::read(input)?,
            frequency: Read::read(input)?,
            grid: Read::read(input)?,
            objects: Read::read(input)?,
        })
    }
}

impl Write for Brush {
    fn write(&self, output: &mut impl std::io::Write) -> std::io::Result<()> {
        self.brush_id.write(output)?;
        self.spread.write(output)?;
        self.frequency.write(output)?;
        self.grid.write(output)?;
        self.objects.write(output)
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
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            entity_id: Read::read(input)?,
            properties: Read::read(input)?,
            weight: Read::read(input)?,
            scale: Read::read(input)?,
            rotation: Read::read(input)?,
            flip_x: Read::read(input)?,
            flip_y: Read::read(input)?,
        })
    }
}

impl Write for BrushObject {
    fn write(&self, output: &mut impl std::io::Write) -> std::io::Result<()> {
        self.entity_id.write(output)?;
        self.properties.write(output)?;
        self.weight.write(output)?;
        self.scale.write(output)?;
        self.rotation.write(output)?;
        self.flip_x.write(output)?;
        self.flip_y.write(output)
    }
}

#[derive(Debug)]
pub struct BrushGrid {
    pub x: i32,
    pub y: i32,
}

impl Read for BrushGrid {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            x: Read::read(input)?,
            y: Read::read(input)?,
        })
    }
}

impl Write for BrushGrid {
    fn write(&self, output: &mut impl std::io::Write) -> std::io::Result<()> {
        self.x.write(output)?;
        self.y.write(output)
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
    fn write(&self, output: &mut impl std::io::Write) -> std::io::Result<()> {
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

#[derive(Debug)]
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
            action_type: ReadWith::read_with(input, action_type)?,
        })
    }
}

impl Write for Action {
    fn write(&self, output: &mut impl std::io::Write) -> std::io::Result<()> {
        let action_type = i32::from(&self.action_type);

        action_type.write(output)?;
        self.closed.write(output)?;
        self.wait.write(output)?;
        self.action_type.write(output)
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

impl From<&ActionType> for i32 {
    fn from(action_type: &ActionType) -> i32 {
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
        }
    }
}

impl ReadWith for ActionType {
    type With = i32;

    fn read_with(input: &mut impl std::io::Read, with: Self::With) -> Result<Self, Error> {
        Ok(match with {
            0 => ActionType::Repeat {
                actions: Read::read(input)?,
                count: Read::read(input)?,
            },
            1 => ActionType::RepeatWhile {
                actions: Read::read(input)?,
                condition: Read::read(input)?,
            },
            2 => ActionType::ConditionBlock {
                if_actions: Read::read(input)?,
                else_actions: Read::read(input)?,
                condition: Read::read(input)?,
            },
            3 => ActionType::Wait {
                duration: Read::read(input)?,
            },
            4 => ActionType::WaitFrames {
                frames: Read::read(input)?,
            },
            5 => ActionType::Move {
                target_objects: Read::read(input)?,
                position: Read::read(input)?,
                global: Read::read(input)?,
                duration: Read::read(input)?,
                easing: Read::read(input)?,
            },
            6 => ActionType::Scale {
                target_objects: Read::read(input)?,
                scale: Read::read(input)?,
                duration: Read::read(input)?,
                easing: Read::read(input)?,
            },
            7 => ActionType::Rotate {
                target_objects: Read::read(input)?,
                rotation: Read::read(input)?,
                shortest_path: Read::read(input)?,
                global: Read::read(input)?,
                duration: Read::read(input)?,
                easing: Read::read(input)?,
            },
            8 => ActionType::RotateAround {
                target_objects: Read::read(input)?,
                pivot: Read::read(input)?,
                rotation: Read::read(input)?,
                rotate_target: Read::read(input)?,
                duration: Read::read(input)?,
                easing: Read::read(input)?,
            },
            9 => ActionType::SetVariable {
                variable: Read::read(input)?,
                value: Read::read(input)?,
            },
            10 => ActionType::ResetVariable {
                variable: Read::read(input)?,
            },
            11 => ActionType::ResetObject {
                target_objects: Read::read(input)?,
            },
            12 => ActionType::SetColor {
                target_objects: Read::read(input)?,
                color: Read::read(input)?,
                channel: Read::read(input)?,
                duration: Read::read(input)?,
                easing: Read::read(input)?,
            },
            13 => ActionType::SetTransparency {
                target_objects: Read::read(input)?,
                transparency: Read::read(input)?,
                channel: Read::read(input)?,
                duration: Read::read(input)?,
                easing: Read::read(input)?,
            },
            14 => ActionType::SetSecondaryColor {
                target_objects: Read::read(input)?,
                color: Read::read(input)?,
                duration: Read::read(input)?,
                easing: Read::read(input)?,
            },
            15 => ActionType::SetSecondaryTransparency {
                target_objects: Read::read(input)?,
                transparency: Read::read(input)?,
                duration: Read::read(input)?,
                easing: Read::read(input)?,
            },
            16 => ActionType::SetBorderColor {
                target_objects: Read::read(input)?,
                color: Read::read(input)?,
                duration: Read::read(input)?,
                easing: Read::read(input)?,
            },
            17 => ActionType::SetBorderTransparency {
                target_objects: Read::read(input)?,
                transparency: Read::read(input)?,
                duration: Read::read(input)?,
                easing: Read::read(input)?,
            },
            18 => ActionType::SetSprite {
                target_objects: Read::read(input)?,
                sprite: Read::read(input)?,
            },
            19 => ActionType::SetText {
                target_objects: Read::read(input)?,
                text: Read::read(input)?,
            },
            20 => ActionType::SetEnabled {
                target_objects: Read::read(input)?,
                enabled: Read::read(input)?,
            },
            21 => ActionType::Activate {
                target_objects: Read::read(input)?,
            },
            22 => ActionType::Deactivate {
                target_objects: Read::read(input)?,
            },
            23 => ActionType::Damage {
                target_objects: Read::read(input)?,
                damage: Read::read(input)?,
            },
            24 => ActionType::Kill {
                target_objects: Read::read(input)?,
            },
            25 => ActionType::GameFinish,
            26 => ActionType::CameraPan {
                position: Read::read(input)?,
                duration: Read::read(input)?,
                easing: Read::read(input)?,
            },
            27 => ActionType::CameraFollowPlayer,
            28 => ActionType::CameraZoom {
                viewport_size: Read::read(input)?,
                duration: Read::read(input)?,
                easing: Read::read(input)?,
            },
            29 => ActionType::CameraZoomReset {
                duration: Read::read(input)?,
                easing: Read::read(input)?,
            },
            30 => ActionType::CameraOffset {
                offset: Read::read(input)?,
                duration: Read::read(input)?,
                easing: Read::read(input)?,
            },
            31 => ActionType::CameraOffsetReset {
                duration: Read::read(input)?,
                easing: Read::read(input)?,
            },
            32 => ActionType::CameraShake {
                strength: Read::read(input)?,
                roughness: Read::read(input)?,
                fade_in: Read::read(input)?,
                fade_out: Read::read(input)?,
                duration: Read::read(input)?,
            },
            33 => ActionType::PlaySound {
                sound: Read::read(input)?,
                volume: Read::read(input)?,
                pitch: Read::read(input)?,
            },
            34 => ActionType::PlayMusic {
                music: Read::read(input)?,
                volume: Read::read(input)?,
                pitch: Read::read(input)?,
            },
            35 => ActionType::SetDirection {
                target_objects: Read::read(input)?,
                direction: Read::read(input)?,
            },
            36 => ActionType::SetGravity {
                target_objects: Read::read(input)?,
                gravity: Read::read(input)?,
            },
            37 => ActionType::SetVelocity {
                target_objects: Read::read(input)?,
                velocity: Read::read(input)?,
            },
            38 => ActionType::SetCinematic {
                enabled: Read::read(input)?,
            },
            39 => ActionType::SetInputEnabled {
                enabled: Read::read(input)?,
            },
            40 => ActionType::SetTimerEnabled {
                enabled: Read::read(input)?,
            },
            41 => ActionType::GameTextShow {
                text: Read::read(input)?,
                duration: Read::read(input)?,
            },
            42 => ActionType::DialogueShow {
                text: Read::read(input)?,
                position: Read::read(input)?,
                reverse_direction: Read::read(input)?,
            },
            43 => ActionType::StopScript {
                script: Read::read(input)?,
            },
            44 => ActionType::TransitionIn {
                type_: Read::read(input)?,
                color: Read::read(input)?,
                duration: Read::read(input)?,
                easing: Read::read(input)?,
            },
            45 => ActionType::TransitionOut {
                type_: Read::read(input)?,
                color: Read::read(input)?,
                duration: Read::read(input)?,
                easing: Read::read(input)?,
            },
            46 => ActionType::TimeScale {
                time_scale: Read::read(input)?,
                duration: Read::read(input)?,
                easing: Read::read(input)?,
            },
            47 => ActionType::RunFunction {
                function: Read::read(input)?,
            },
            48 => ActionType::SetVariableOverTime {
                variable: Read::read(input)?,
                value: Read::read(input)?,
                duration: Read::read(input)?,
                easing: Read::read(input)?,
            },
            49 => ActionType::RepeatForEachObject {
                target_objects: Read::read(input)?,
                actions: Read::read(input)?,
            },
            n => return Err(Error::InvalidActionType(n)),
        })
    }
}

impl Write for ActionType {
    fn write(&self, output: &mut impl std::io::Write) -> std::io::Result<()> {
        match self {
            ActionType::Repeat { actions, count } => {
                actions.write(output)?;
                count.write(output)
            }
            ActionType::RepeatWhile { actions, condition } => {
                actions.write(output)?;
                condition.write(output)
            }
            ActionType::ConditionBlock {
                if_actions,
                else_actions,
                condition,
            } => {
                if_actions.write(output)?;
                else_actions.write(output)?;
                condition.write(output)
            }
            ActionType::Wait { duration } => duration.write(output),
            ActionType::WaitFrames { frames } => frames.write(output),
            ActionType::Move {
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
            ActionType::Scale {
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
            ActionType::Rotate {
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
            ActionType::RotateAround {
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
            ActionType::SetVariable { variable, value } => {
                variable.write(output)?;
                value.write(output)
            }
            ActionType::ResetVariable { variable } => variable.write(output),
            ActionType::ResetObject { target_objects } => target_objects.write(output),
            ActionType::SetColor {
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
            ActionType::SetTransparency {
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
            ActionType::SetSecondaryColor {
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
            ActionType::SetSecondaryTransparency {
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
            ActionType::SetBorderColor {
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
            ActionType::SetBorderTransparency {
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
            ActionType::SetSprite {
                target_objects,
                sprite,
            } => {
                target_objects.write(output)?;
                sprite.write(output)
            }
            ActionType::SetText {
                target_objects,
                text,
            } => {
                target_objects.write(output)?;
                text.write(output)
            }
            ActionType::SetEnabled {
                target_objects,
                enabled,
            } => {
                target_objects.write(output)?;
                enabled.write(output)
            }
            ActionType::Activate { target_objects } => target_objects.write(output),
            ActionType::Deactivate { target_objects } => target_objects.write(output),
            ActionType::Damage {
                target_objects,
                damage,
            } => {
                target_objects.write(output)?;
                damage.write(output)
            }
            ActionType::Kill { target_objects } => target_objects.write(output),
            ActionType::GameFinish => Ok(()),
            ActionType::CameraPan {
                position,
                duration,
                easing,
            } => {
                position.write(output)?;
                duration.write(output)?;
                easing.write(output)
            }
            ActionType::CameraFollowPlayer => Ok(()),
            ActionType::CameraZoom {
                viewport_size,
                duration,
                easing,
            } => {
                viewport_size.write(output)?;
                duration.write(output)?;
                easing.write(output)
            }
            ActionType::CameraZoomReset { duration, easing } => {
                duration.write(output)?;
                easing.write(output)
            }
            ActionType::CameraOffset {
                offset,
                duration,
                easing,
            } => {
                offset.write(output)?;
                duration.write(output)?;
                easing.write(output)
            }
            ActionType::CameraOffsetReset { duration, easing } => {
                duration.write(output)?;
                easing.write(output)
            }
            ActionType::CameraShake {
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
            ActionType::PlaySound {
                sound,
                volume,
                pitch,
            } => {
                sound.write(output)?;
                volume.write(output)?;
                pitch.write(output)
            }
            ActionType::PlayMusic {
                music,
                volume,
                pitch,
            } => {
                music.write(output)?;
                volume.write(output)?;
                pitch.write(output)
            }
            ActionType::SetDirection {
                target_objects,
                direction,
            } => {
                target_objects.write(output)?;
                direction.write(output)
            }
            ActionType::SetGravity {
                target_objects,
                gravity,
            } => {
                target_objects.write(output)?;
                gravity.write(output)
            }
            ActionType::SetVelocity {
                target_objects,
                velocity,
            } => {
                target_objects.write(output)?;
                velocity.write(output)
            }
            ActionType::SetCinematic { enabled } => enabled.write(output),
            ActionType::SetInputEnabled { enabled } => enabled.write(output),
            ActionType::SetTimerEnabled { enabled } => enabled.write(output),
            ActionType::GameTextShow { text, duration } => {
                text.write(output)?;
                duration.write(output)
            }
            ActionType::DialogueShow {
                text,
                position,
                reverse_direction,
            } => {
                text.write(output)?;
                position.write(output)?;
                reverse_direction.write(output)
            }
            ActionType::StopScript { script } => script.write(output),
            ActionType::TransitionIn {
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
            ActionType::TransitionOut {
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
            ActionType::TimeScale {
                time_scale,
                duration,
                easing,
            } => {
                time_scale.write(output)?;
                duration.write(output)?;
                easing.write(output)
            }
            ActionType::RunFunction { function } => function.write(output),
            ActionType::SetVariableOverTime {
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
            ActionType::RepeatForEachObject {
                target_objects,
                actions,
            } => {
                target_objects.write(output)?;
                actions.write(output)
            }
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
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            dynamic_type: DynamicType::read(input)?,
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
    fn write(&self, output: &mut impl std::io::Write) -> std::io::Result<()> {
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
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let value = i32::read(input)?;

        Self::try_from(value).map_err(|_| Error::InvalidDynamicType(value))
    }
}

impl Write for DynamicType {
    fn write(&self, output: &mut impl std::io::Write) -> std::io::Result<()> {
        i32::from(self).write(output)
    }
}

#[derive(Debug)]
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
    fn write(&self, output: &mut impl std::io::Write) -> std::io::Result<()> {
        self.id.write(output)?;
        self.parameters.write(output)
    }
}

#[derive(Debug)]
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
    fn write(&self, output: &mut impl std::io::Write) -> std::io::Result<()> {
        self.parameter_id.write(output)?;
        self.value.write(output)
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
    fn write(&self, output: &mut impl std::io::Write) -> std::io::Result<()> {
        self.variable_id.write(output)?;
        self.name.write(output)?;
        self.static_type.write(output)?;
        self.initial_value.write(output)
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
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let value = i32::read(input)?;

        Self::try_from(value).map_err(|_| Error::InvalidStaticType(value))
    }
}

impl Write for StaticType {
    fn write(&self, output: &mut impl std::io::Write) -> std::io::Result<()> {
        i32::from(self).write(output)
    }
}

#[derive(Debug)]
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
    fn write(&self, output: &mut impl std::io::Write) -> std::io::Result<()> {
        self.activator_type.write(output)?;
        self.parameters.write(output)
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
    fn write(&self, output: &mut impl std::io::Write) -> std::io::Result<()> {
        self.parameter_id.write(output)?;
        self.name.write(output)?;
        self.static_type.write(output)?;
        self.default_value.write(output)
    }
}
