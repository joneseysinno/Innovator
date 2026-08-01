use crate::walls::default_props::default_wall_props;
use crate::walls::persist_wall::persist_wall;
use hypernode::{Graph, Node, NodeId, SpaceClass};
use infinite_db::InfiniteDb;

/// Create a wall HyperNode, insert into the in-memory graph, and persist to infinite-db.
pub fn new_wall(graph: &mut Graph, db: &mut InfiniteDb, name: impl Into<String>) -> NodeId {
    let name = name.into();
    let id = graph.insert_node(Node {
        id: NodeId(0),
        space_class: SpaceClass::Entity,
        label: name.clone(),
        world_pos: [0.0, 0.0],
        props: default_wall_props(name),
    });
    if let Some(node) = graph.nodes.get_mut(&id) {
        node.world_pos = [id.0 as f64 * 10.0, 0.0];
        let _ = persist_wall(db, node);
    }
    id
}
