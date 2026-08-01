use super::check_result::CheckResult;
use super::checks::all_checks;
use hypernode::{Node, NodeId, PropValue, SpaceClass};
use std::collections::BTreeMap;
use std::time::{SystemTime, UNIX_EPOCH};

/// Analysis engine output: Results HyperNode + parsed checks.
#[derive(Debug, Clone)]
pub struct AnalysisOutput {
    pub results_node: Node,
    pub checks: Vec<CheckResult>,
    pub overall_pass: bool,
    pub governing: String,
    pub run_timestamp: i64,
}

/// Run ACI checks for `wall` and build a ResultsNode.
pub fn run_analysis(wall: &Node) -> AnalysisOutput {
    let checks = all_checks(wall);
    let structural: Vec<_> = checks.iter().filter(|c| !c.informational).collect();
    let overall_pass = structural.iter().all(|c| c.pass);
    let governing = structural
        .iter()
        .max_by(|a, b| {
            a.ratio
                .partial_cmp(&b.ratio)
                .unwrap_or(std::cmp::Ordering::Equal)
        })
        .map(|c| c.name.clone())
        .unwrap_or_else(|| "—".into());

    let run_timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0);

    let checks_json = serde_json::to_string(&checks).unwrap_or_else(|_| "[]".into());

    let results_node = Node {
        id: NodeId(0),
        space_class: SpaceClass::Function,
        label: format!("Results · {}", wall.label),
        world_pos: [wall.world_pos[0] + 1.0, wall.world_pos[1]],
        props: BTreeMap::from([
            ("wall_id".into(), PropValue::I64(wall.id.0 as i64)),
            ("run_timestamp".into(), PropValue::I64(run_timestamp)),
            ("checks".into(), PropValue::Text(checks_json)),
            ("governing".into(), PropValue::Text(governing.clone())),
            ("overall_pass".into(), PropValue::Bool(overall_pass)),
            (
                "code_ref".into(),
                PropValue::Text("ACI 318-19 (simplified)".into()),
            ),
        ]),
    };

    AnalysisOutput {
        results_node,
        checks,
        overall_pass,
        governing,
        run_timestamp,
    }
}
