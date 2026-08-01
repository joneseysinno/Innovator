use crate::geom::WorldRect;

use super::{EdgeDrawCmd, SceneNode, SpatialSource};

/// Simple in-memory spatial source for demos and tests.
#[derive(Debug, Default, Clone)]
pub struct InMemorySpatial {
    pub nodes: Vec<SceneNode>,
    pub edges: Vec<EdgeDrawCmd>,
}

impl SpatialSource for InMemorySpatial {
    fn query_nodes_in_rect(&self, rect: WorldRect) -> Vec<SceneNode> {
        self.nodes
            .iter()
            .filter(|n| rect.contains(n.world_pos))
            .cloned()
            .collect()
    }

    fn query_edges_for_visible(&self, rect: WorldRect) -> Vec<EdgeDrawCmd> {
        self.edges
            .iter()
            .filter(|e| {
                let p0 = [e.p0[0] as f64, e.p0[1] as f64];
                let p3 = [e.p3[0] as f64, e.p3[1] as f64];
                // edges are stored in screen space after camera transform in upload path;
                // for world-space demos, store world coords and transform in cull_and_upload.
                rect.contains(p0) || rect.contains(p3)
            })
            .cloned()
            .collect()
    }
}
