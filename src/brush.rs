use crate::{bool_to_u8, object, MyVec, Vec2};
use binread::BinRead;
use binwrite::BinWrite;

#[derive(Debug, BinRead, BinWrite)]
pub struct Brush {
    pub brush_id: i32,
    pub spread: Vec2,
    pub frequency: f32,
    pub grid: BrushGrid,
    pub objects: MyVec<BrushObject>,
}

#[derive(Debug, BinRead, BinWrite)]
pub struct BrushObject {
    pub entity_id: i32,
    pub properties: MyVec<object::ObjectProperty>,
    pub weight: f32,
    pub scale: f32,
    pub rotation: f32,
    #[br(map = |x: u8| x != 0)]
    #[binwrite(preprocessor(|x: &bool| bool_to_u8(*x)))]
    pub flip_x: bool,
    #[br(map = |x: u8| x != 0)]
    #[binwrite(preprocessor(|x: &bool| bool_to_u8(*x)))]
    pub flip_y: bool,
}

#[derive(Debug, BinRead, BinWrite)]
pub struct BrushGrid {
    pub x: i32,
    pub y: i32,
}
