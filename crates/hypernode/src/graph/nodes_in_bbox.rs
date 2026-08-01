use super::Graph;
use crate::node::Node;

impl Graph {
    pub fn nodes_in_bbox(&self, min: [f64; 2], max: [f64; 2]) -> Vec<&Node> {
        self.nodes
            .values()
            .filter(|n| {
                n.world_pos[0] >= min[0]
                    && n.world_pos[0] <= max[0]
                    && n.world_pos[1] >= min[1]
                    && n.world_pos[1] <= max[1]
            })
            .collect()
    }
}
