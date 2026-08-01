use super::HyperRenderer;

impl HyperRenderer {
    pub fn set_clear_color(&mut self, color: wgpu::Color) {
        self.clear_color = color;
    }
}
