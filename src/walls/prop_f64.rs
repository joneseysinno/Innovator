use hypernode::{HyperNode, Node, PropValue};

/// Read an f64-ish property (F64 or U8) with a default.
pub fn prop_f64(node: &Node, key: &str, default: f64) -> f64 {
    match node.get_prop(key) {
        Some(PropValue::F64(v)) => *v,
        Some(PropValue::U8(v)) => *v as f64,
        Some(PropValue::I64(v)) => *v as f64,
        _ => default,
    }
}
