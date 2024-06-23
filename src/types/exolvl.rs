use super::{
    author_replay::AuthorReplay, color::Colour, level_data::LevelData, local_level::LocalLevel,
    vec2::Vec2,
};
use crate::{error::Error, Read, ReadVersioned, Write};

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
            local_level: LocalLevel::default_with_id(level_id),
            level_data: LevelData {
                level_id,
                level_version: 1,
                nova_level: true,
                under_decoration_tiles: Vec::default(),
                background_decoration_tiles: Vec::default(),
                terrain_tiles: Vec::default(),
                floating_zone_tiles: Vec::default(),
                object_tiles: Vec::default(),
                foreground_decoration_tiles: Vec::default(),
                objects: Vec::default(),
                layers: Vec::default(),
                prefabs: Vec::default(),
                brushes: Vec::default(),
                patterns: Vec::default(),
                colour_palette: Some(Vec::default()),
                author_time: Default::default(),
                author_lap_times: Vec::default(),
                silver_medal_time: Default::default(),
                gold_medal_time: Default::default(),
                laps: 1,
                center_camera: Default::default(),
                scripts: Vec::default(),
                nova_scripts: Vec::default(),
                global_variables: Vec::default(),
                theme: "mountains".to_string(),
                custom_background_colour: Colour::default(),
                unknown1: [0; 4],
                custom_terrain_pattern_id: Default::default(),
                custom_terrain_pattern_tiling: Vec2::default(),
                custom_terrain_pattern_offset: Vec2::default(),
                custom_terrain_colour: Colour::default(),
                custom_terrain_secondary_color: Colour::default(),
                custom_terrain_blend_mode: Default::default(),
                custom_terrain_border_colour: Colour::default(),
                custom_terrain_border_thickness: Default::default(),
                custom_terrain_border_corner_radius: Default::default(),
                custom_terrain_round_reflex_angles: Default::default(),
                custom_terrain_round_collider: Default::default(),
                custom_terrain_friction: Default::default(),
                default_music: true,
                music_ids: Vec::default(),
                allow_direction_change: Default::default(),
                disable_replays: Default::default(),
                disable_revive_pads: Default::default(),
                disable_start_animation: Default::default(),
                gravity: Vec2 { x: 0.0, y: -75.0 },
            },
            author_replay: AuthorReplay(Vec::default()),
        }
    }
}
