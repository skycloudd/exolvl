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
    local_level: LocalLevel,
    level_data: LevelData,
    author_replay: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct LocalLevel {
    id: String,
    version: i32,
    name: String,
    thumbnail: String,
    creation_date: DateTime<Utc>,
    update_date: DateTime<Utc>,
    author_time: i32,
    author_lap_times: Vec<i32>,
    silver_medal_time: i32,
    gold_medal_time: i32,
    laps: i32,
    private_level: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct LevelData {
    id: String,
    version: i32,
    serialization_version: i32,
    under_decoration_tiles: Vec<UnderDecorationTile>,
    background_decoration_tiles: Vec<BackgroundDecorationTile>,
    terrain_tiles: Vec<TerrainTile>,
    floating_zone_tiles: Vec<FloatingZoneTile>,
    object_tiles: Vec<ObjectTile>,
    foreground_decoration_tiles: Vec<ForegroundDecorationTile>,
    author_time: i32,
    author_lap_times: Vec<i32>,
    silver_medal_time: i32,
    gold_medal_time: i32,
    laps: i32,
    center_camera: bool,
    scripts: Vec<Script>,
    theme: Theme,
    default_music: bool,
    music_ids: Vec<String>,
    allow_direction_change: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct UnderDecorationTile {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct BackgroundDecorationTile {
    pos: Pos,
    tile_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct TerrainTile {
    pos: Pos,
    tile_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct FloatingZoneTile {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ObjectTile {
    pos: Pos,
    tile_id: String,
    entity_id: i32,
    offset: Offset,
    properties: Vec<Property>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct ForegroundDecorationTile {}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Pos {
    x: i32,
    y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Offset {
    x: f32,
    y: f32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Property {
    name: String,
    value: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Script {}

#[derive(Serialize, Deserialize, Debug)]
enum Theme {
    #[serde(rename = "mountains")]
    Mountains,

    #[serde(rename = "halloween")]
    Halloween,
}
