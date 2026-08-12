use crate::edge_kind::EdgeKind;
use crate::ids::{EdgeId, NodeId};
use crate::prop_value::PropValue;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Directed hyperedge connecting one or more source nodes to targets.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HyperEdge {
    pub id: EdgeId,
    pub kind: EdgeKind,
    pub sources: Vec<NodeId>,
    pub targets: Vec<NodeId>,
    /// Curvature hint for Bézier rendering (0 = straight-ish).
    pub curvature: f32,
    pub label: Option<String>,
    /// Edge properties (e.g. Binding `"order"` for deterministic traversal).
    #[serde(default)]
    pub props: BTreeMap<String, PropValue>,
}
