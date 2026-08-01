use serde::{Deserialize, Serialize};

/// Stable identifier for a node in a hypergraph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct NodeId(pub u64);
