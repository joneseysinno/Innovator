use super::Graph;
use crate::ids::NodeId;
use crate::node::Node;

impl Graph {
    pub fn insert_node(&mut self, mut node: Node) -> NodeId {
        if node.id.0 == 0 {
            self.next_node += 1;
            node.id = NodeId(self.next_node);
        } else {
            self.next_node = self.next_node.max(node.id.0);
        }
        let id = node.id;
        self.nodes.insert(id, node);
        id
    }
}
