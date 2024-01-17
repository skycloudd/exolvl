use crate::{bool_to_u8, Colour, MyString, MyVec, Vec2};
use binread::{BinRead, BinReaderExt as _, BinResult, ReadOptions};
use binwrite::BinWrite;
use std::io::{Read, Seek};

#[derive(Debug, BinRead, BinWrite)]
pub struct Object {
    pub entity_id: i32,
    pub tile_id: i32,
    pub prefab_entity_id: i32,
    pub prefab_id: i32,
    pub position: Vec2,
    pub scale: Vec2,
    pub rotation: f32,
    pub tag: MyString,
    pub properties: MyVec<ObjectProperty>,
    pub in_layer: i32,
    pub in_group: i32,
    pub group_members: MyVec<i32>,
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
    TerrainCorners(MyVec<MyVec<Vec2>>),
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
}

impl BinRead for ObjectProperty {
    type Args = ();

    fn read_options<R: Read + Seek>(
        reader: &mut R,
        _options: &ReadOptions,
        _args: Self::Args,
    ) -> BinResult<Self> {
        match reader.read_le::<i32>()? {
            0 => Ok(Self::Colour(reader.read_le()?)),
            1 => Ok(Self::Resolution(reader.read_le()?)),
            2 => Ok(Self::FillMode(reader.read_le()?)),
            3 => Ok(Self::SecondaryColour(reader.read_le()?)),
            4 => Ok(Self::Thickness(reader.read_le()?)),
            5 => Ok(Self::TotalAngle(reader.read_le()?)),
            6 => Ok(Self::Corners(reader.read_le()?)),
            7 => Ok(Self::Blending(reader.read_le()?)),
            8 => Ok(Self::GridOffset(reader.read_le()?)),
            9 => Ok(Self::CornerRadius(reader.read_le()?)),
            10 => Ok(Self::Width(reader.read_le()?)),
            11 => Ok(Self::Height(reader.read_le()?)),
            12 => Ok(Self::BorderColour(reader.read_le()?)),
            13 => Ok(Self::BorderThickness(reader.read_le()?)),
            14 => Ok(Self::PhysicsType(reader.read_le()?)),
            15 => Ok(Self::Friction(reader.read_le()?)),
            16 => Ok(Self::TerrainCorners(
                reader.read_le::<MyVec<MyVec<Vec2>>>()?,
            )),
            17 => Ok(Self::Direction(reader.read_le()?)),
            18 => Ok(Self::Impulse(reader.read_le()?)),
            19 => Ok(Self::Killer(reader.read_le::<u8>()? != 0)),
            20 => Ok(Self::RoundReflexAngles(reader.read_le::<u8>()? != 0)),
            21 => Ok(Self::RoundCollider(reader.read_le::<u8>()? != 0)),
            22 => Ok(Self::Radius(reader.read_le()?)),
            23 => Ok(Self::Size(reader.read_le()?)),
            24 => Ok(Self::ReverseDirection(reader.read_le::<u8>()? != 0)),
            25 => Ok(Self::CollisionDetector(reader.read_le::<u8>()? != 0)),
            26 => Ok(Self::Pattern(reader.read_le()?)),
            27 => Ok(Self::PatternTiling(reader.read_le()?)),
            28 => Ok(Self::PatternOffset(reader.read_le()?)),
            35 => Ok(Self::Sprite(reader.read_le::<MyString>()?.0)),
            36 => Ok(Self::Trigger(reader.read_le::<u8>()? != 0)),
            37 => Ok(Self::Health(reader.read_le()?)),
            38 => Ok(Self::DamageFromJump(reader.read_le::<u8>()? != 0)),
            39 => Ok(Self::DamageFromDash(reader.read_le::<u8>()? != 0)),
            40 => Ok(Self::ReverseDirOnDamage(reader.read_le::<u8>()? != 0)),
            41 => Ok(Self::Floating(reader.read_le::<u8>()? != 0)),
            43 => Ok(Self::FlipX(reader.read_le::<u8>()? != 0)),
            44 => Ok(Self::FlipY(reader.read_le::<u8>()? != 0)),
            45 => Ok(Self::Text(reader.read_le::<MyString>()?.0)),
            46 => Ok(Self::FontSize(reader.read_le()?)),
            47 => Ok(Self::EditorColour(reader.read_le()?)),
            other => unreachable!("Unknown property id: {}", other),
        }
    }
}

