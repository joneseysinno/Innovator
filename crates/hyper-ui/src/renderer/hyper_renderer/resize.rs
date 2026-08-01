use crate::geom::UVec2;
use winit::dpi::PhysicalSize;

use super::HyperRenderer;

impl HyperRenderer {
    pub fn resize(&mut self, size: PhysicalSize<u32>) {
        if size.width == 0 || size.height == 0 {
            return;
        }
        self.config.width = size.width;
        self.config.height = size.height;
        self.surface.configure(&self.device, &self.config);
        self.scene
            .camera
            .set_screen_size(UVec2::new(size.width, size.height));
        self.text.resize(size.width, size.height);
    }
}
