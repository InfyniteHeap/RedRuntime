use super::*;

pub struct RedstoneRepeater {
    pub delay: u8,
    pub facing: BlockDirection,
    pub locked: bool,
    pub powered: bool,
}

impl RedstoneRepeater {
    pub fn new(delay: u8, facing: BlockDirection, locked: bool, powered: bool) -> Self {
        Self {
            delay,
            facing,
            locked,
            powered,
        }
    }
}

impl Default for RedstoneRepeater {
    fn default() -> Self {
        Self {
            delay: 1,
            facing: Default::default(),
            locked: false,
            powered: false,
        }
    }
}
