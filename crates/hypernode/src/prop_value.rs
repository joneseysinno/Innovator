use serde::{Deserialize, Serialize};

/// Typed property value stored on a HyperNode.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum PropValue {
    F64(f64),
    I64(i64),
    U8(u8),
    Bool(bool),
    Text(String),
}
