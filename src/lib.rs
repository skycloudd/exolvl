use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

pub fn load_exolvl(data: &str) -> Result<ExoLvl, Box<dyn std::error::Error>> {
    Ok(parse_exolvl(data)?)
}

pub fn save_exolvl(exolvl: &ExoLvl) -> Result<String, Box<dyn std::error::Error>> {
    Ok(serde_json::to_string_pretty(exolvl)?)
}

pub fn parse_exolvl(data: &str) -> Result<ExoLvl, Box<dyn std::error::Error>> {
    let exolvl = serde_json::from_str(data)?;

    Ok(exolvl)
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExoLvl {
    pub local_level: LocalLevel,
    pub level_data: LevelData,
    pub author_replay: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LocalLevel {
    pub id: String,
    pub version: i32,
    pub name: String,
    pub thumbnail: String,
    pub creation_date: DateTime<Utc>,
    pub update_date: DateTime<Utc>,
    pub author_time: i32,
    pub author_lap_times: Vec<i32>,
    pub silver_medal_time: i32,
    pub gold_medal_time: i32,
    pub laps: i32,
    pub private_level: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LevelData {
    pub id: String,
    pub version: i32,
    pub serialization_version: i32,
    pub under_decoration_tiles: Vec<UnderDecorationTile>,
    pub background_decoration_tiles: Vec<BackgroundDecorationTile>,
    pub terrain_tiles: Vec<TerrainTile>,
    pub floating_zone_tiles: Vec<FloatingZoneTile>,
    pub object_tiles: Vec<ObjectTile>,
    pub foreground_decoration_tiles: Vec<ForegroundDecorationTile>,
    pub author_time: i32,
    pub author_lap_times: Vec<i32>,
    pub silver_medal_time: i32,
    pub gold_medal_time: i32,
    pub laps: i32,
    pub center_camera: bool,
    pub scripts: Vec<Script>,
    pub theme: Theme,
    pub default_music: bool,
    pub music_ids: Vec<String>,
    pub allow_direction_change: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UnderDecorationTile {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BackgroundDecorationTile {
    pub pos: Pos,
    pub tile_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TerrainTile {
    pub pos: Pos,
    pub tile_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FloatingZoneTile {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ObjectTile {
    pub pos: Pos,
    pub tile_id: String,
    pub entity_id: i32,
    pub offset: Offset,
    pub properties: Vec<Property>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ForegroundDecorationTile {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Pos {
    pub x: i32,
    pub y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Offset {
    pub x: f32,
    pub y: f32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Property {
    pub name: String,
    pub value: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Script {}

#[derive(Serialize, Deserialize, Debug)]
pub enum Theme {
    #[serde(rename = "mountains")]
    Mountains,

    #[serde(rename = "halloween")]
    Halloween,
}
