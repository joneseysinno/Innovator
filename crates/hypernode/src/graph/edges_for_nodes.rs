use super::Graph;
use crate::hyper_edge::HyperEdge;
use crate::ids::NodeId;

impl Graph {
    pub fn edges_for_nodes(&self, ids: &[NodeId]) -> Vec<&HyperEdge> {
        self.edges
            .values()
            .filter(|e| {
                e.sources.iter().any(|s| ids.contains(s))
                    || e.targets.iter().any(|t| ids.contains(t))
            })
            .collect()
    }
}
