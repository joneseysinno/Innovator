pub mod edges_for_nodes;
pub mod insert_edge;
pub mod insert_node;
pub mod new;
pub mod nodes_in_bbox;

use crate::hyper_edge::HyperEdge;
use crate::ids::{EdgeId, NodeId};
use crate::node::Node;
use std::collections::BTreeMap;

/// In-memory graph for examples and light-weight tooling.
#[derive(Debug, Default)]
pub struct Graph {
    pub nodes: BTreeMap<NodeId, Node>,
    pub edges: BTreeMap<EdgeId, HyperEdge>,
    pub(crate) next_node: u64,
    pub(crate) next_edge: u64,
}
