use binread::{BinRead, BinReaderExt, BinResult, ReadOptions};
use binwrite::BinWrite;
use std::io::{self, Read, Seek};

pub mod brush;
pub mod object;
pub mod scripts;

pub const SERIALIZATION_VERSION: i32 = 16;

pub fn read<R: BinReaderExt>(reader: &mut R) -> BinResult<Exolvl> {
    reader.read_le()
}

pub fn write(exolvl: &Exolvl) -> io::Result<Vec<u8>> {
    let mut buf = vec![];

    exolvl.write(&mut buf)?;

    Ok(buf)
}

#[derive(Debug, BinRead)]
#[br(magic = b"NYA^")]
pub struct Exolvl {
    pub local_level: LocalLevel,
    pub level_data: LevelData,
    pub author_replay: AuthorReplay,
}

impl BinWrite for Exolvl {
    fn write_options<W: std::io::prelude::Write>(
        &self,
        writer: &mut W,
        options: &binwrite::WriterOption,
    ) -> std::io::Result<()> {
        writer.write_all(b"NYA^")?;

        self.local_level.write_options(writer, options)?;

        self.level_data.write_options(writer, options)?;

        self.author_replay.write_options(writer, options)?;

        Ok(())
    }
}

#[derive(Debug, BinRead, BinWrite)]
#[br(assert(serialization_version == SERIALIZATION_VERSION, "incorrect serialization version, must be 16"))]
pub struct LocalLevel {
    pub serialization_version: i32,
    pub level_id: MyString,
    pub level_version: i32,
    pub level_name: MyString,
    pub thumbnail: MyString,
    pub creation_date: MyDateTime,
    pub update_date: MyDateTime,
    pub author_time: i64,
    pub author_lap_times: MyVec<i64>,
    pub silver_medal_time: i64,
    pub gold_medal_time: i64,
    pub laps: i32,
    #[br(map = |x: u8| x != 0)]
    #[binwrite(preprocessor(|x: &bool| bool_to_u8(*x)))]
    pub private: bool,

    unknown_1: u8,
}

#[derive(Debug)]
pub struct MyDateTime {
    pub inner: chrono::DateTime<chrono::Utc>,
    ticks: i64,
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

        let ticks = reader.read_le::<i64>()?;

        let masked_ticks = ticks & 0x3FFF_FFFF_FFFF_FFFF;
        let seconds = masked_ticks / TICKS_TO_SECONDS - EPOCH_DIFFERENCE;

        Ok(Self {
            inner: chrono::DateTime::<chrono::Utc>::from_timestamp(seconds, 0).unwrap(),
            ticks,
        })
    }
}

impl BinWrite for MyDateTime {
    fn write_options<W: std::io::prelude::Write>(
        &self,
        writer: &mut W,
        _options: &binwrite::WriterOption,
    ) -> std::io::Result<()> {
        writer.write_all(&self.ticks.to_le_bytes())?;

        Ok(())
    }
}

#[derive(Debug, BinRead, BinWrite)]
pub struct LevelData {
    pub level_id: MyString,
    pub level_version: i32,
    #[br(map = |x: u8| x != 0)]
    #[binwrite(preprocessor(|x: &bool| bool_to_u8(*x)))]
    pub nova_level: bool,
    pub under_decoration_tiles: MyVec<i32>,
    pub background_decoration_tiles_2: MyVec<i32>,
    pub terrain_tiles: MyVec<i32>,
    pub floating_zone_tiles: MyVec<i32>,
    pub object_tiles: MyVec<i32>,
    pub foreground_decoration_tiles: MyVec<i32>,
    pub objects: MyVec<object::Object>,
    pub layers: MyVec<Layer>,
    pub prefabs: MyVec<Prefab>,
    pub brushes: MyVec<brush::Brush>,
    pub patterns: MyVec<Pattern>,
    pub author_time: i64,
    pub author_lap_times: MyVec<i64>,
    pub silver_medal_time: i64,
    pub gold_medal_time: i64,
    pub laps: i32,
    #[br(map = |x: u8| x != 0)]
    #[binwrite(preprocessor(|x: &bool| bool_to_u8(*x)))]
    pub center_camera: bool,
    pub scripts: MyVec<i32>,
    pub nova_scripts: MyVec<scripts::NovaScript>,
    pub global_variables: MyVec<scripts::Variable>,
    pub theme: MyString,
    pub custom_background_colour: Colour,

    #[br(count = 24)]
    _unknown1: Vec<u8>,

    pub custom_terrain_colour: Colour,

    #[br(count = 20)]
    _unknown_2: Vec<u8>,

    pub custom_terrain_border_colour: Colour,
    pub custom_terrain_border_thickness: f32,
    pub custom_terrain_border_corner_radius: f32,

    #[br(count = 6)]
    _unknown_3: Vec<u8>,

