#![forbid(unsafe_code)]

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

pub trait Read: private::Sealed {
    /// Reads a value from the given input.
    ///
    /// The input is any type that implements [`std::io::Read`]. This can also be a mutable reference to a type that implements [`std::io::Read`].
    ///
    /// This trait is sealed and cannot be implemented for types outside of this crate.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying reader returns an error.
    fn read(input: impl std::io::Read) -> Result<Self, Error>
    where
        Self: Sized;
}

trait ReadVersioned {
    fn read(input: impl std::io::Read, version: i32) -> Result<Self, Error>
    where
        Self: Sized;
}

trait ReadWith {
    type With;

    fn read_with(input: impl std::io::Read, with: Self::With) -> Result<Self, Error>
    where
        Self: Sized;
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Error {
    WrongMagic,
    InvalidDynamicType(i32),
    InvalidStaticType(i32),
    Eof,
    InvalidObjectPropertyType(i32),
    InvalidActionType(i32),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WrongMagic => write!(f, "wrong magic"),
            Self::InvalidDynamicType(value) => write!(f, "invalid dynamic type: {value}"),
            Self::InvalidStaticType(value) => write!(f, "invalid static type: {value}"),
            Self::Eof => write!(f, "end of file"),
            Self::InvalidObjectPropertyType(value) => {
                write!(f, "invalid object property type: {value}")
            }
            Self::InvalidActionType(value) => write!(f, "invalid action type: {value}"),
        }
    }
}

impl std::error::Error for Error {}

pub trait Write: private::Sealed {
    /// Writes a value to the given output.
    ///
    /// The output is any type that implements [`std::io::Write`]. This can also be a mutable reference to a type that implements [`std::io::Write`].
    ///
    /// This trait is sealed and cannot be implemented for types outside of this crate.
    ///
    /// # Errors
    ///
    /// Returns an error if the underlying writer returns an error.
    fn write(&self, output: impl std::io::Write) -> std::io::Result<()>;
}

const SEGMENT_BITS: i32 = 0x7F;
const CONTINUE_BIT: i32 = 0x80;

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Varint(i32);

impl Read for Varint {
    fn read(mut input: impl std::io::Read) -> Result<Self, Error> {
        let value = leb128::read::unsigned(&mut input).map_err(|_| Error::Eof)?;

        Ok(Self(i32::try_from(value).unwrap()))
    }
}

impl Write for Varint {
    fn write(&self, mut output: impl std::io::Write) -> std::io::Result<()> {
        let mut value = self.0;

        loop {
            if (value & !SEGMENT_BITS) == 0 {
                output.write_all(&[u8::try_from(value).unwrap()])?;
                return Ok(());
            }

            (u8::try_from(value & SEGMENT_BITS).unwrap() | u8::try_from(CONTINUE_BIT).unwrap())
                .write(&mut output)?;

            value >>= 7;
        }
    }
}

impl Read for String {
    fn read(mut input: impl std::io::Read) -> Result<Self, Error> {
        let len = Varint::read(&mut input)?;

        let mut string = Self::with_capacity(usize::try_from(len.0).unwrap());

        for _ in 0..len.0 {
            let c = u8::read(&mut input)? as char;
            string.push(c);
        }

        Ok(string)
    }
}

impl Write for String {
    fn write(&self, mut output: impl std::io::Write) -> std::io::Result<()> {
        Varint(i32::try_from(self.len()).unwrap()).write(&mut output)?;

        for c in self.chars() {
            (c as u8).write(&mut output)?;
        }

        Ok(())
    }
}

impl Write for u32 {
    fn write(&self, mut output: impl std::io::Write) -> std::io::Result<()> {
        output.write_all(&self.to_le_bytes())
    }
}

impl Read for i32 {
    fn read(mut input: impl std::io::Read) -> Result<Self, Error> {
        let mut bytes = [0; 4];

        for byte in &mut bytes {
            *byte = Read::read(&mut input)?;
        }

        Ok(Self::from_le_bytes(bytes))
    }
}

impl Write for i32 {
    fn write(&self, mut output: impl std::io::Write) -> std::io::Result<()> {
        output.write_all(&self.to_le_bytes())
    }
}

impl Read for i64 {
    fn read(mut input: impl std::io::Read) -> Result<Self, Error> {
        let mut bytes = [0; 8];

        for byte in &mut bytes {
            *byte = Read::read(&mut input)?;
        }

        Ok(Self::from_le_bytes(bytes))
    }
}

impl Write for i64 {
    fn write(&self, mut output: impl std::io::Write) -> std::io::Result<()> {
        output.write_all(&self.to_le_bytes())
    }
}

impl Read for f32 {
    fn read(mut input: impl std::io::Read) -> Result<Self, Error> {
        let mut bytes = [0; 4];

        for byte in &mut bytes {
            *byte = Read::read(&mut input)?;
        }

        Ok(Self::from_le_bytes(bytes))
    }
}

impl Write for f32 {
    fn write(&self, mut output: impl std::io::Write) -> std::io::Result<()> {
        output.write_all(&self.to_le_bytes())
    }
}

impl<T: Read> Read for Vec<T> {
    fn read(mut input: impl std::io::Read) -> Result<Self, Error> {
        let len = usize::try_from(i32::read(&mut input)?).unwrap();

        let mut vec = Self::with_capacity(len);

        for _ in 0..len {
            vec.push(Read::read(&mut input)?);
        }

        Ok(vec)
    }
}

impl<T: Write> Write for Vec<T> {
    fn write(&self, mut output: impl std::io::Write) -> std::io::Result<()> {
        i32::try_from(self.len()).unwrap().write(&mut output)?;

        for item in self {
            item.write(&mut output)?;
        }

        Ok(())
    }
}

impl<T: Read + Copy + Default, const LEN: usize> Read for [T; LEN] {
    fn read(mut input: impl std::io::Read) -> Result<Self, Error> {
        let mut arr = [Default::default(); LEN];

        for item in &mut arr {
            *item = Read::read(&mut input)?;
        }

        Ok(arr)
    }
}

impl<T: Write, const LEN: usize> Write for [T; LEN] {
    fn write(&self, mut output: impl std::io::Write) -> std::io::Result<()> {
        for item in self {
            item.write(&mut output)?;
        }

        Ok(())
    }
}

impl<T: Read> Read for Option<T> {
    fn read(mut input: impl std::io::Read) -> Result<Self, Error> {
        if bool::read(&mut input)? {
            Ok(Some(Read::read(&mut input)?))
        } else {
            Ok(None)
        }
    }
}

impl<T: Write> Write for Option<T> {
    fn write(&self, mut output: impl std::io::Write) -> std::io::Result<()> {
        self.is_some().write(&mut output)?;

        if let Some(value) = self {
            value.write(&mut output)?;
        }

        Ok(())
    }
}

