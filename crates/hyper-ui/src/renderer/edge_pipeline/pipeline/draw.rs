use super::EdgePipeline;

impl EdgePipeline {
    pub fn draw<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>) {
        if self.instances.is_empty() {
            return;
        }
        // 33 samples * 2 verts = 66 verts per strip instance
        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.bind_group, &[]);
        pass.set_vertex_buffer(0, self.instance_buf.slice(..));
        pass.draw(0..66, 0..self.instances.len() as u32);
    }
}
