pub mod as_u32;

use serde::{Deserialize, Serialize};

/// Directed hyperedge relationship kinds (GPU dash patterns map to these).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EdgeKind {
    Signal = 0,
    Stream = 1,
    Wave = 2,
    Binding = 3,
}
