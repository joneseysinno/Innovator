use serde::{Deserialize, Serialize};

/// Stable identifier for a hyperedge.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub struct EdgeId(pub u64);
