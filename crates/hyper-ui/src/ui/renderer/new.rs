use crate::input::InputRouter;
use crate::particles::ParticleTree;
use crate::renderer::node_pipeline::NodePipeline;
use crate::seam::SeamRenderer;

use super::UiRenderer;

impl UiRenderer {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat) -> Self {
        Self {
            rects: NodePipeline::new(device, format),
            focus_ring: NodePipeline::new(device, format),
            tree: ParticleTree::default(),
            input: InputRouter::new(),
            page_seams: SeamRenderer::new(),
            pod_seams: SeamRenderer::new(),
        }
    }
}
