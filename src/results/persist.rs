use crate::results::space::RESULTS_SPACE;
use hypernode::Node;
use infinite_db::infinitedb_core::address::DimensionVector;
use infinite_db::InfiniteDb;

/// Persist a ResultsNode (keyed by wall_id in the point).
pub fn persist_results(db: &mut InfiniteDb, node: &Node) -> std::io::Result<()> {
    let wall_id = match node.props.get("wall_id") {
        Some(hypernode::PropValue::I64(v)) => *v as u32,
        _ => node.id.0 as u32,
    };
    let data = serde_json::to_vec(node)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    let point = DimensionVector::new(vec![wall_id, 0]);
    db.insert(RESULTS_SPACE, point, data)?;
    db.flush(RESULTS_SPACE)?;
    Ok(())
}
