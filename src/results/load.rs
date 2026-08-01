use crate::results::space::RESULTS_SPACE;
use hypernode::{Node, NodeId};
use infinite_db::InfiniteDb;

/// Load the latest ResultsNode for a wall, if present.
pub fn load_results_for_wall(db: &mut InfiniteDb, wall_id: NodeId) -> Option<Node> {
    let Ok(records) = db.query(RESULTS_SPACE, None) else {
        return None;
    };
    let mut best: Option<Node> = None;
    for record in records {
        if record.tombstone {
            continue;
        }
        let Ok(node) = serde_json::from_slice::<Node>(&record.data) else {
            continue;
        };
        let matches = matches!(
            node.props.get("wall_id"),
            Some(hypernode::PropValue::I64(v)) if *v as u64 == wall_id.0
        );
        if !matches {
            continue;
        }
        best = Some(node);
    }
    best
}
