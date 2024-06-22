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
mod private;
pub mod traits;

use error::Error;
#[cfg(feature = "image")]
use image::{DynamicImage, ImageFormat};
pub use traits::{Read, ReadContext, ReadVersioned, Write};
use uuid::Uuid;

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Varint(i32);

impl Read for Varint {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let value = leb128::read::signed(input)?;

        Ok(Self(value.try_into().unwrap()))
    }
}

impl Write for Varint {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        leb128::write::signed(output, self.0.into())?;

        Ok(())
    }
}

impl Read for String {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let len = Varint::read(input)?;

        let mut string = Self::with_capacity(usize::try_from(len.0).unwrap());

        for _ in 0..len.0 {
            let c = u8::read(input)? as char;
            string.push(c);
        }

        Ok(string)
    }
}

impl Write for String {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        Varint(i32::try_from(self.len()).unwrap()).write(output)?;

        for c in self.chars() {
            (c as u8).write(output)?;
        }

        Ok(())
    }
}

impl Write for u32 {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        Ok(output.write_all(&self.to_le_bytes())?)
    }
}

impl Read for i32 {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let mut bytes = [0; 4];

        for byte in &mut bytes {
            *byte = Read::read(input)?;
        }

        Ok(Self::from_le_bytes(bytes))
    }
}

impl Write for i32 {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        Ok(output.write_all(&self.to_le_bytes())?)
    }
}

impl Read for i64 {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let mut bytes = [0; 8];

        for byte in &mut bytes {
            *byte = Read::read(input)?;
        }

        Ok(Self::from_le_bytes(bytes))
    }
}

impl Write for i64 {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        Ok(output.write_all(&self.to_le_bytes())?)
    }
}

impl Read for f32 {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let mut bytes = [0; 4];

        for byte in &mut bytes {
            *byte = Read::read(input)?;
        }

        Ok(Self::from_le_bytes(bytes))
    }
}

impl Write for f32 {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        Ok(output.write_all(&self.to_le_bytes())?)
    }
}

impl<T: Read> Read for Vec<T> {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let len = usize::try_from(i32::read(input)?).unwrap();

        let mut vec = Self::with_capacity(len);

        for _ in 0..len {
            vec.push(Read::read(input)?);
        }

        Ok(vec)
    }
}

impl<T: Write> Write for Vec<T> {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        i32::try_from(self.len()).unwrap().write(output)?;

        for item in self {
            item.write(output)?;
        }

        Ok(())
    }
}

impl<T: Read + Copy + Default, const LEN: usize> Read for [T; LEN] {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let mut arr = [Default::default(); LEN];

        for item in &mut arr {
            *item = Read::read(input)?;
        }

        Ok(arr)
    }
}

impl<T: Write, const LEN: usize> Write for [T; LEN] {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        for item in self {
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
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
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
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        u8::from(*self).write(output)
    }
}

impl Read for u8 {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let mut buf = [0; 1];
        input.read_exact(&mut buf)?;
        Ok(buf[0])
    }
}

impl Write for u8 {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        Ok(output.write_all(&[*self])?)
    }
}

#[cfg(feature = "image")]
impl Read for DynamicImage {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let vec = Vec::<u8>::read(input)?;

        image::load_from_memory(&vec).map_err(Error::from)
    }
}

#[cfg(feature = "image")]
impl Write for DynamicImage {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        let mut vec = std::io::Cursor::new(Vec::new());
        self.write_to(&mut vec, ImageFormat::Png)?;

        output.write_all(&vec.into_inner())?;

        Ok(())
    }
}

/// A full Exoracer level.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct Exolvl {
    /// The local level data for this level.
    pub local_level: LocalLevel,
    /// The actual level data.
    pub level_data: LevelData,
    /// The data for the author time replay.
    pub author_replay: AuthorReplay,
}

const EXPECTED_MAGIC: &[u8; 4] = b"NYA^";

impl Read for Exolvl {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let magic: [u8; 4] = Read::read(input)?;

        if &magic != EXPECTED_MAGIC {
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
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        EXPECTED_MAGIC.write(output)?;
        self.local_level.write(output)?;
        self.level_data.write(output)?;
        self.author_replay.write(output)
    }
}

