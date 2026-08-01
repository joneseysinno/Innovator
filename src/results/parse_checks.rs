use crate::engine::CheckResult;
use hypernode::{HyperNode, Node, PropValue};

/// Deserialize checks JSON from a ResultsNode.
pub fn parse_checks(results: &Node) -> Vec<CheckResult> {
    match results.get_prop("checks") {
        Some(PropValue::Text(json)) => serde_json::from_str(json).unwrap_or_default(),
        _ => Vec::new(),
    }
}
