pub mod hyper_node_impl;

use crate::ids::NodeId;
use crate::prop_value::PropValue;
use crate::space_class::SpaceClass;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// A concrete, general-purpose HyperNode.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    pub id: NodeId,
    pub space_class: SpaceClass,
    pub label: String,
    pub world_pos: [f64; 2],
    pub props: BTreeMap<String, PropValue>,
}
