use crate::types::{
    action_type::ActionType,
    author_replay::AuthorReplay,
    brush::{Brush, BrushGrid, BrushObject},
    color::Color,
    dynamic_type::DynamicType,
    exolvl::Exolvl,
    function_call::{CallParameter, FunctionCall},
    image::Image,
    layer::Layer,
    level_data::LevelData,
    local_level::LocalLevel,
    novascript::{
        action::Action, activator::Activator, nova_value::NovaValue, parameter::Parameter,
        static_type::StaticType, variable::Variable, NovaScript,
    },
    object::Object,
    object_property::ObjectProperty,
    old_script::{OldAction, OldActionProperty, OldActionType, Script},
    pattern::Pattern,
    prefab::Prefab,
    theme::Theme,
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
    &str,
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
    Color,
    AuthorReplay,
    Object,
    ObjectProperty,
    Brush,
    BrushObject,
    BrushGrid,
    Script,
    NovaScript,
    OldAction,
    OldActionType,
    OldActionProperty,
    Action,
    ActionType,
    NovaValue,
    DynamicType,
    FunctionCall,
    CallParameter,
    Variable,
    StaticType,
    Activator,
    Parameter,
    Uuid,
    Theme
);

#[cfg(feature = "image")]
impl Sealed for DynamicImage {}

impl<T> Sealed for Vec<T> {}
impl<T, const LEN: usize> Sealed for [T; LEN] {}
impl<T> Sealed for Option<T> {}
