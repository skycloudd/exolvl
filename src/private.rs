use crate::types::{
    action_type::ActionType,
    author_replay::AuthorReplay,
    brush::{Brush, BrushGrid, BrushObject},
    color::Colour,
    exolvl::Exolvl,
    image::Image,
    layer::Layer,
    level_data::LevelData,
    local_level::LocalLevel,
    object::Object,
    object_property::ObjectProperty,
    pattern::Pattern,
    prefab::Prefab,
    varint::Varint,
    vec2::Vec2,
};
use chrono::{DateTime, Utc};
#[cfg(feature = "image")]
use image::DynamicImage;
use uuid::Uuid;

pub trait Sealed {}

macro_rules! impl_sealed {
    ($($ty:ty),*$(,)?) => {
        $(
            impl Sealed for $ty {}
        )*
    };
}

impl_sealed!(
    Varint,
    String,
    u32,
    i32,
    i64,
    f32,
    bool,
    u8,
    Exolvl,
    LocalLevel,
    DateTime<Utc>,
    LevelData,
    Pattern,
    Prefab,
    Image,
    Layer,
    Vec2,
    Colour,
    AuthorReplay,
    Object,
    ObjectProperty,
    Brush,
    BrushObject,
    BrushGrid,
    super::Script,
    super::NovaScript,
    super::OldAction,
    super::OldActionType,
    super::OldActionProperty,
    super::Action,
    ActionType,
    super::NovaValue,
    super::DynamicType,
    super::FunctionCall,
    super::CallParameter,
    super::Variable,
    super::StaticType,
    super::Activator,
    super::Parameter,
    Uuid,
);

#[cfg(feature = "image")]
impl Sealed for DynamicImage {}

impl<T> Sealed for Vec<T> {}
impl<T, const LEN: usize> Sealed for [T; LEN] {}
impl<T> Sealed for Option<T> {}