impl Default for Exolvl {
    fn default() -> Self {
        let level_id = uuid::Uuid::new_v4();
        Self { 
            local_level: LocalLevel { 
                serialization_version: 18, 
                level_id: level_id.clone(),
                level_version: 1,
                level_name: "New level".to_string(),
                thumbnail: "".to_string(),
                creation_date: chrono::Utc::now(),
                update_date: chrono::Utc::now(),
                author_time: Default::default(),
                author_lap_times: Default::default(),
                silver_medal_time: Default::default(),
                gold_medal_time: Default::default(),
                laps: 1,
                private: Default::default(),
                nova_level: true,
            }, 
            level_data: LevelData { 
                level_id: level_id,
                level_version: 1,
                nova_level: true,
                under_decoration_tiles: Default::default(),
                background_decoration_tiles: Default::default(),
                terrain_tiles: Default::default(),
                floating_zone_tiles: Default::default(),
                object_tiles: Default::default(),
                foreground_decoration_tiles: Default::default(),
                objects: Default::default(),
                layers: Default::default(),
                prefabs: Default::default(),
                brushes: Default::default(),
                patterns: Default::default(),
                colour_palette: Some(Default::default()),
                author_time: Default::default(),
                author_lap_times: Default::default(),
                silver_medal_time: Default::default(),
                gold_medal_time: Default::default(),
                laps: 1,
                center_camera: Default::default(),
                scripts: Default::default(),
                nova_scripts: Default::default(),
                global_variables: Default::default(),
                theme: "mountains".to_string(),
                custom_background_colour: Default::default(),
                unknown1: [0; 4],
                custom_terrain_pattern_id: Default::default(),
                custom_terrain_pattern_tiling: Default::default(),
                custom_terrain_pattern_offset: Default::default(),
                custom_terrain_colour: Default::default(),
                custom_terrain_secondary_color: Default::default(),
                custom_terrain_blend_mode: Default::default(),
                custom_terrain_border_colour: Default::default(),
                custom_terrain_border_thickness: Default::default(),
                custom_terrain_border_corner_radius: Default::default(),
                custom_terrain_round_reflex_angles: Default::default(),
                custom_terrain_round_collider: Default::default(),
                custom_terrain_friction: Default::default(),
                default_music: true,
                music_ids: Default::default(),
                allow_direction_change: Default::default(),
                disable_replays: Default::default(),
                disable_revive_pads: Default::default(),
                disable_start_animation: Default::default(),
                gravity: Vec2 { x: 0.0, y: -75.0 } 
            },
            author_replay: AuthorReplay(Default::default()),
        }
    }
}
/// The local level data for this level.
///
/// This data is only ever used in the level editor and is not uploaded to the server.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash)]
pub struct LocalLevel {
    /// The version of the exolvl format that this level uses.
    /// 
    /// The current latest serialization version is 18.
    pub serialization_version: i32,
    /// The UUID of the level.
    pub level_id: Uuid,
    /// The version of the level e.g. v1, v2, etc.
    pub level_version: i32,
    /// The name of the level.
    pub level_name: String,
    /// The base64-encoded data for the thumbnail of the level.
    pub thumbnail: String,
    /// When this level was created.
    pub creation_date: chrono::DateTime<chrono::Utc>,
    /// When this level was last updated.
    pub update_date: chrono::DateTime<chrono::Utc>,
    /// The author medal time for this level in milliseconds.
    pub author_time: i64,
    /// The author medal lap times for this level in milliseconds.
    pub author_lap_times: Vec<i64>,
    /// The silver medal time for this level in milliseconds.
    pub silver_medal_time: i64,
    /// The gold medal time for this level in milliseconds.
    pub gold_medal_time: i64,
    /// The number of laps in this level.
    pub laps: i32,
    /// Whether this level is private or public.
    pub private: bool,

    /// If this is true, the level can be opened in the new level editor. Otherwise it's for the "legacy" editor.
    pub nova_level: bool,
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
            nova_level: Read::read(input)?,
        })
    }
}

impl Write for LocalLevel {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
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
        self.nova_level.write(output)
    }
}

const TICKS_TO_SECONDS: i64 = 10_000_000;
const EPOCH_DIFFERENCE: i64 = 62_135_596_800;

