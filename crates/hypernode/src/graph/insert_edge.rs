use super::Graph;
use crate::hyper_edge::HyperEdge;
use crate::ids::EdgeId;

impl Graph {
    pub fn insert_edge(&mut self, mut edge: HyperEdge) -> EdgeId {
        if edge.id.0 == 0 {
            self.next_edge += 1;
            edge.id = EdgeId(self.next_edge);
        } else {
            self.next_edge = self.next_edge.max(edge.id.0);
        }
        let id = edge.id;
        self.edges.insert(id, edge);
        id
    }
}
