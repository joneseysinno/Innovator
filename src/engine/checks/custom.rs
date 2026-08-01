use crate::engine::check_result::CheckResult;
use crate::walls::{format_prop, is_standard_key};
use hypernode::{HyperNode, Node, PropValue};

/// Surface custom wall properties as informational result rows.
pub fn checks(wall: &Node) -> Vec<CheckResult> {
    let mut out = Vec::new();
    for (key, value) in wall.props() {
        if is_standard_key(key) || key.ends_with("__unit") {
            continue;
        }
        let name = key.strip_prefix("custom:").unwrap_or(key);
        let unit = wall
            .get_prop(&format!("{key}__unit"))
            .map(format_prop)
            .unwrap_or_default();
        let num = match value {
            PropValue::F64(v) => *v,
            PropValue::U8(v) => *v as f64,
            PropValue::I64(v) => *v as f64,
            PropValue::Bool(b) => {
                out.push(CheckResult::info(
                    name,
                    if *b { 1.0 } else { 0.0 },
                    if unit.is_empty() { "bool" } else { &unit },
                ));
                continue;
            }
            PropValue::Text(s) => {
                out.push(CheckResult {
                    name: name.into(),
                    demand: 0.0,
                    capacity: 0.0,
                    ratio: 0.0,
                    pass: true,
                    unit: s.clone(),
                    informational: true,
                });
                continue;
            }
        };
        out.push(CheckResult::info(name, num, unit));
    }
    out
}
