//! Layer A scene renderer + HyperRenderer bootstrap (winit + wgpu).

pub mod camera;
pub mod edge_pipeline;
pub mod node_pipeline;

mod cull_nodes;
mod frame_ctx;
mod hyper_renderer;
mod in_memory_spatial;
mod in_memory_world_spatial;
mod scene_node;
mod scene_renderer;
mod spatial_source;
mod world_edge;

pub use camera::SceneCamera;
pub use cull_nodes::cull_nodes_from_infinite_db;
pub use edge_pipeline::{EdgeDrawCmd, EdgeKindGpu, EdgePipeline};
pub use frame_ctx::FrameCtx;
pub use hyper_renderer::HyperRenderer;
pub use in_memory_spatial::InMemorySpatial;
pub use in_memory_world_spatial::InMemoryWorldSpatial;
pub use node_pipeline::{NodeInstance, NodePipeline};
pub use scene_node::SceneNode;
pub use scene_renderer::SceneRenderer;
pub use spatial_source::SpatialSource;
pub use world_edge::WorldEdge;
