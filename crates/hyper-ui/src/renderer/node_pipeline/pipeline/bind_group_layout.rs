use super::NodePipeline;

impl NodePipeline {
    // silence unused field warning for layout kept for future shader variants
    #[allow(dead_code)]
    pub fn bind_group_layout(&self) -> &wgpu::BindGroupLayout {
        &self.bind_group_layout
    }
}