impl Read for chrono::DateTime<chrono::Utc> {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let ticks = i64::read(input)?;

        let seconds = ticks / TICKS_TO_SECONDS - EPOCH_DIFFERENCE;

        Ok(Self::from_timestamp(seconds, 0).unwrap())
    }
}

impl Write for chrono::DateTime<chrono::Utc> {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        let ticks = (self.timestamp() + EPOCH_DIFFERENCE) * TICKS_TO_SECONDS;

        ticks.write(output)
    }
}

/// The level data for an Exoracer level.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub struct LevelData {
    /// The UUID of the level.
    pub level_id: Uuid,
    /// The version of the level e.g. v1, v2, etc.
    pub level_version: i32,
    /// Whether this level is for the new level editor.
    ///
    /// If this is true, the level can be opened in the new level editor. Otherwise it's for the "legacy" editor. 
    /// This Field is presumably only useful in .level files, not in .exolvl ones. A mismatch with the correspomding LocalLevel field should be avoided.
    pub nova_level: bool,
    /// The tile ids for the "under decoration" layer.
    pub under_decoration_tiles: Vec<i32>,
    /// The tile ids for the "background decoration" layer.
    pub background_decoration_tiles: Vec<i32>,
    /// The tile ids for the terrain layer.
    pub terrain_tiles: Vec<i32>,
    /// The tile ids for the floating zone layer.
    pub floating_zone_tiles: Vec<i32>,
    /// The tile ids for the "object" layer.
    pub object_tiles: Vec<i32>,
    /// The tile ids for the "foreground decoration" layer.
    pub foreground_decoration_tiles: Vec<i32>,
    /// The objects in the level.
    pub objects: Vec<Object>,
    /// The layers in the level.
    pub layers: Vec<Layer>,
    /// The prefabs in the level.
    pub prefabs: Vec<Prefab>,
    /// The brushes in the level.
    pub brushes: Vec<Brush>,
    /// The patterns in the level.
    pub patterns: Vec<Pattern>,
    /// The colour palettes in the level.
    ///
    /// This is only present in levels with version 17 or higher.
    pub colour_palette: Option<Vec<Colour>>,
    /// The author medal time for this level in milliseconds.
    pub author_time: i64,
    /// The author medal lap times for this level in milliseconds.
    pub author_lap_times: Vec<i64>,
    /// The silver medal time for this level in milliseconds.
    pub silver_medal_time: i64,
    /// The gold medal time for this level in milliseconds.
    pub gold_medal_time: i64,
    /// The number of laps in this level.
    pub laps: i32,
    /// Whether the camera should be centered while playing this level.
    ///
    /// This is mostly deprecated and should stay set to false.
    pub center_camera: bool,
    /// The scripts in the level.
    ///
    /// These are used in the legacy level editor.
    pub scripts: Vec<i32>,
    /// The "new" scripts in the level.
    ///
    /// These are the scripts that are used in the new level editor. As opposed to the `scripts` field which is for the legacy editor.
    pub nova_scripts: Vec<NovaScript>,
    /// All the global variables in the level.
    pub global_variables: Vec<Variable>,
    /// The theme name of the level.
    pub theme: String,
    /// The custom background colour of the level.
    pub custom_background_colour: Colour,

    /// Unknown data.
    unknown1: [u8; 4],
    /// The following terrain related fields are all used when explicitly copying certain terrain data.
    /// 
    /// The custom terrain pattern that can be pasted with the colour_paste button if the recieving object has the FillMode set to `Pattern`.
    pub custom_terrain_pattern_id: i32,
    /// The tiling of that pattern.
    pub custom_terrain_pattern_tiling: Vec2,
    /// the offset of that pattern.
    pub custom_terrain_pattern_offset: Vec2,
    /// In the legacy editor: The custom terrain colour of the level.
    /// In the new editor: The colour of the copied terrain.
    pub custom_terrain_colour: Colour,
    /// Not 100% sure of the use of this, presumably the replacement for the border color in the new editor.
    /// Used when copying and pasting properties of terrain.
    pub custom_terrain_secondary_color: Colour,
    /// The blend mode of the copied terrain.
    pub custom_terrain_blend_mode: i32,
    /// The custom terrain border colour of the level.
    pub custom_terrain_border_colour: Colour,
    /// The thickness of the terrain border.
    pub custom_terrain_border_thickness: f32,
    /// The corner radius of the terrain border.
    pub custom_terrain_border_corner_radius: f32,
    /// Wether the copied terrain has round reflex angles or not (only visual).
    pub custom_terrain_round_reflex_angles: bool,
    /// Wether the copied terrain has a round collider or not (not visual).
    pub custom_terrain_round_collider: bool,
    /// The friction of the copied terrain.
    pub custom_terrain_friction: f32,
    /// Whether the default music should be played or not.
    pub default_music: bool,
    /// The music ids for the level. The game randomly picks one of these to play each time.
    pub music_ids: Vec<String>,
    /// Whether the game lets the player change directions or not.
    pub allow_direction_change: bool,
    /// Whether replays are disabled or not.
    ///
    /// If this is true, the game won't upload replays on this level. 
    /// (unless you explicitly upload blank shells from the history section, that only contain the time you set without any replay data. Could be a bug).
    pub disable_replays: bool,
    /// Whether revive pads are disabled or not.
    pub disable_revive_pads: bool,
    /// Whether the start animation is disabled or not.
    pub disable_start_animation: bool,
    /// The gravity vector for this level.
    pub gravity: Vec2,
}

