use serde::{Deserialize, Serialize};

/// Classification of a node's role in the spatial hypergraph.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SpaceClass {
    /// Interactive UI surface / view node.
    UIView,
    /// Pure computation / analysis function.
    Function,
    /// Domain entity (e.g. a wall, result set).
    Entity,
    /// Ephemeral carrier node for signal traffic.
    Carrier,
}
