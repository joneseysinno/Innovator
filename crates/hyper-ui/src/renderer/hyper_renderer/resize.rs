use crate::geom::UVec2;
use winit::dpi::PhysicalSize;

use super::HyperRenderer;

impl HyperRenderer {
    /// Resize the physical wgpu surface and align UI helpers to logical space.
    pub fn resize(&mut self, size: PhysicalSize<u32>, scale_factor: f32) {
        if size.width == 0 || size.height == 0 {
            return;
        }
        self.config.width = size.width;
        self.config.height = size.height;
        self.surface.configure(&self.device, &self.config);
        self.scene
            .camera
            .set_screen_size(UVec2::new(size.width, size.height));

        let s = scale_factor.max(0.01);
        let logical_w = ((size.width as f32) / s).ceil().max(1.0) as u32;
        let logical_h = ((size.height as f32) / s).ceil().max(1.0) as u32;
        self.text.resize(logical_w, logical_h, s);
        self.ui.input.set_scale_factor(s);
    }
}