impl ReadVersioned for LevelData {
    fn read(input: &mut impl std::io::Read, version: i32) -> Result<Self, Error> {
        Ok(Self {
            level_id: Read::read(input)?,
            level_version: Read::read(input)?,
            nova_level: Read::read(input)?,
            under_decoration_tiles: Read::read(input)?,
            background_decoration_tiles: Read::read(input)?,
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
            unknown1: Read::read(input)?,
            custom_terrain_pattern_id: Read::read(input)?,
            custom_terrain_pattern_tiling: Read::read(input)?,
            custom_terrain_pattern_offset: Read::read(input)?,
            custom_terrain_colour: Read::read(input)?,
            custom_terrain_secondary_color: Read::read(input)?,
            custom_terrain_blend_mode: Read::read(input)?,
            custom_terrain_border_colour: Read::read(input)?,
            custom_terrain_border_thickness: Read::read(input)?,
            custom_terrain_border_corner_radius: Read::read(input)?,
            custom_terrain_round_reflex_angles: Read::read(input)?,
            custom_terrain_round_collider: Read::read(input)?,
            custom_terrain_friction: Read::read(input)?,
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
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.level_id.write(output)?;
        self.level_version.write(output)?;
        self.nova_level.write(output)?;
        self.under_decoration_tiles.write(output)?;
        self.background_decoration_tiles.write(output)?;
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
        self.unknown1.write(output)?;
        self.custom_terrain_pattern_id.write(output)?;
        self.custom_terrain_pattern_tiling.write(output)?;
        self.custom_terrain_pattern_offset.write(output)?;
        self.custom_terrain_colour.write(output)?;
        self.custom_terrain_secondary_color.write(output)?;
        self.custom_terrain_blend_mode.write(output)?;
        self.custom_terrain_border_colour.write(output)?;
        self.custom_terrain_border_thickness.write(output)?;
        self.custom_terrain_border_corner_radius.write(output)?;
        self.custom_terrain_round_reflex_angles.write(output)?;
        self.custom_terrain_round_collider.write(output)?;
        self.custom_terrain_friction.write(output)?;
        self.default_music.write(output)?;
        self.music_ids.write(output)?;
        self.allow_direction_change.write(output)?;
        self.disable_replays.write(output)?;
        self.disable_revive_pads.write(output)?;
        self.disable_start_animation.write(output)?;
        self.gravity.write(output)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[cfg_attr(feature = "image", derive(Clone, Debug, PartialEq))]
#[cfg_attr(
    not(feature = "image"),
    derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)
)]
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
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.pattern_id.write(output)?;
        self.pattern_frames.write(output)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
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
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.prefab_id.write(output)?;
        self.prefab_image_data.write(output)?;
        self.items.write(output)
    }
}

#[cfg(feature = "image")]
#[derive(Clone, Debug, PartialEq)]
pub struct Image(pub DynamicImage);

