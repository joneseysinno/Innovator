use super::UiRenderer;

impl UiRenderer {
    pub fn draw_focus_ring<'a>(&'a self, pass: &mut wgpu::RenderPass<'a>) {
        self.focus_ring.draw(pass);
    }
}
