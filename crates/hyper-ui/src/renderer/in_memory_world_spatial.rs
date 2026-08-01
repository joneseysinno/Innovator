use crate::geom::WorldRect;

use super::{EdgeDrawCmd, SceneNode, SpatialSource, WorldEdge};

#[derive(Debug, Default, Clone)]
pub struct InMemoryWorldSpatial {
    pub nodes: Vec<SceneNode>,
    pub edges: Vec<WorldEdge>,
}

impl SpatialSource for InMemoryWorldSpatial {
    fn query_nodes_in_rect(&self, rect: WorldRect) -> Vec<SceneNode> {
        self.nodes
            .iter()
            .filter(|n| rect.contains(n.world_pos))
            .cloned()
            .collect()
    }

    fn query_edges_for_visible(&self, rect: WorldRect) -> Vec<EdgeDrawCmd> {
        // Return world-space endpoints packed into EdgeDrawCmd; SceneRenderer transforms.
        self.edges
            .iter()
            .filter(|e| rect.contains(e.source) || rect.contains(e.target))
            .map(|e| EdgeDrawCmd {
                p0: [e.source[0] as f32, e.source[1] as f32],
                p1: [0.0, 0.0],
                p2: [0.0, 0.0],
                p3: [e.target[0] as f32, e.target[1] as f32],
                color: e.color,
                width: e.width,
                arrow: true,
                edge_kind: e.kind,
            })
            .collect()
    }
}