#[cfg(all(feature = "image", feature = "serde"))]
impl serde::Serialize for Image {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.as_bytes().serialize(serializer)
    }
}

#[cfg(all(feature = "image", feature = "serde"))]
impl<'de> serde::Deserialize<'de> for Image {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let buffer = serde::Deserialize::deserialize(deserializer)?;

        let img = image::load_from_memory(buffer).map_err(serde::de::Error::custom)?;

        Ok(Self(img))
    }
}

#[cfg(not(feature = "image"))]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Image(pub Vec<u8>);

impl Read for Image {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let data = Read::read(input)?;

        Ok(Self(data))
    }
}

impl Write for Image {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.0.write(output)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
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
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
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

/// A 2D vector.
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug)]
pub struct Vec2 {
    /// The x-coordinate.
    pub x: f32,
    /// The y-coordinate.
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
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.x.write(output)?;
        self.y.write(output)
    }
}

impl Default for Vec2 {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug)]
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
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.r.write(output)?;
        self.g.write(output)?;
        self.b.write(output)?;
        self.a.write(output)
    }
}

impl Default for Colour {
    fn default() -> Self {
        Self {
            r: 0.0,
            g: 0.0,
            b: 0.0,
            a: 1.0,
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct AuthorReplay(pub Vec<u8>);

impl Read for AuthorReplay {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        Ok(Self(Read::read(input)?))
    }
}

impl Write for AuthorReplay {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.0.write(output)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
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
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
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

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
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
    Bounce(bool),
    RestoreVelocity(bool),
    Sprite(String),
    Trigger(bool),
    Health(f32),
    DamageFromJump(bool),
    DamageFromDash(bool),
    ReverseDirOnDamage(bool),
    Floating(bool),
    LinkedObjects(Vec::<i32>),
    FlipX(bool),
    FlipY(bool),
    Text(String),
    FontSize(f32),
    EditorColour(Colour),
    Colour2(Colour),
    Colour3(Colour),
    Colour4(Colour),
    ParticleTexture(String),
    Duration(f32),
    Delay(f32),
    Loop(bool),
    AutoPlay(bool),
    LifetimeMin(f32),
    LifetimeMax(f32),
    SimulationSpace(i32),
    Rate(f32),
    Burst(i32),
    EmitterShape(i32),
    EmitterWidth(f32),
    EmitterHeight(f32),
    EmitterTotalAngle(f32),
    SizeMin(f32),
    SizeMax(f32),
    SizeOverLifetime(bool),
    StartSizeMultiplier(f32),
    EndSizeMultiplier(f32),
    SpeedMin(f32),
    SpeedMax(f32),
    SpeeLimit(f32),
    SpeedDampen(f32),
    RotationMin(f32),
    RotationMax(f32),
    Rotationspeed(f32),
    ColourOverLifetime(bool),
    StartColourMultiplier(Colour),
    EndColourMultiplier(Colour),
    GravityMultiplier(f32),
    AnchorPos(Vec2),
    MoonInnerRadius(f32),
    MoonOffset(f32),
}

impl Read for ObjectProperty {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let property_type = Read::read(input)?;

        Ok(match property_type {
            0 => Self::Colour(Read::read(input)?),
            1 => Self::Resolution(Read::read(input)?),
            2 => Self::FillMode(Read::read(input)?),
            3 => Self::SecondaryColour(Read::read(input)?),
            4 => Self::Thickness(Read::read(input)?),
            5 => Self::TotalAngle(Read::read(input)?),
            6 => Self::Corners(Read::read(input)?),
            7 => Self::Blending(Read::read(input)?),
            8 => Self::GridOffset(Read::read(input)?),
            9 => Self::CornerRadius(Read::read(input)?),
            10 => Self::Width(Read::read(input)?),
            11 => Self::Height(Read::read(input)?),
            12 => Self::BorderColour(Read::read(input)?),
            13 => Self::BorderThickness(Read::read(input)?),
            14 => Self::PhysicsType(Read::read(input)?),
            15 => Self::Friction(Read::read(input)?),
            16 => Self::TerrainCorners(Read::read(input)?),
            17 => Self::Direction(Read::read(input)?),
            18 => Self::Impulse(Read::read(input)?),
            19 => Self::Killer(Read::read(input)?),
            20 => Self::RoundReflexAngles(Read::read(input)?),
            21 => Self::RoundCollider(Read::read(input)?),
            22 => Self::Radius(Read::read(input)?),
            23 => Self::Size(Read::read(input)?),
            24 => Self::ReverseDirection(Read::read(input)?),
            25 => Self::CollisionDetector(Read::read(input)?),
            26 => Self::Pattern(Read::read(input)?),
            27 => Self::PatternTiling(Read::read(input)?),
            28 => Self::PatternOffset(Read::read(input)?),
            32 => Self::BorderThickness(Read::read(input)?),
            34 => Self::RestoreVelocity(Read::read(input)?),
            35 => Self::Sprite(Read::read(input)?),
            36 => Self::Trigger(Read::read(input)?),
            37 => Self::Health(Read::read(input)?),
            38 => Self::DamageFromJump(Read::read(input)?),
            39 => Self::DamageFromDash(Read::read(input)?),
            40 => Self::ReverseDirOnDamage(Read::read(input)?),
            41 => Self::Floating(Read::read(input)?),
            42 => Self::LinkedObjects(Read::read(input)?),
            43 => Self::FlipX(Read::read(input)?),
            44 => Self::FlipY(Read::read(input)?),
            45 => Self::Text(Read::read(input)?),
            46 => Self::FontSize(Read::read(input)?),
            47 => Self::EditorColour(Read::read(input)?),
            48 => Self::Colour2(Read::read(input)?),
            49 => Self::Colour3(Read::read(input)?),
            50 => Self::Colour4(Read::read(input)?),
            51 => Self::ParticleTexture(Read::read(input)?),
            52 => Self::Duration(Read::read(input)?),
            53 => Self::Delay(Read::read(input)?),
            54 => Self::Loop(Read::read(input)?),
            55 => Self::AutoPlay(Read::read(input)?),
            56 => Self::LifetimeMin(Read::read(input)?),
            57 => Self::LifetimeMax(Read::read(input)?),
            58 => Self::SimulationSpace(Read::read(input)?),
            59 => Self::Rate(Read::read(input)?),
            60 => Self::Burst(Read::read(input)?),
            61 => Self::EmitterShape(Read::read(input)?),
            62 => Self::EmitterWidth(Read::read(input)?),
            63 => Self::EmitterHeight(Read::read(input)?),
            64 => Self::EmitterTotalAngle(Read::read(input)?),
            65 => Self::SizeMin(Read::read(input)?),
            66 => Self::SizeMax(Read::read(input)?),
            67 => Self::SizeOverLifetime(Read::read(input)?),
            68 => Self::StartSizeMultiplier(Read::read(input)?),
            69 => Self::EndSizeMultiplier(Read::read(input)?),
            71 => Self::SpeedMin(Read::read(input)?),
            72 => Self::SpeedMax(Read::read(input)?),
            73 => Self::SpeeLimit(Read::read(input)?),
            74 => Self::SpeedDampen(Read::read(input)?),
            75 => Self::RotationMin(Read::read(input)?),
            76 => Self::RotationMax(Read::read(input)?),
            77 => Self::Rotationspeed(Read::read(input)?),
            78 => Self::ColourOverLifetime(Read::read(input)?),
            79 => Self::StartColourMultiplier(Read::read(input)?),
            80 => Self::EndColourMultiplier(Read::read(input)?),
            81 => Self::GravityMultiplier(Read::read(input)?),
            82 => Self::AnchorPos(Read::read(input)?),
            83 => Self::MoonInnerRadius(Read::read(input)?),
            84 => Self::MoonOffset(Read::read(input)?),
            n => return Err(crate::error::Error::InvalidObjectPropertyType(n)),
        })
    }
}

impl Write for ObjectProperty {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        match self {
            Self::Colour(value) => {
                0.write(output)?;
                value.write(output)
            }
            Self::Resolution(value) => {
                1.write(output)?;
                value.write(output)
            }
            Self::FillMode(value) => {
                2.write(output)?;
                value.write(output)
            }
            Self::SecondaryColour(value) => {
                3.write(output)?;
                value.write(output)
            }
            Self::Thickness(value) => {
                4.write(output)?;
                value.write(output)
            }
            Self::TotalAngle(value) => {
                5.write(output)?;
                value.write(output)
            }
            Self::Corners(value) => {
                6.write(output)?;
                value.write(output)
            }
            Self::Blending(value) => {
                7.write(output)?;
                value.write(output)
            }
            Self::GridOffset(value) => {
                8.write(output)?;
                value.write(output)
            }
            Self::CornerRadius(value) => {
                9.write(output)?;
                value.write(output)
            }
            Self::Width(value) => {
                10.write(output)?;
                value.write(output)
            }
            Self::Height(value) => {
                11.write(output)?;
                value.write(output)
            }
            Self::BorderColour(value) => {
                12.write(output)?;
                value.write(output)
            }
            Self::BorderThickness(value) => {
                13.write(output)?;
                value.write(output)
            }
            Self::PhysicsType(value) => {
                14.write(output)?;
                value.write(output)
            }
            Self::Friction(value) => {
                15.write(output)?;
                value.write(output)
            }
            Self::TerrainCorners(value) => {
                16.write(output)?;
                value.write(output)
            }
            Self::Direction(value) => {
                17.write(output)?;
                value.write(output)
            }
            Self::Impulse(value) => {
                18.write(output)?;
                value.write(output)
            }
            Self::Killer(value) => {
                19.write(output)?;
                value.write(output)
            }
            Self::RoundReflexAngles(value) => {
                20.write(output)?;
                value.write(output)
            }
            Self::RoundCollider(value) => {
                21.write(output)?;
                value.write(output)
            }
            Self::Radius(value) => {
                22.write(output)?;
                value.write(output)
            }
            Self::Size(value) => {
                23.write(output)?;
                value.write(output)
            }
            Self::ReverseDirection(value) => {
                24.write(output)?;
                value.write(output)
            }
            Self::CollisionDetector(value) => {
                25.write(output)?;
                value.write(output)
            }
            Self::Pattern(value) => {
                26.write(output)?;
                value.write(output)
            }
            Self::PatternTiling(value) => {
                27.write(output)?;
                value.write(output)
            }
            Self::PatternOffset(value) => {
                28.write(output)?;
                value.write(output)
            }
            Self::Bounce(value) => {
                32.write(output)?;
                value.write(output)
            }
            Self::RestoreVelocity(value) => {
                32.write(output)?;
                value.write(output)
            }
            Self::Sprite(value) => {
                35.write(output)?;
                value.write(output)
            }
            Self::Trigger(value) => {
                36.write(output)?;
                value.write(output)
            }
            Self::Health(value) => {
                37.write(output)?;
                value.write(output)
            }
            Self::DamageFromJump(value) => {
                38.write(output)?;
                value.write(output)
            }
            Self::DamageFromDash(value) => {
                39.write(output)?;
                value.write(output)
            }
            Self::ReverseDirOnDamage(value) => {
                40.write(output)?;
                value.write(output)
            }
            Self::Floating(value) => {
                41.write(output)?;
                value.write(output)
            }
            Self::LinkedObjects(value) => {
                42.write(output)?;
                value.write(output)
            }
            Self::FlipX(value) => {
                43.write(output)?;
                value.write(output)
            }
            Self::FlipY(value) => {
                44.write(output)?;
                value.write(output)
            }
            Self::Text(value) => {
                45.write(output)?;
                value.write(output)
            }
            Self::FontSize(value) => {
                46.write(output)?;
                value.write(output)
            }
            Self::EditorColour(value) => {
                47.write(output)?;
                value.write(output)
            }
            Self::Colour2(value) => {
                48.write(output)?;
                value.write(output)
            }
            Self::Colour3(value) => {
                49.write(output)?;
                value.write(output)
            }
            Self::Colour4(value) => {
                50.write(output)?;
                value.write(output)
            }
            Self::ParticleTexture(value) => {
                51.write(output)?;
                value.write(output)
            }
            Self::Duration(value) => {
                52.write(output)?;
                value.write(output)
            }
            Self::Delay(value) => {
                53.write(output)?;
                value.write(output)
            }
            Self::Loop(value) => {
                54.write(output)?;
                value.write(output)
            }
            Self::AutoPlay(value) => {
                55.write(output)?;
                value.write(output)
            }
            Self::LifetimeMin(value) => {
                56.write(output)?;
                value.write(output)
            }
            Self::LifetimeMax(value) => {
                57.write(output)?;
                value.write(output)
            }
            Self::SimulationSpace(value) => {
                58.write(output)?;
                value.write(output)
            }
            Self::Rate(value) => {
                59.write(output)?;
                value.write(output)
            }
            Self::Burst(value) => {
                60.write(output)?;
                value.write(output)
            }
            Self::EmitterShape(value) => {
                61.write(output)?;
                value.write(output)
            }
            Self::EmitterWidth(value) => {
                62.write(output)?;
                value.write(output)
            }
            Self::EmitterHeight(value) => {
                63.write(output)?;
                value.write(output)
            }
            Self::EmitterTotalAngle(value) => {
                64.write(output)?;
                value.write(output)
            }
            Self::SizeMin(value) => {
                65.write(output)?;
                value.write(output)
            }
            Self::SizeMax(value) => {
                66.write(output)?;
                value.write(output)
            }
            Self::SizeOverLifetime(value) => {
                67.write(output)?;
                value.write(output)
            }
            Self::StartSizeMultiplier(value) => {
                68.write(output)?;
                value.write(output)
            }
            Self::EndSizeMultiplier(value) => {
                69.write(output)?;
                value.write(output)
            }
            Self::SpeedMin(value) => {
                71.write(output)?;
                value.write(output)
            }
            Self::SpeedMax(value) => {
                72.write(output)?;
                value.write(output)
            }
            Self::SpeeLimit(value) => {
                73.write(output)?;
                value.write(output)
            }
            Self::SpeedDampen(value) => {
                74.write(output)?;
                value.write(output)
            }
            Self::RotationMin(value) => {
                75.write(output)?;
                value.write(output)
            }
            Self::RotationMax(value) => {
                76.write(output)?;
                value.write(output)
            }
            Self::Rotationspeed(value) => {
                77.write(output)?;
                value.write(output)
            }
            Self::ColourOverLifetime(value) => {
                78.write(output)?;
                value.write(output)
            }
            Self::StartColourMultiplier(value) => {
                79.write(output)?;
                value.write(output)
            }
            Self::EndColourMultiplier(value) => {
                80.write(output)?;
                value.write(output)
            }
            Self::GravityMultiplier(value) => {
                81.write(output)?;
                value.write(output)
            }
            Self::AnchorPos(value) => {
                82.write(output)?;
                value.write(output)
            }
            Self::MoonInnerRadius(value) => {
                83.write(output)?;
                value.write(output)
            }
            Self::MoonOffset(value) => {
                84.write(output)?;
                value.write(output)
            }
        }
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
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
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.brush_id.write(output)?;
        self.spread.write(output)?;
        self.frequency.write(output)?;
        self.grid.write(output)?;
        self.objects.write(output)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
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
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.entity_id.write(output)?;
        self.properties.write(output)?;
        self.weight.write(output)?;
        self.scale.write(output)?;
        self.rotation.write(output)?;
        self.flip_x.write(output)?;
        self.flip_y.write(output)
    }
}

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
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
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.x.write(output)?;
        self.y.write(output)
    }
}

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
            Self: Sized {
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
            Self: Sized {
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
            Self: Sized {
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
        fade_out: NovaValue
    },
    PlayParticleSystem {
        target_objects: NovaValue
    },
    StopParticleSystem {
        target_objects: NovaValue, 
        clear: NovaValue
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
            | Self::Kill { target_objects } => target_objects.write(output),
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
                fade_out 
            } => {
                sound_instance.write(output)?;
                fade_out.write(output)
            }
            Self::PlayParticleSystem { 
                target_objects 
            } => {
                target_objects.write(output)
            }
            Self::StopParticleSystem { 
                target_objects, 
                clear 
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
            Self: Sized {
        Ok(uuid::Uuid::parse_str(&String::read(input)?).unwrap())
    }
}

impl Write for Uuid {
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        self.to_string().write(output)
    }
}
