mod graph;
pub mod redstone_component;
mod runtime;
pub mod unified_block;

pub use unified_block::UnifiedBlock;

pub fn sort_blocks(blocks: Vec<UnifiedBlock>, enable_multi_threading: bool) {
    if enable_multi_threading {
        for _ in 1..=num_cpus::get() {
            std::thread::spawn(|| {});
        }
        todo!()
    } else {
        todo!()
    }
}

// fn process_in_single_thread<B: RedstoneComponent, Rdc>(vector: Vec<B>) -> Vec<Vec<Rdc>> {
//     let mut redstone_wire: Vec<impl Conductor>: Vec::new();
// }
