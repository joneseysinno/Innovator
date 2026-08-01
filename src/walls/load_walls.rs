use crate::walls::space::WALLS_SPACE;
use hypernode::{Graph, Node};
use infinite_db::InfiniteDb;

/// Load all persisted wall nodes into a graph.
pub fn load_walls(db: &mut InfiniteDb) -> Graph {
    let mut graph = Graph::new();
    let Ok(records) = db.query(WALLS_SPACE, None) else {
        return graph;
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
    graph
}