    #[br(map = |x: u8| x != 0)]
    #[binwrite(preprocessor(|x: &bool| bool_to_u8(*x)))]
    pub default_music: bool,
    pub music_ids: MyVec<MyString>,
    #[br(map = |x: u8| x != 0)]
    #[binwrite(preprocessor(|x: &bool| bool_to_u8(*x)))]
    pub allow_direction_change: bool,
    #[br(map = |x: u8| x != 0)]
    #[binwrite(preprocessor(|x: &bool| bool_to_u8(*x)))]
    pub disable_replays: bool,
    #[br(map = |x: u8| x != 0)]
    #[binwrite(preprocessor(|x: &bool| bool_to_u8(*x)))]
    pub disable_revive_pads: bool,
    #[br(map = |x: u8| x != 0)]
    #[binwrite(preprocessor(|x: &bool| bool_to_u8(*x)))]
    pub disable_start_animation: bool,
    pub gravity: Vec2,
}

#[derive(Debug, BinRead, BinWrite)]
pub struct Pattern {
    pub pattern_id: i32,
    pub pattern_frames: MyVec<Image>,
}

#[derive(Debug, BinRead, BinWrite)]
pub struct Prefab {
    pub prefab_id: i32,
    pub prefab_image_data: Image,
    pub items: MyVec<object::Object>,
}

#[derive(Debug, BinRead)]
pub struct Image(pub MyVec<u8>);

impl BinWrite for Image {
    fn write_options<W: std::io::prelude::Write>(
        &self,
        writer: &mut W,
        options: &binwrite::WriterOption,
    ) -> std::io::Result<()> {
        self.0.write_options(writer, options)
    }
}

#[derive(Debug, BinRead, BinWrite)]
pub struct Layer {
    pub layer_id: i32,
    pub layer_name: MyString,
    #[br(map = |x: u8| x != 0)]
    #[binwrite(preprocessor(|x: &bool| bool_to_u8(*x)))]
    pub selected: bool,
    #[br(map = |x: u8| x != 0)]
    #[binwrite(preprocessor(|x: &bool| bool_to_u8(*x)))]
    pub invisible: bool,
    #[br(map = |x: u8| x != 0)]
    #[binwrite(preprocessor(|x: &bool| bool_to_u8(*x)))]
    pub locked: bool,
    pub foreground_type: i32,
    pub parallax: Vec2,
    #[br(map = |x: u8| x != 0)]
    #[binwrite(preprocessor(|x: &bool| bool_to_u8(*x)))]
    pub fixed_size: bool,
    pub children: MyVec<i32>,
}

#[derive(Debug, BinRead, BinWrite)]
pub struct Vec2 {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, BinRead, BinWrite)]
pub struct Colour {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

#[derive(Debug, BinRead, BinWrite)]
pub struct AuthorReplay {
    pub replay_data: MyVec<u8>,
}

#[derive(Debug)]
pub struct MyString(pub String);

impl BinRead for MyString {
    type Args = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        options: &ReadOptions,
        args: Self::Args,
    ) -> BinResult<Self> {
        let len = reader.read_le::<VarInt>()?;

        let mut options = *options;
        options.count = Some(len.0 as usize);

        let buf = <Vec<char>>::read_options(reader, &options, args)?;

        let string = buf.into_iter().collect::<String>();

        Ok(Self(string))
    }
}

impl BinWrite for MyString {
    fn write_options<W: std::io::prelude::Write>(
        &self,
        writer: &mut W,
        options: &binwrite::WriterOption,
    ) -> std::io::Result<()> {
        let bytes = self.0.as_bytes();

        VarInt(bytes.len() as i32).write_options(writer, options)?;

        writer.write_all(bytes)?;

        Ok(())
    }
}

#[derive(Debug)]
pub struct MyVec<T>(pub Vec<T>);

impl<T: BinRead<Args = ()>> BinRead for MyVec<T> {
    type Args = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        options: &ReadOptions,
        args: Self::Args,
    ) -> BinResult<Self> {
        let len = reader.read_le::<i32>()?;

        let mut options = *options;
        options.count = Some(len as usize);

        let buf = <Vec<T>>::read_options(reader, &options, args)?;

        Ok(Self(buf))
    }
}

impl<T: BinWrite> BinWrite for MyVec<T> {
    fn write_options<W: std::io::prelude::Write>(
        &self,
        writer: &mut W,
        options: &binwrite::WriterOption,
    ) -> std::io::Result<()> {
        writer.write_all(&(self.0.len() as i32).to_le_bytes())?;

        self.0.write_options(writer, options)
    }
}

#[derive(Debug, BinRead)]
pub struct VarInt(#[br(parse_with = parse_var_int)] pub i32);

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

        value |= i32::from((current_byte & SEGMENT_BITS) << position);

        if current_byte & CONTINUE_BIT == 0 {
            break;
        }

        position += 7;
    }

    Ok(value)
}

impl BinWrite for VarInt {
    fn write_options<W: std::io::prelude::Write>(
        &self,
        writer: &mut W,
        _options: &binwrite::WriterOption,
    ) -> std::io::Result<()> {
        let mut value = self.0;

        loop {
            let mut current_byte = (value & 0x7F) as u8;
            value >>= 7;

            if value != 0 {
                current_byte |= 0x80;
            }

            writer.write_all(&[current_byte])?;

            if value == 0 {
                break;
            }
        }

        Ok(())
    }
}

fn bool_to_u8(value: bool) -> u8 {
    u8::from(value)
}
