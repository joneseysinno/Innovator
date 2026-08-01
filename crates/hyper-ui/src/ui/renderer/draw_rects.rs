use super::UiRenderer;

impl UiRenderer {
    pub fn draw_rects<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>) {
        self.rects.draw(pass);
    }
}
