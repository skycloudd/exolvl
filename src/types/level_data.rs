use super::{
    brush::Brush,
    color::Colour,
    layer::Layer,
    novascript::{variable::Variable, NovaScript},
    object::Object,
    pattern::Pattern,
    prefab::Prefab,
    vec2::Vec2,
};
use crate::{error::Error, Read, ReadVersioned, Write};
use uuid::Uuid;

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
    /// This Field is presumably only useful in .level files, not in .exolvl ones. A mismatch with the corresponding `LocalLevel` field should be avoided.
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
    pub(crate) unknown1: [u8; 4],
    /// The following terrain related fields are all used when explicitly copying certain terrain data.
    ///
    /// The custom terrain pattern that can be pasted with the `colour_paste` button if the recieving object has the `FillMode` set to `Pattern`.
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
    /// Whether the copied terrain has round reflex angles or not (only visual).
    pub custom_terrain_round_reflex_angles: bool,
    /// Whether the copied terrain has a round collider or not (not visual).
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
