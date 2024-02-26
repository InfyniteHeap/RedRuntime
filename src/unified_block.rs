#[derive(Default)]
pub struct UnifiedBlock {
    // The name displayed in game.
    pub name: String,
    // The id that identifies blocks.
    pub id: String,
    // The position of block.
    pub pos: Vec<i32>,
    // The strength of the signal the block holds.
    // Its type isn't `Option<i32>` because the unpowered blocks will be culled by RedRuntime.
    pub signal: i32,
    // The flag that tells RedRuntime whether this block is powered block.
    pub is_powered_block: bool,
    // If the block is a container, this field holds its subtances.
    pub substances: Vec<UnifiedBlock>,
}
