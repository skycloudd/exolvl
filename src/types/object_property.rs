use super::{color::Color, vec2::Vec2};
use crate::{error::Error, Read, Write};

#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[derive(Clone, Debug)]
pub enum ObjectProperty {
    Color(Color),
    Resolution(i32),
    FillMode(i32),
    SecondaryColor(Color),
    Thickness(f32),
    TotalAngle(i32),
    Corners(i32),
    Blending(i32),
    GridOffset(Vec2),
    CornerRadius(f32),
    Width(f32),
    Height(f32),
    BorderColor(Color),
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
    LinkedObjects(Vec<i32>),
    FlipX(bool),
    FlipY(bool),
    Text(String),
    FontSize(f32),
    EditorColor(Color),
    Color2(Color),
    Color3(Color),
    Color4(Color),
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
    ColorOverLifetime(bool),
    StartColorMultiplier(Color),
    EndColorMultiplier(Color),
    GravityMultiplier(f32),
    AnchorPos(Vec2),
    MoonInnerRadius(f32),
    MoonOffset(f32),
}

impl Read for ObjectProperty {
    fn read(input: &mut impl std::io::Read) -> Result<Self, Error> {
        let property_type = Read::read(input)?;

        Ok(match property_type {
            0 => Self::Color(Read::read(input)?),
            1 => Self::Resolution(Read::read(input)?),
            2 => Self::FillMode(Read::read(input)?),
            3 => Self::SecondaryColor(Read::read(input)?),
            4 => Self::Thickness(Read::read(input)?),
            5 => Self::TotalAngle(Read::read(input)?),
            6 => Self::Corners(Read::read(input)?),
            7 => Self::Blending(Read::read(input)?),
            8 => Self::GridOffset(Read::read(input)?),
            9 => Self::CornerRadius(Read::read(input)?),
            10 => Self::Width(Read::read(input)?),
            11 => Self::Height(Read::read(input)?),
            12 => Self::BorderColor(Read::read(input)?),
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
            32 => Self::Bounce(Read::read(input)?),
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
            47 => Self::EditorColor(Read::read(input)?),
            48 => Self::Color2(Read::read(input)?),
            49 => Self::Color3(Read::read(input)?),
            50 => Self::Color4(Read::read(input)?),
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
            78 => Self::ColorOverLifetime(Read::read(input)?),
            79 => Self::StartColorMultiplier(Read::read(input)?),
            80 => Self::EndColorMultiplier(Read::read(input)?),
            81 => Self::GravityMultiplier(Read::read(input)?),
            82 => Self::AnchorPos(Read::read(input)?),
            83 => Self::MoonInnerRadius(Read::read(input)?),
            84 => Self::MoonOffset(Read::read(input)?),
            n => return Err(Error::InvalidObjectPropertyType(n)),
        })
    }
}

impl Write for ObjectProperty {
    #[allow(clippy::too_many_lines)]
    fn write(&self, output: &mut impl std::io::Write) -> Result<(), Error> {
        match self {
            Self::Color(value) => {
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
            Self::SecondaryColor(value) => {
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
            Self::BorderColor(value) => {
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
                34.write(output)?;
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
            Self::EditorColor(value) => {
                47.write(output)?;
                value.write(output)
            }
            Self::Color2(value) => {
                48.write(output)?;
                value.write(output)
            }
            Self::Color3(value) => {
                49.write(output)?;
                value.write(output)
            }
            Self::Color4(value) => {
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
            Self::ColorOverLifetime(value) => {
                78.write(output)?;
                value.write(output)
            }
            Self::StartColorMultiplier(value) => {
                79.write(output)?;
                value.write(output)
            }
            Self::EndColorMultiplier(value) => {
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
