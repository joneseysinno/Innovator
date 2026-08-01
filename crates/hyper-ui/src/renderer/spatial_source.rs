use crate::geom::WorldRect;

use super::{EdgeDrawCmd, SceneNode};

/// Spatial source used for frustum culling. Implement for `infinite-db` or in-memory graphs.
pub trait SpatialSource {
    fn query_nodes_in_rect(&self, rect: WorldRect) -> Vec<SceneNode>;
    fn query_edges_for_visible(&self, rect: WorldRect) -> Vec<EdgeDrawCmd>;
}
