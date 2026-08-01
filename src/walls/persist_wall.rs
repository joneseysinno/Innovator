use crate::walls::space::WALLS_SPACE;
use hypernode::Node;
use infinite_db::infinitedb_core::address::DimensionVector;
use infinite_db::InfiniteDb;

/// Write (or overwrite) a wall node into infinite-db.
pub fn persist_wall(db: &mut InfiniteDb, node: &Node) -> std::io::Result<()> {
    let data = serde_json::to_vec(node)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    let point = DimensionVector::new(vec![node.id.0 as u32, 0]);
    db.insert(WALLS_SPACE, point, data)?;
    db.flush(WALLS_SPACE)?;
    Ok(())
}
