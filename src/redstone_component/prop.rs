pub(crate) mod button;
pub(crate) mod comparator;
pub(crate) mod container;
pub(crate) mod lever;
pub(crate) mod powered_block;
pub(crate) mod repeater;
pub(crate) mod wire;

#[derive(Default)]
pub enum BlockDirection {
    #[default]
    North,
    South,
    West,
    East,
}