impl Read for bool {
    fn read(input: impl std::io::Read) -> Result<Self, Error> {
        Ok(u8::read(input)? != 0)
    }
}

impl Write for bool {
    fn write(&self, output: impl std::io::Write) -> std::io::Result<()> {
        u8::from(*self).write(output)
    }
}

impl Read for u8 {
    fn read(mut input: impl std::io::Read) -> Result<Self, Error> {
        let mut buf = [0; 1];
        input.read_exact(&mut buf).map_err(|_| Error::Eof)?;
        Ok(buf[0])
    }
}

impl Write for u8 {
    fn write(&self, mut output: impl std::io::Write) -> std::io::Result<()> {
        output.write_all(&[*self])
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Exolvl {
    pub local_level: LocalLevel,
    pub level_data: LevelData,
    pub author_replay: AuthorReplay,
}

const MAGIC: &[u8; 4] = b"NYA^";

impl Read for Exolvl {
    fn read(mut input: impl std::io::Read) -> Result<Self, Error> {
        let magic: [u8; 4] = Read::read(&mut input)?;

        if &magic != MAGIC {
            return Err(Error::WrongMagic);
        }

        let local_level = LocalLevel::read(&mut input)?;
        let level_data = ReadVersioned::read(&mut input, local_level.serialization_version)?;
        let author_replay = Read::read(&mut input)?;

        Ok(Self {
            local_level,
            level_data,
            author_replay,
        })
    }
}

impl Write for Exolvl {
    fn write(&self, mut output: impl std::io::Write) -> std::io::Result<()> {
        MAGIC.write(&mut output)?;
        self.local_level.write(&mut output)?;
        self.level_data.write(&mut output)?;
        self.author_replay.write(output)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct LocalLevel {
    pub serialization_version: i32,
    pub level_id: String,
    pub level_version: i32,
    pub level_name: String,
    pub thumbnail: String,
    pub creation_date: chrono::DateTime<chrono::Utc>,
    pub update_date: chrono::DateTime<chrono::Utc>,
    pub author_time: i64,
    pub author_lap_times: Vec<i64>,
    pub silver_medal_time: i64,
    pub gold_medal_time: i64,
    pub laps: i32,
    pub private: bool,

    unknown_1: u8,
}

impl Read for LocalLevel {
    fn read(mut input: impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            serialization_version: Read::read(&mut input)?,
            level_id: Read::read(&mut input)?,
            level_version: Read::read(&mut input)?,
            level_name: Read::read(&mut input)?,
            thumbnail: Read::read(&mut input)?,
            creation_date: Read::read(&mut input)?,
            update_date: Read::read(&mut input)?,
            author_time: Read::read(&mut input)?,
            author_lap_times: Read::read(&mut input)?,
            silver_medal_time: Read::read(&mut input)?,
            gold_medal_time: Read::read(&mut input)?,
            laps: Read::read(&mut input)?,
            private: Read::read(&mut input)?,
            unknown_1: Read::read(&mut input)?,
        })
    }
}

impl Write for LocalLevel {
    fn write(&self, mut output: impl std::io::Write) -> std::io::Result<()> {
        self.serialization_version.write(&mut output)?;
        self.level_id.write(&mut output)?;
        self.level_version.write(&mut output)?;
        self.level_name.write(&mut output)?;
        self.thumbnail.write(&mut output)?;
        self.creation_date.write(&mut output)?;
        self.update_date.write(&mut output)?;
        self.author_time.write(&mut output)?;
        self.author_lap_times.write(&mut output)?;
        self.silver_medal_time.write(&mut output)?;
        self.gold_medal_time.write(&mut output)?;
        self.laps.write(&mut output)?;
        self.private.write(&mut output)?;
        self.unknown_1.write(output)
    }
}

const TICKS_TO_SECONDS: i64 = 10_000_000;
const EPOCH_DIFFERENCE: i64 = 62_135_596_800;

impl Read for chrono::DateTime<chrono::Utc> {
    fn read(input: impl std::io::Read) -> Result<Self, Error> {
        let ticks = i64::read(input)?;

        let seconds = ticks / TICKS_TO_SECONDS - EPOCH_DIFFERENCE;

        Ok(Self::from_timestamp(seconds, 0).unwrap())
    }
}

impl Write for chrono::DateTime<chrono::Utc> {
    fn write(&self, output: impl std::io::Write) -> std::io::Result<()> {
        let ticks = (self.timestamp() + EPOCH_DIFFERENCE) * TICKS_TO_SECONDS;

        ticks.write(output)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
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

    unknown1: [u8; 24],

    pub custom_terrain_colour: Colour,

    unknown_2: [u8; 20],

    pub custom_terrain_border_colour: Colour,
    pub custom_terrain_border_thickness: f32,
    pub custom_terrain_border_corner_radius: f32,

    unknown_3: [u8; 6],

    pub default_music: bool,
    pub music_ids: Vec<String>,
    pub allow_direction_change: bool,
    pub disable_replays: bool,
    pub disable_revive_pads: bool,
    pub disable_start_animation: bool,
    pub gravity: Vec2,
}

impl ReadVersioned for LevelData {
    fn read(mut input: impl std::io::Read, version: i32) -> Result<Self, Error> {
        Ok(Self {
            level_id: Read::read(&mut input)?,
            level_version: Read::read(&mut input)?,
            nova_level: Read::read(&mut input)?,
            under_decoration_tiles: Read::read(&mut input)?,
            background_decoration_tiles_2: Read::read(&mut input)?,
            terrain_tiles: Read::read(&mut input)?,
            floating_zone_tiles: Read::read(&mut input)?,
            object_tiles: Read::read(&mut input)?,
            foreground_decoration_tiles: Read::read(&mut input)?,
            objects: Read::read(&mut input)?,
            layers: Read::read(&mut input)?,
            prefabs: Read::read(&mut input)?,
            brushes: Read::read(&mut input)?,
            patterns: Read::read(&mut input)?,
            colour_palette: if version >= 17 {
                Some(Read::read(&mut input)?)
            } else {
                None
            },
            author_time: Read::read(&mut input)?,
            author_lap_times: Read::read(&mut input)?,
            silver_medal_time: Read::read(&mut input)?,
            gold_medal_time: Read::read(&mut input)?,
            laps: Read::read(&mut input)?,
            center_camera: Read::read(&mut input)?,
            scripts: Read::read(&mut input)?,
            nova_scripts: Read::read(&mut input)?,
            global_variables: Read::read(&mut input)?,
            theme: Read::read(&mut input)?,
            custom_background_colour: Read::read(&mut input)?,
            unknown1: Read::read(&mut input)?,
            custom_terrain_colour: Read::read(&mut input)?,
            unknown_2: Read::read(&mut input)?,
            custom_terrain_border_colour: Read::read(&mut input)?,
            custom_terrain_border_thickness: Read::read(&mut input)?,
            custom_terrain_border_corner_radius: Read::read(&mut input)?,
            unknown_3: Read::read(&mut input)?,
            default_music: Read::read(&mut input)?,
            music_ids: Read::read(&mut input)?,
            allow_direction_change: Read::read(&mut input)?,
            disable_replays: Read::read(&mut input)?,
            disable_revive_pads: Read::read(&mut input)?,
            disable_start_animation: Read::read(&mut input)?,
            gravity: Read::read(&mut input)?,
        })
    }
}

impl Write for LevelData {
    fn write(&self, mut output: impl std::io::Write) -> std::io::Result<()> {
        self.level_id.write(&mut output)?;
        self.level_version.write(&mut output)?;
        self.nova_level.write(&mut output)?;
        self.under_decoration_tiles.write(&mut output)?;
        self.background_decoration_tiles_2.write(&mut output)?;
        self.terrain_tiles.write(&mut output)?;
        self.floating_zone_tiles.write(&mut output)?;
        self.object_tiles.write(&mut output)?;
        self.foreground_decoration_tiles.write(&mut output)?;
        self.objects.write(&mut output)?;
        self.layers.write(&mut output)?;
        self.prefabs.write(&mut output)?;
        self.brushes.write(&mut output)?;
        self.patterns.write(&mut output)?;
        if let Some(colour_palette) = &self.colour_palette {
            colour_palette.write(&mut output)?;
        }
        self.author_time.write(&mut output)?;
        self.author_lap_times.write(&mut output)?;
        self.silver_medal_time.write(&mut output)?;
        self.gold_medal_time.write(&mut output)?;
        self.laps.write(&mut output)?;
        self.center_camera.write(&mut output)?;
        self.scripts.write(&mut output)?;
        self.nova_scripts.write(&mut output)?;
        self.global_variables.write(&mut output)?;
        self.theme.write(&mut output)?;
        self.custom_background_colour.write(&mut output)?;
        self.unknown1.write(&mut output)?;
        self.custom_terrain_colour.write(&mut output)?;
        self.unknown_2.write(&mut output)?;
        self.custom_terrain_border_colour.write(&mut output)?;
        self.custom_terrain_border_thickness.write(&mut output)?;
        self.custom_terrain_border_corner_radius
            .write(&mut output)?;
        self.unknown_3.write(&mut output)?;
        self.default_music.write(&mut output)?;
        self.music_ids.write(&mut output)?;
        self.allow_direction_change.write(&mut output)?;
        self.disable_replays.write(&mut output)?;
        self.disable_revive_pads.write(&mut output)?;
        self.disable_start_animation.write(&mut output)?;
        self.gravity.write(output)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Pattern {
    pub pattern_id: i32,
    pub pattern_frames: Vec<Image>,
}

impl Read for Pattern {
    fn read(mut input: impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            pattern_id: Read::read(&mut input)?,
            pattern_frames: Read::read(&mut input)?,
        })
    }
}

impl Write for Pattern {
    fn write(&self, mut output: impl std::io::Write) -> std::io::Result<()> {
        self.pattern_id.write(&mut output)?;
        self.pattern_frames.write(output)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Prefab {
    pub prefab_id: i32,
    pub prefab_image_data: Image,
    pub items: Vec<Object>,
}

impl Read for Prefab {
    fn read(mut input: impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            prefab_id: Read::read(&mut input)?,
            prefab_image_data: Read::read(&mut input)?,
            items: Read::read(&mut input)?,
        })
    }
}

impl Write for Prefab {
    fn write(&self, mut output: impl std::io::Write) -> std::io::Result<()> {
        self.prefab_id.write(&mut output)?;
        self.prefab_image_data.write(&mut output)?;
        self.items.write(output)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Image(pub Vec<u8>);

impl Read for Image {
    fn read(input: impl std::io::Read) -> Result<Self, Error> {
        let data = Read::read(input)?;

        Ok(Self(data))
    }
}

impl Write for Image {
    fn write(&self, output: impl std::io::Write) -> std::io::Result<()> {
        self.0.write(output)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
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
    fn read(mut input: impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            layer_id: Read::read(&mut input)?,
            layer_name: Read::read(&mut input)?,
            selected: Read::read(&mut input)?,
            invisible: Read::read(&mut input)?,
            locked: Read::read(&mut input)?,
            foreground_type: Read::read(&mut input)?,
            parallax: Read::read(&mut input)?,
            fixed_size: Read::read(&mut input)?,
            children: Read::read(&mut input)?,
        })
    }
}

impl Write for Layer {
    fn write(&self, mut output: impl std::io::Write) -> std::io::Result<()> {
        self.layer_id.write(&mut output)?;
        self.layer_name.write(&mut output)?;
        self.selected.write(&mut output)?;
        self.invisible.write(&mut output)?;
        self.locked.write(&mut output)?;
        self.foreground_type.write(&mut output)?;
        self.parallax.write(&mut output)?;
        self.fixed_size.write(&mut output)?;
        self.children.write(output)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

impl Read for Vec2 {
    fn read(mut input: impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            x: Read::read(&mut input)?,
            y: Read::read(&mut input)?,
        })
    }
}

impl Write for Vec2 {
    fn write(&self, mut output: impl std::io::Write) -> std::io::Result<()> {
        self.x.write(&mut output)?;
        self.y.write(output)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Colour {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Read for Colour {
    fn read(mut input: impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            r: Read::read(&mut input)?,
            g: Read::read(&mut input)?,
            b: Read::read(&mut input)?,
            a: Read::read(&mut input)?,
        })
    }
}

impl Write for Colour {
    fn write(&self, mut output: impl std::io::Write) -> std::io::Result<()> {
        self.r.write(&mut output)?;
        self.g.write(&mut output)?;
        self.b.write(&mut output)?;
        self.a.write(output)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AuthorReplay(pub Vec<u8>);

impl Read for AuthorReplay {
    fn read(input: impl std::io::Read) -> Result<Self, Error> {
        Ok(Self(Read::read(input)?))
    }
}

impl Write for AuthorReplay {
    fn write(&self, output: impl std::io::Write) -> std::io::Result<()> {
        self.0.write(output)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
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
    fn read(mut input: impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            entity_id: Read::read(&mut input)?,
            tile_id: Read::read(&mut input)?,
            prefab_entity_id: Read::read(&mut input)?,
            prefab_id: Read::read(&mut input)?,
            position: Read::read(&mut input)?,
            scale: Read::read(&mut input)?,
            rotation: Read::read(&mut input)?,
            tag: Read::read(&mut input)?,
            properties: Read::read(&mut input)?,
            in_layer: Read::read(&mut input)?,
            in_group: Read::read(&mut input)?,
            group_members: Read::read(&mut input)?,
        })
    }
}

impl Write for Object {
    fn write(&self, mut output: impl std::io::Write) -> std::io::Result<()> {
        self.entity_id.write(&mut output)?;
        self.tile_id.write(&mut output)?;
        self.prefab_entity_id.write(&mut output)?;
        self.prefab_id.write(&mut output)?;
        self.position.write(&mut output)?;
        self.scale.write(&mut output)?;
        self.rotation.write(&mut output)?;
        self.tag.write(&mut output)?;
        self.properties.write(&mut output)?;
        self.in_layer.write(&mut output)?;
        self.in_group.write(&mut output)?;
        self.group_members.write(output)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
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
    fn read(mut input: impl std::io::Read) -> Result<Self, Error> {
        let property_type = Read::read(&mut input)?;

        Ok(match property_type {
            0 => Self::Colour(Read::read(&mut input)?),
            1 => Self::Resolution(Read::read(&mut input)?),
            2 => Self::FillMode(Read::read(&mut input)?),
            3 => Self::SecondaryColour(Read::read(&mut input)?),
            4 => Self::Thickness(Read::read(&mut input)?),
            5 => Self::TotalAngle(Read::read(&mut input)?),
            6 => Self::Corners(Read::read(&mut input)?),
            7 => Self::Blending(Read::read(&mut input)?),
            8 => Self::GridOffset(Read::read(&mut input)?),
            9 => Self::CornerRadius(Read::read(&mut input)?),
            10 => Self::Width(Read::read(&mut input)?),
            11 => Self::Height(Read::read(&mut input)?),
            12 => Self::BorderColour(Read::read(&mut input)?),
            13 => Self::BorderThickness(Read::read(&mut input)?),
            14 => Self::PhysicsType(Read::read(&mut input)?),
            15 => Self::Friction(Read::read(&mut input)?),
            16 => Self::TerrainCorners(Read::read(&mut input)?),
            17 => Self::Direction(Read::read(&mut input)?),
            18 => Self::Impulse(Read::read(&mut input)?),
            19 => Self::Killer(Read::read(&mut input)?),
            20 => Self::RoundReflexAngles(Read::read(&mut input)?),
            21 => Self::RoundCollider(Read::read(&mut input)?),
            22 => Self::Radius(Read::read(&mut input)?),
            23 => Self::Size(Read::read(&mut input)?),
            24 => Self::ReverseDirection(Read::read(&mut input)?),
            25 => Self::CollisionDetector(Read::read(&mut input)?),
            26 => Self::Pattern(Read::read(&mut input)?),
            27 => Self::PatternTiling(Read::read(&mut input)?),
            28 => Self::PatternOffset(Read::read(&mut input)?),
            35 => Self::Sprite(Read::read(&mut input)?),
            36 => Self::Trigger(Read::read(&mut input)?),
            37 => Self::Health(Read::read(&mut input)?),
            38 => Self::DamageFromJump(Read::read(&mut input)?),
            39 => Self::DamageFromDash(Read::read(&mut input)?),
            40 => Self::ReverseDirOnDamage(Read::read(&mut input)?),
            41 => Self::Floating(Read::read(&mut input)?),
            43 => Self::FlipX(Read::read(&mut input)?),
            44 => Self::FlipY(Read::read(&mut input)?),
            45 => Self::Text(Read::read(&mut input)?),
            46 => Self::FontSize(Read::read(&mut input)?),
            47 => Self::EditorColour(Read::read(&mut input)?),
            83 => Self::MoonInnerRadius(Read::read(&mut input)?),
            84 => Self::MoonOffset(Read::read(&mut input)?),
            n => return Err(Error::InvalidObjectPropertyType(n)),
        })
    }
}

impl Write for ObjectProperty {
    fn write(&self, mut output: impl std::io::Write) -> std::io::Result<()> {
        match self {
            Self::Colour(value) => {
                0.write(&mut output)?;
                value.write(output)
            }
            Self::Resolution(value) => {
                1.write(&mut output)?;
                value.write(output)
            }
            Self::FillMode(value) => {
                2.write(&mut output)?;
                value.write(output)
            }
            Self::SecondaryColour(value) => {
                3.write(&mut output)?;
                value.write(output)
            }
            Self::Thickness(value) => {
                4.write(&mut output)?;
                value.write(output)
            }
            Self::TotalAngle(value) => {
                5.write(&mut output)?;
                value.write(output)
            }
            Self::Corners(value) => {
                6.write(&mut output)?;
                value.write(output)
            }
            Self::Blending(value) => {
                7.write(&mut output)?;
                value.write(output)
            }
            Self::GridOffset(value) => {
                8.write(&mut output)?;
                value.write(output)
            }
            Self::CornerRadius(value) => {
                9.write(&mut output)?;
                value.write(output)
            }
            Self::Width(value) => {
                10.write(&mut output)?;
                value.write(output)
            }
            Self::Height(value) => {
                11.write(&mut output)?;
                value.write(output)
            }
            Self::BorderColour(value) => {
                12.write(&mut output)?;
                value.write(output)
            }
            Self::BorderThickness(value) => {
                13.write(&mut output)?;
                value.write(output)
            }
            Self::PhysicsType(value) => {
                14.write(&mut output)?;
                value.write(output)
            }
            Self::Friction(value) => {
                15.write(&mut output)?;
                value.write(output)
            }
            Self::TerrainCorners(value) => {
                16.write(&mut output)?;
                value.write(output)
            }
            Self::Direction(value) => {
                17.write(&mut output)?;
                value.write(output)
            }
            Self::Impulse(value) => {
                18.write(&mut output)?;
                value.write(output)
            }
            Self::Killer(value) => {
                19.write(&mut output)?;
                value.write(output)
            }
            Self::RoundReflexAngles(value) => {
                20.write(&mut output)?;
                value.write(output)
            }
            Self::RoundCollider(value) => {
                21.write(&mut output)?;
                value.write(output)
            }
            Self::Radius(value) => {
                22.write(&mut output)?;
                value.write(output)
            }
            Self::Size(value) => {
                23.write(&mut output)?;
                value.write(output)
            }
            Self::ReverseDirection(value) => {
                24.write(&mut output)?;
                value.write(output)
            }
            Self::CollisionDetector(value) => {
                25.write(&mut output)?;
                value.write(output)
            }
            Self::Pattern(value) => {
                26.write(&mut output)?;
                value.write(output)
            }
            Self::PatternTiling(value) => {
                27.write(&mut output)?;
                value.write(output)
            }
            Self::PatternOffset(value) => {
                28.write(&mut output)?;
                value.write(output)
            }
            Self::Sprite(value) => {
                35.write(&mut output)?;
                value.write(output)
            }
            Self::Trigger(value) => {
                36.write(&mut output)?;
                value.write(output)
            }
            Self::Health(value) => {
                37.write(&mut output)?;
                value.write(output)
            }
            Self::DamageFromJump(value) => {
                38.write(&mut output)?;
                value.write(output)
            }
            Self::DamageFromDash(value) => {
                39.write(&mut output)?;
                value.write(output)
            }
            Self::ReverseDirOnDamage(value) => {
                40.write(&mut output)?;
                value.write(output)
            }
            Self::Floating(value) => {
                41.write(&mut output)?;
                value.write(output)
            }
            Self::FlipX(value) => {
                43.write(&mut output)?;
                value.write(output)
            }
            Self::FlipY(value) => {
                44.write(&mut output)?;
                value.write(output)
            }
            Self::Text(value) => {
                45.write(&mut output)?;
                value.write(output)
            }
            Self::FontSize(value) => {
                46.write(&mut output)?;
                value.write(output)
            }
            Self::EditorColour(value) => {
                47.write(&mut output)?;
                value.write(output)
            }
            Self::MoonInnerRadius(value) => {
                83.write(&mut output)?;
                value.write(output)
            }
            Self::MoonOffset(value) => {
                84.write(&mut output)?;
                value.write(output)
            }
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Brush {
    pub brush_id: i32,
    pub spread: Vec2,
    pub frequency: f32,
    pub grid: BrushGrid,
    pub objects: Vec<BrushObject>,
}

impl Read for Brush {
    fn read(mut input: impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            brush_id: Read::read(&mut input)?,
            spread: Read::read(&mut input)?,
            frequency: Read::read(&mut input)?,
            grid: Read::read(&mut input)?,
            objects: Read::read(&mut input)?,
        })
    }
}

impl Write for Brush {
    fn write(&self, mut output: impl std::io::Write) -> std::io::Result<()> {
        self.brush_id.write(&mut output)?;
        self.spread.write(&mut output)?;
        self.frequency.write(&mut output)?;
        self.grid.write(&mut output)?;
        self.objects.write(output)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
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
    fn read(mut input: impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            entity_id: Read::read(&mut input)?,
            properties: Read::read(&mut input)?,
            weight: Read::read(&mut input)?,
            scale: Read::read(&mut input)?,
            rotation: Read::read(&mut input)?,
            flip_x: Read::read(&mut input)?,
            flip_y: Read::read(&mut input)?,
        })
    }
}

impl Write for BrushObject {
    fn write(&self, mut output: impl std::io::Write) -> std::io::Result<()> {
        self.entity_id.write(&mut output)?;
        self.properties.write(&mut output)?;
        self.weight.write(&mut output)?;
        self.scale.write(&mut output)?;
        self.rotation.write(&mut output)?;
        self.flip_x.write(&mut output)?;
        self.flip_y.write(output)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct BrushGrid {
    pub x: i32,
    pub y: i32,
}

impl Read for BrushGrid {
    fn read(mut input: impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            x: Read::read(&mut input)?,
            y: Read::read(&mut input)?,
        })
    }
}

impl Write for BrushGrid {
    fn write(&self, mut output: impl std::io::Write) -> std::io::Result<()> {
        self.x.write(&mut output)?;
        self.y.write(output)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
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
    fn read(mut input: impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            script_id: Read::read(&mut input)?,
            script_name: Read::read(&mut input)?,
            is_function: Read::read(&mut input)?,
            activation_count: Read::read(&mut input)?,
            condition: Read::read(&mut input)?,
            activation_list: Read::read(&mut input)?,
            parameters: Read::read(&mut input)?,
            variables: Read::read(&mut input)?,
            actions: Read::read(&mut input)?,
        })
    }
}

impl Write for NovaScript {
    fn write(&self, mut output: impl std::io::Write) -> std::io::Result<()> {
        self.script_id.write(&mut output)?;
        self.script_name.write(&mut output)?;
        self.is_function.write(&mut output)?;
        self.activation_count.write(&mut output)?;
        self.condition.write(&mut output)?;
        self.activation_list.write(&mut output)?;
        self.parameters.write(&mut output)?;
        self.variables.write(&mut output)?;
        self.actions.write(output)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Action {
    pub closed: bool,
    pub wait: bool,
    pub action_type: ActionType,
}

impl Read for Action {
    fn read(mut input: impl std::io::Read) -> Result<Self, Error> {
        let action_type = Read::read(&mut input)?;

        Ok(Self {
            closed: Read::read(&mut input)?,
            wait: Read::read(&mut input)?,
            action_type: ReadWith::read_with(input, action_type)?,
        })
    }
}

impl Write for Action {
    fn write(&self, mut output: impl std::io::Write) -> std::io::Result<()> {
        let action_type = i32::from(&self.action_type);

        action_type.write(&mut output)?;
        self.closed.write(&mut output)?;
        self.wait.write(&mut output)?;
        self.action_type.write(output)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
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
        }
    }
}

impl ReadWith for ActionType {
    type With = i32;

    fn read_with(mut input: impl std::io::Read, with: Self::With) -> Result<Self, Error> {
        Ok(match with {
            0 => Self::Repeat {
                actions: Read::read(&mut input)?,
                count: Read::read(&mut input)?,
            },
            1 => Self::RepeatWhile {
                actions: Read::read(&mut input)?,
                condition: Read::read(&mut input)?,
            },
            2 => Self::ConditionBlock {
                if_actions: Read::read(&mut input)?,
                else_actions: Read::read(&mut input)?,
                condition: Read::read(&mut input)?,
            },
            3 => Self::Wait {
                duration: Read::read(&mut input)?,
            },
            4 => Self::WaitFrames {
                frames: Read::read(&mut input)?,
            },
            5 => Self::Move {
                target_objects: Read::read(&mut input)?,
                position: Read::read(&mut input)?,
                global: Read::read(&mut input)?,
                duration: Read::read(&mut input)?,
                easing: Read::read(&mut input)?,
            },
            6 => Self::Scale {
                target_objects: Read::read(&mut input)?,
                scale: Read::read(&mut input)?,
                duration: Read::read(&mut input)?,
                easing: Read::read(&mut input)?,
            },
            7 => Self::Rotate {
                target_objects: Read::read(&mut input)?,
                rotation: Read::read(&mut input)?,
                shortest_path: Read::read(&mut input)?,
                global: Read::read(&mut input)?,
                duration: Read::read(&mut input)?,
                easing: Read::read(&mut input)?,
            },
            8 => Self::RotateAround {
                target_objects: Read::read(&mut input)?,
                pivot: Read::read(&mut input)?,
                rotation: Read::read(&mut input)?,
                rotate_target: Read::read(&mut input)?,
                duration: Read::read(&mut input)?,
                easing: Read::read(&mut input)?,
            },
            9 => Self::SetVariable {
                variable: Read::read(&mut input)?,
                value: Read::read(&mut input)?,
            },
            10 => Self::ResetVariable {
                variable: Read::read(&mut input)?,
            },
            11 => Self::ResetObject {
                target_objects: Read::read(&mut input)?,
            },
            12 => Self::SetColor {
                target_objects: Read::read(&mut input)?,
                color: Read::read(&mut input)?,
                channel: Read::read(&mut input)?,
                duration: Read::read(&mut input)?,
                easing: Read::read(&mut input)?,
            },
            13 => Self::SetTransparency {
                target_objects: Read::read(&mut input)?,
                transparency: Read::read(&mut input)?,
                channel: Read::read(&mut input)?,
                duration: Read::read(&mut input)?,
                easing: Read::read(&mut input)?,
            },
            14 => Self::SetSecondaryColor {
                target_objects: Read::read(&mut input)?,
                color: Read::read(&mut input)?,
                duration: Read::read(&mut input)?,
                easing: Read::read(&mut input)?,
            },
            15 => Self::SetSecondaryTransparency {
                target_objects: Read::read(&mut input)?,
                transparency: Read::read(&mut input)?,
                duration: Read::read(&mut input)?,
                easing: Read::read(&mut input)?,
            },
            16 => Self::SetBorderColor {
                target_objects: Read::read(&mut input)?,
                color: Read::read(&mut input)?,
                duration: Read::read(&mut input)?,
                easing: Read::read(&mut input)?,
            },
            17 => Self::SetBorderTransparency {
                target_objects: Read::read(&mut input)?,
                transparency: Read::read(&mut input)?,
                duration: Read::read(&mut input)?,
                easing: Read::read(&mut input)?,
            },
            18 => Self::SetSprite {
                target_objects: Read::read(&mut input)?,
                sprite: Read::read(&mut input)?,
            },
            19 => Self::SetText {
                target_objects: Read::read(&mut input)?,
                text: Read::read(&mut input)?,
            },
            20 => Self::SetEnabled {
                target_objects: Read::read(&mut input)?,
                enabled: Read::read(&mut input)?,
            },
            21 => Self::Activate {
                target_objects: Read::read(&mut input)?,
            },
            22 => Self::Deactivate {
                target_objects: Read::read(&mut input)?,
            },
            23 => Self::Damage {
                target_objects: Read::read(&mut input)?,
                damage: Read::read(&mut input)?,
            },
            24 => Self::Kill {
                target_objects: Read::read(&mut input)?,
            },
            25 => Self::GameFinish,
            26 => Self::CameraPan {
                position: Read::read(&mut input)?,
                duration: Read::read(&mut input)?,
                easing: Read::read(&mut input)?,
            },
            27 => Self::CameraFollowPlayer,
            28 => Self::CameraZoom {
                viewport_size: Read::read(&mut input)?,
                duration: Read::read(&mut input)?,
                easing: Read::read(&mut input)?,
            },
            29 => Self::CameraZoomReset {
                duration: Read::read(&mut input)?,
                easing: Read::read(&mut input)?,
            },
            30 => Self::CameraOffset {
                offset: Read::read(&mut input)?,
                duration: Read::read(&mut input)?,
                easing: Read::read(&mut input)?,
            },
            31 => Self::CameraOffsetReset {
                duration: Read::read(&mut input)?,
                easing: Read::read(&mut input)?,
            },
            32 => Self::CameraShake {
                strength: Read::read(&mut input)?,
                roughness: Read::read(&mut input)?,
                fade_in: Read::read(&mut input)?,
                fade_out: Read::read(&mut input)?,
                duration: Read::read(&mut input)?,
            },
            33 => Self::PlaySound {
                sound: Read::read(&mut input)?,
                volume: Read::read(&mut input)?,
                pitch: Read::read(&mut input)?,
            },
            34 => Self::PlayMusic {
                music: Read::read(&mut input)?,
                volume: Read::read(&mut input)?,
                pitch: Read::read(&mut input)?,
            },
            35 => Self::SetDirection {
                target_objects: Read::read(&mut input)?,
                direction: Read::read(&mut input)?,
            },
            36 => Self::SetGravity {
                target_objects: Read::read(&mut input)?,
                gravity: Read::read(&mut input)?,
            },
            37 => Self::SetVelocity {
                target_objects: Read::read(&mut input)?,
                velocity: Read::read(&mut input)?,
            },
            38 => Self::SetCinematic {
                enabled: Read::read(&mut input)?,
            },
            39 => Self::SetInputEnabled {
                enabled: Read::read(&mut input)?,
            },
            40 => Self::SetTimerEnabled {
                enabled: Read::read(&mut input)?,
            },
            41 => Self::GameTextShow {
                text: Read::read(&mut input)?,
                duration: Read::read(&mut input)?,
            },
            42 => Self::DialogueShow {
                text: Read::read(&mut input)?,
                position: Read::read(&mut input)?,
                reverse_direction: Read::read(&mut input)?,
            },
            43 => Self::StopScript {
                script: Read::read(&mut input)?,
            },
            44 => Self::TransitionIn {
                type_: Read::read(&mut input)?,
                color: Read::read(&mut input)?,
                duration: Read::read(&mut input)?,
                easing: Read::read(&mut input)?,
            },
            45 => Self::TransitionOut {
                type_: Read::read(&mut input)?,
                color: Read::read(&mut input)?,
                duration: Read::read(&mut input)?,
                easing: Read::read(&mut input)?,
            },
            46 => Self::TimeScale {
                time_scale: Read::read(&mut input)?,
                duration: Read::read(&mut input)?,
                easing: Read::read(&mut input)?,
            },
            47 => Self::RunFunction {
                function: Read::read(&mut input)?,
            },
            48 => Self::SetVariableOverTime {
                variable: Read::read(&mut input)?,
                value: Read::read(&mut input)?,
                duration: Read::read(&mut input)?,
                easing: Read::read(&mut input)?,
            },
            49 => Self::RepeatForEachObject {
                target_objects: Read::read(&mut input)?,
                actions: Read::read(&mut input)?,
            },
            n => return Err(Error::InvalidActionType(n)),
        })
    }
}

impl Write for ActionType {
    fn write(&self, mut output: impl std::io::Write) -> std::io::Result<()> {
        match self {
            Self::Repeat { actions, count } => {
                actions.write(&mut output)?;
                count.write(output)
            }
            Self::RepeatWhile { actions, condition } => {
                actions.write(&mut output)?;
                condition.write(output)
            }
            Self::ConditionBlock {
                if_actions,
                else_actions,
                condition,
            } => {
                if_actions.write(&mut output)?;
                else_actions.write(&mut output)?;
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
                target_objects.write(&mut output)?;
                position.write(&mut output)?;
                global.write(&mut output)?;
                duration.write(&mut output)?;
                easing.write(output)
            }
            Self::Scale {
                target_objects,
                scale,
                duration,
                easing,
            } => {
                target_objects.write(&mut output)?;
                scale.write(&mut output)?;
                duration.write(&mut output)?;
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
                target_objects.write(&mut output)?;
                rotation.write(&mut output)?;
                shortest_path.write(&mut output)?;
                global.write(&mut output)?;
                duration.write(&mut output)?;
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
                target_objects.write(&mut output)?;
                pivot.write(&mut output)?;
                rotation.write(&mut output)?;
                rotate_target.write(&mut output)?;
                duration.write(&mut output)?;
                easing.write(output)
            }
            Self::SetVariable { variable, value } => {
                variable.write(&mut output)?;
                value.write(output)
            }
            Self::ResetVariable { variable } => variable.write(output),
            Self::ResetObject { target_objects }
            | Self::Activate { target_objects }
            | Self::Deactivate { target_objects }
            | Self::Kill { target_objects } => target_objects.write(output),
            Self::SetColor {
                target_objects,
                color,
                channel,
                duration,
                easing,
            } => {
                target_objects.write(&mut output)?;
                color.write(&mut output)?;
                channel.write(&mut output)?;
                duration.write(&mut output)?;
                easing.write(output)
            }
            Self::SetTransparency {
                target_objects,
                transparency,
                channel,
                duration,
                easing,
            } => {
                target_objects.write(&mut output)?;
                transparency.write(&mut output)?;
                channel.write(&mut output)?;
                duration.write(&mut output)?;
                easing.write(output)
            }
            Self::SetSecondaryColor {
                target_objects,
                color,
                duration,
                easing,
            } => {
                target_objects.write(&mut output)?;
                color.write(&mut output)?;
                duration.write(&mut output)?;
                easing.write(output)
            }
            Self::SetSecondaryTransparency {
                target_objects,
                transparency,
                duration,
                easing,
            } => {
                target_objects.write(&mut output)?;
                transparency.write(&mut output)?;
                duration.write(&mut output)?;
                easing.write(output)
            }
            Self::SetBorderColor {
                target_objects,
                color,
                duration,
                easing,
            } => {
                target_objects.write(&mut output)?;
                color.write(&mut output)?;
                duration.write(&mut output)?;
                easing.write(output)
            }
            Self::SetBorderTransparency {
                target_objects,
                transparency,
                duration,
                easing,
            } => {
                target_objects.write(&mut output)?;
                transparency.write(&mut output)?;
                duration.write(&mut output)?;
                easing.write(output)
            }
            Self::SetSprite {
                target_objects,
                sprite,
            } => {
                target_objects.write(&mut output)?;
                sprite.write(output)
            }
            Self::SetText {
                target_objects,
                text,
            } => {
                target_objects.write(&mut output)?;
                text.write(output)
            }
            Self::SetEnabled {
                target_objects,
                enabled,
            } => {
                target_objects.write(&mut output)?;
                enabled.write(output)
            }
            Self::Damage {
                target_objects,
                damage,
            } => {
                target_objects.write(&mut output)?;
                damage.write(output)
            }
            Self::CameraPan {
                position,
                duration,
                easing,
            } => {
                position.write(&mut output)?;
                duration.write(&mut output)?;
                easing.write(output)
            }
            Self::GameFinish | Self::CameraFollowPlayer => Ok(()),
            Self::CameraZoom {
                viewport_size,
                duration,
                easing,
            } => {
                viewport_size.write(&mut output)?;
                duration.write(&mut output)?;
                easing.write(output)
            }
            Self::CameraZoomReset { duration, easing } => {
                duration.write(&mut output)?;
                easing.write(output)
            }
            Self::CameraOffset {
                offset,
                duration,
                easing,
            } => {
                offset.write(&mut output)?;
                duration.write(&mut output)?;
                easing.write(output)
            }
            Self::CameraOffsetReset { duration, easing } => {
                duration.write(&mut output)?;
                easing.write(output)
            }
            Self::CameraShake {
                strength,
                roughness,
                fade_in,
                fade_out,
                duration,
            } => {
                strength.write(&mut output)?;
                roughness.write(&mut output)?;
                fade_in.write(&mut output)?;
                fade_out.write(&mut output)?;
                duration.write(output)
            }
            Self::PlaySound {
                sound,
                volume,
                pitch,
            } => {
                sound.write(&mut output)?;
                volume.write(&mut output)?;
                pitch.write(output)
            }
            Self::PlayMusic {
                music,
                volume,
                pitch,
            } => {
                music.write(&mut output)?;
                volume.write(&mut output)?;
                pitch.write(output)
            }
            Self::SetDirection {
                target_objects,
                direction,
            } => {
                target_objects.write(&mut output)?;
                direction.write(output)
            }
            Self::SetGravity {
                target_objects,
                gravity,
            } => {
                target_objects.write(&mut output)?;
                gravity.write(output)
            }
            Self::SetVelocity {
                target_objects,
                velocity,
            } => {
                target_objects.write(&mut output)?;
                velocity.write(output)
            }
            Self::SetCinematic { enabled }
            | Self::SetInputEnabled { enabled }
            | Self::SetTimerEnabled { enabled } => enabled.write(output),
            Self::GameTextShow { text, duration } => {
                text.write(&mut output)?;
                duration.write(output)
            }
            Self::DialogueShow {
                text,
                position,
                reverse_direction,
            } => {
                text.write(&mut output)?;
                position.write(&mut output)?;
                reverse_direction.write(output)
            }
            Self::StopScript { script } => script.write(output),
            Self::TransitionIn {
                type_,
                color,
                duration,
                easing,
            } => {
                type_.write(&mut output)?;
                color.write(&mut output)?;
                duration.write(&mut output)?;
                easing.write(output)
            }
            Self::TransitionOut {
                type_,
                color,
                duration,
                easing,
            } => {
                type_.write(&mut output)?;
                color.write(&mut output)?;
                duration.write(&mut output)?;
                easing.write(output)
            }
            Self::TimeScale {
                time_scale,
                duration,
                easing,
            } => {
                time_scale.write(&mut output)?;
                duration.write(&mut output)?;
                easing.write(output)
            }
            Self::RunFunction { function } => function.write(output),
            Self::SetVariableOverTime {
                variable,
                value,
                duration,
                easing,
            } => {
                variable.write(&mut output)?;
                value.write(&mut output)?;
                duration.write(&mut output)?;
                easing.write(output)
            }
            Self::RepeatForEachObject {
                target_objects,
                actions,
            } => {
                target_objects.write(&mut output)?;
                actions.write(output)
            }
        }
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
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
    fn read(mut input: impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            dynamic_type: Read::read(&mut input)?,
            bool_value: Read::read(&mut input)?,
            int_value: Read::read(&mut input)?,
            float_value: Read::read(&mut input)?,
            string_value: Read::read(&mut input)?,
            color_value: Read::read(&mut input)?,
            vector_value: Read::read(&mut input)?,
            int_list_value: Read::read(&mut input)?,
            sub_values: Read::read(&mut input)?,
        })
    }
}

impl Write for NovaValue {
    fn write(&self, mut output: impl std::io::Write) -> std::io::Result<()> {
        self.dynamic_type.write(&mut output)?;
        self.bool_value.write(&mut output)?;
        self.int_value.write(&mut output)?;
        self.float_value.write(&mut output)?;
        self.string_value.write(&mut output)?;
        self.color_value.write(&mut output)?;
        self.vector_value.write(&mut output)?;
        self.int_list_value.write(&mut output)?;
        self.sub_values.write(output)
    }
}

macro_rules! define_dynamic_type {
    ($($name:ident = $number:expr),*) => {
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
        #[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
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
    fn read(input: impl std::io::Read) -> Result<Self, Error> {
        let value = i32::read(input)?;

        Self::try_from(value).map_err(|()| Error::InvalidDynamicType(value))
    }
}

impl Write for DynamicType {
    fn write(&self, output: impl std::io::Write) -> std::io::Result<()> {
        i32::from(self).write(output)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct FunctionCall {
    pub id: i32,
    pub parameters: Vec<CallParameter>,
}

impl Read for FunctionCall {
    fn read(mut input: impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            id: Read::read(&mut input)?,
            parameters: Read::read(&mut input)?,
        })
    }
}

impl Write for FunctionCall {
    fn write(&self, mut output: impl std::io::Write) -> std::io::Result<()> {
        self.id.write(&mut output)?;
        self.parameters.write(output)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct CallParameter {
    pub parameter_id: i32,
    pub value: NovaValue,
}

impl Read for CallParameter {
    fn read(mut input: impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            parameter_id: Read::read(&mut input)?,
            value: Read::read(&mut input)?,
        })
    }
}

impl Write for CallParameter {
    fn write(&self, mut output: impl std::io::Write) -> std::io::Result<()> {
        self.parameter_id.write(&mut output)?;
        self.value.write(output)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Variable {
    pub variable_id: i32,
    pub name: String,
    pub static_type: StaticType,
    pub initial_value: NovaValue,
}

impl Read for Variable {
    fn read(mut input: impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            variable_id: Read::read(&mut input)?,
            name: Read::read(&mut input)?,
            static_type: Read::read(&mut input)?,
            initial_value: Read::read(&mut input)?,
        })
    }
}

impl Write for Variable {
    fn write(&self, mut output: impl std::io::Write) -> std::io::Result<()> {
        self.variable_id.write(&mut output)?;
        self.name.write(&mut output)?;
        self.static_type.write(&mut output)?;
        self.initial_value.write(&mut output)
    }
}

macro_rules! define_static_type {
    ($($name:ident = $number:expr),*) => {
        #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
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
    fn read(input: impl std::io::Read) -> Result<Self, Error> {
        let value = i32::read(input)?;

        Self::try_from(value).map_err(|()| Error::InvalidStaticType(value))
    }
}

impl Write for StaticType {
    fn write(&self, output: impl std::io::Write) -> std::io::Result<()> {
        i32::from(self).write(output)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Activator {
    pub activator_type: i32,
    pub parameters: Vec<NovaValue>,
}

impl Read for Activator {
    fn read(mut input: impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            activator_type: Read::read(&mut input)?,
            parameters: Read::read(&mut input)?,
        })
    }
}

impl Write for Activator {
    fn write(&self, mut output: impl std::io::Write) -> std::io::Result<()> {
        self.activator_type.write(&mut output)?;
        self.parameters.write(&mut output)
    }
}

#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Clone, Debug, PartialEq, PartialOrd)]
pub struct Parameter {
    pub parameter_id: i32,
    pub name: String,
    pub static_type: StaticType,
    pub default_value: NovaValue,
}

impl Read for Parameter {
    fn read(mut input: impl std::io::Read) -> Result<Self, Error> {
        Ok(Self {
            parameter_id: Read::read(&mut input)?,
            name: Read::read(&mut input)?,
            static_type: Read::read(&mut input)?,
            default_value: Read::read(&mut input)?,
        })
    }
}

impl Write for Parameter {
    fn write(&self, mut output: impl std::io::Write) -> std::io::Result<()> {
        self.parameter_id.write(&mut output)?;
        self.name.write(&mut output)?;
        self.static_type.write(&mut output)?;
        self.default_value.write(&mut output)
    }
}

mod private {
    use super::*;

    pub trait Sealed {}

    impl Sealed for Varint {}
    impl Sealed for String {}
    impl Sealed for u32 {}
    impl Sealed for i32 {}
    impl Sealed for i64 {}
    impl Sealed for f32 {}
    impl<T> Sealed for Vec<T> {}
    impl<T, const LEN: usize> Sealed for [T; LEN] {}
    impl<T> Sealed for Option<T> {}
    impl Sealed for bool {}
    impl Sealed for u8 {}
    impl Sealed for Exolvl {}
    impl Sealed for LocalLevel {}
    impl Sealed for chrono::DateTime<chrono::Utc> {}
    impl Sealed for LevelData {}
    impl Sealed for Pattern {}
    impl Sealed for Prefab {}
    impl Sealed for Image {}
    impl Sealed for Layer {}
    impl Sealed for Vec2 {}
    impl Sealed for Colour {}
    impl Sealed for AuthorReplay {}
    impl Sealed for Object {}
    impl Sealed for ObjectProperty {}
    impl Sealed for Brush {}
    impl Sealed for BrushObject {}
    impl Sealed for BrushGrid {}
    impl Sealed for NovaScript {}
    impl Sealed for Action {}
    impl Sealed for ActionType {}
    impl Sealed for NovaValue {}
    impl Sealed for DynamicType {}
    impl Sealed for FunctionCall {}
    impl Sealed for CallParameter {}
    impl Sealed for Variable {}
    impl Sealed for StaticType {}
    impl Sealed for Activator {}
    impl Sealed for Parameter {}
}
