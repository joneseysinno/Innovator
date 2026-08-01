mod cull_and_upload;
mod draw;
mod new;
mod upload_edge_commands;
mod upload_node_instances;

use super::{EdgePipeline, NodePipeline, SceneCamera};

/// Layer A — hypergraph scene renderer.
pub struct SceneRenderer {
    pub camera: SceneCamera,
    pub nodes: NodePipeline,
    pub edges: EdgePipeline,
    /// When true, EdgeDrawCmd p0/p3 are world coords needing camera transform.
    pub edges_are_world: bool,
}
