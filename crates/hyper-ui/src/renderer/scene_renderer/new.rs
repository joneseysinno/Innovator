use crate::geom::UVec2;

use super::super::{EdgePipeline, NodePipeline, SceneCamera};
use super::SceneRenderer;

impl SceneRenderer {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat, screen: UVec2) -> Self {
        Self {
            camera: SceneCamera::new(screen),
            nodes: NodePipeline::new(device, format),
            edges: EdgePipeline::new(device, format),
            edges_are_world: true,
        }
    }
}
