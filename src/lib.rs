mod graph;
mod redstone_component;
mod runtime;
mod unified_block;

use std::thread;

pub use redstone_component::prop::BlockDirection;
pub use unified_block::UnifiedBlock;

pub fn sort_blocks(blocks: Vec<UnifiedBlock>, enable_multi_threading: bool) {
    if enable_multi_threading {
        let _block_units = blocks.chunks(blocks.len() / num_cpus::get());
    } else {
        let mut wire = Vec::new();
        let mut source = Vec::new();
        let mut comsumer = Vec::new();

        for blk in blocks
            .iter()
            .filter(|blk| blk.is_powered_block || !blk.substances.is_empty())
        {
            match blk.id.as_str() {
                "block.minecraft.redstone_wire" => wire.push(blk),

                "block.minecraft.redstone_block" | "block.minecraft.redstone_torch" => {
                    source.push(blk)
                }

                "block.minecraft.repeater" | "block.minecraft.comparator" => todo!(),

                "block.minecraft.redstone_lamp" => comsumer.push(blk),

                _ => unreachable!(),
            }
        }
    }
}