impl BinWrite for ObjectProperty {
    fn write_options<W: std::io::prelude::Write>(
        &self,
        writer: &mut W,
        options: &binwrite::WriterOption,
    ) -> std::io::Result<()> {
        match self {
            Self::Colour(colour) => {
                writer.write_all(&0_i32.to_le_bytes())?;
                colour.write_options(writer, options)?;
            }
            Self::Resolution(resolution) => {
                writer.write_all(&1_i32.to_le_bytes())?;
                resolution.write_options(writer, options)?;
            }
            Self::FillMode(fill_mode) => {
                writer.write_all(&2_i32.to_le_bytes())?;
                fill_mode.write_options(writer, options)?;
            }
            Self::SecondaryColour(secondary_colour) => {
                writer.write_all(&3_i32.to_le_bytes())?;
                secondary_colour.write_options(writer, options)?;
            }
            Self::Thickness(thickness) => {
                writer.write_all(&4_i32.to_le_bytes())?;
                thickness.write_options(writer, options)?;
            }
            Self::TotalAngle(total_angle) => {
                writer.write_all(&5_i32.to_le_bytes())?;
                total_angle.write_options(writer, options)?;
            }
            Self::Corners(corners) => {
                writer.write_all(&6_i32.to_le_bytes())?;
                corners.write_options(writer, options)?;
            }
            Self::Blending(blending) => {
                writer.write_all(&7_i32.to_le_bytes())?;
                blending.write_options(writer, options)?;
            }
            Self::GridOffset(grid_offset) => {
                writer.write_all(&8_i32.to_le_bytes())?;
                grid_offset.write_options(writer, options)?;
            }
            Self::CornerRadius(corner_radius) => {
                writer.write_all(&9_i32.to_le_bytes())?;
                corner_radius.write_options(writer, options)?;
            }
            Self::Width(width) => {
                writer.write_all(&10_i32.to_le_bytes())?;
                width.write_options(writer, options)?;
            }
            Self::Height(height) => {
                writer.write_all(&11_i32.to_le_bytes())?;
                height.write_options(writer, options)?;
            }
            Self::BorderColour(border_colour) => {
                writer.write_all(&12_i32.to_le_bytes())?;
                border_colour.write_options(writer, options)?;
            }
            Self::BorderThickness(border_thickness) => {
                writer.write_all(&13_i32.to_le_bytes())?;
                border_thickness.write_options(writer, options)?;
            }
            Self::PhysicsType(physics_type) => {
                writer.write_all(&14_i32.to_le_bytes())?;
                physics_type.write_options(writer, options)?;
            }
            Self::Friction(friction) => {
                writer.write_all(&15_i32.to_le_bytes())?;
                friction.write_options(writer, options)?;
            }
            Self::TerrainCorners(terrain_corners) => {
                writer.write_all(&16_i32.to_le_bytes())?;
                terrain_corners.write_options(writer, options)?;
            }
            Self::Direction(direction) => {
                writer.write_all(&17_i32.to_le_bytes())?;
                direction.write_options(writer, options)?;
            }
            Self::Impulse(impulse) => {
                writer.write_all(&18_i32.to_le_bytes())?;
                impulse.write_options(writer, options)?;
            }
            Self::Killer(killer) => {
                writer.write_all(&19_i32.to_le_bytes())?;
                writer.write_all(&[bool_to_u8(*killer)])?;
            }
            Self::RoundReflexAngles(round_reflex_angles) => {
                writer.write_all(&20_i32.to_le_bytes())?;
                writer.write_all(&[bool_to_u8(*round_reflex_angles)])?;
            }
            Self::RoundCollider(round_collider) => {
                writer.write_all(&21_i32.to_le_bytes())?;
                writer.write_all(&[bool_to_u8(*round_collider)])?;
            }
            Self::Radius(radius) => {
                writer.write_all(&22_i32.to_le_bytes())?;
                radius.write_options(writer, options)?;
            }
            Self::Size(size) => {
                writer.write_all(&23_i32.to_le_bytes())?;
                size.write_options(writer, options)?;
            }
            Self::ReverseDirection(reverse_direction) => {
                writer.write_all(&24_i32.to_le_bytes())?;
                writer.write_all(&[bool_to_u8(*reverse_direction)])?;
            }
            Self::CollisionDetector(collision_detector) => {
                writer.write_all(&25_i32.to_le_bytes())?;
                writer.write_all(&[bool_to_u8(*collision_detector)])?;
            }
            Self::Pattern(pattern) => {
                writer.write_all(&26_i32.to_le_bytes())?;
                pattern.write_options(writer, options)?;
            }
            Self::PatternTiling(pattern_tiling) => {
                writer.write_all(&27_i32.to_le_bytes())?;
                pattern_tiling.write_options(writer, options)?;
            }
            Self::PatternOffset(pattern_offset) => {
                writer.write_all(&28_i32.to_le_bytes())?;
                pattern_offset.write_options(writer, options)?;
            }
            Self::Sprite(sprite) => {
                writer.write_all(&35_i32.to_le_bytes())?;
                sprite.write_options(writer, options)?;
            }
            Self::Trigger(trigger) => {
                writer.write_all(&36_i32.to_le_bytes())?;
                writer.write_all(&[bool_to_u8(*trigger)])?;
            }
            Self::Health(health) => {
                writer.write_all(&37_i32.to_le_bytes())?;
                health.write_options(writer, options)?;
            }
            Self::DamageFromJump(damage_from_jump) => {
                writer.write_all(&38_i32.to_le_bytes())?;
                writer.write_all(&[bool_to_u8(*damage_from_jump)])?;
            }
            Self::DamageFromDash(damage_from_dash) => {
                writer.write_all(&39_i32.to_le_bytes())?;
                writer.write_all(&[bool_to_u8(*damage_from_dash)])?;
            }
            Self::ReverseDirOnDamage(reverse_dir_on_damage) => {
                writer.write_all(&40_i32.to_le_bytes())?;
                writer.write_all(&[bool_to_u8(*reverse_dir_on_damage)])?;
            }
            Self::Floating(floating) => {
                writer.write_all(&41_i32.to_le_bytes())?;
                writer.write_all(&[bool_to_u8(*floating)])?;
            }
            Self::FlipX(flip_x) => {
                writer.write_all(&43_i32.to_le_bytes())?;
                writer.write_all(&[bool_to_u8(*flip_x)])?;
            }
            Self::FlipY(flip_y) => {
                writer.write_all(&44_i32.to_le_bytes())?;
                writer.write_all(&[bool_to_u8(*flip_y)])?;
            }
            Self::Text(text) => {
                writer.write_all(&45_i32.to_le_bytes())?;
                text.write_options(writer, options)?;
            }
            Self::FontSize(font_size) => {
                writer.write_all(&46_i32.to_le_bytes())?;
                font_size.write_options(writer, options)?;
            }
            Self::EditorColour(editor_colour) => {
                writer.write_all(&47_i32.to_le_bytes())?;
                editor_colour.write_options(writer, options)?;
            }
        }

        Ok(())
    }
}
