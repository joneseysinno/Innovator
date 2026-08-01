use super::SceneRenderer;

impl SceneRenderer {
    pub fn draw<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>) {
        self.edges.draw(pass);
        self.nodes.draw(pass);
    }
}
