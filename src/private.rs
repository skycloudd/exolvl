use chrono::{DateTime, Utc};
use uuid::Uuid;
#[cfg(feature = "image")]
use image::DynamicImage;

pub trait Sealed {}

macro_rules! impl_sealed {
    ($($ty:ty),*$(,)?) => {
        $(
            impl Sealed for $ty {}
        )*
    };
}

impl_sealed!(
    super::Varint,
    String,
    u32,
    i32,
    i64,
    f32,
    bool,
    u8,
    super::Exolvl,
    super::LocalLevel,
    DateTime<Utc>,
    super::LevelData,
    super::Pattern,
    super::Prefab,
    super::Image,
    super::Layer,
    super::Vec2,
    super::Colour,
    super::AuthorReplay,
    super::Object,
    super::ObjectProperty,
    super::Brush,
    super::BrushObject,
    super::BrushGrid,
    super::Script,
    super::NovaScript,
    super::OldAction,
    super::OldActionType,
    super::OldActionProperty,
    super::Action,
    super::ActionType,
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
