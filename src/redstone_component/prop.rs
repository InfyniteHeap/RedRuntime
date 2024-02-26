pub mod button;
pub mod comparator;
pub mod container;
pub mod lever;
pub mod powered_block;
pub mod repeater;
pub mod wire;

#[derive(Default)]
pub enum BlockDirection {
    #[default]
    North,
    South,
    West,
    East,
}

#[derive(Default)]
pub struct BlockPos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}
