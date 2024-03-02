mod component_connection;

use petgraph::*;

use crate::UnifiedBlock;

pub(crate) fn create_graph() -> Graph<UnifiedBlock, i32> {
    Graph::new()
}

pub(crate) fn add_node(g: &mut Graph<UnifiedBlock, i32>, node: UnifiedBlock) {
    g.add_node(node);
}
