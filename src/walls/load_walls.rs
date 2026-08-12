use crate::walls::space::WALLS_SPACE;
use hypernode::{Graph, Node};
use infinite_db::InfiniteDb;

/// Load all persisted wall nodes into the composed-view graph.
pub fn load_walls(db: &mut InfiniteDb, graph: &mut Graph) {
    let Ok(records) = db.query(WALLS_SPACE, None) else {
        return;
    };
    for record in records {
        if record.tombstone {
            continue;
        }
        let Ok(node) = serde_json::from_slice::<Node>(&record.data) else {
            continue;
        };
        graph.insert_node(node);
    }
}
