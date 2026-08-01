use std::sync::Arc;
use winit::window::Window;

use super::HyperRenderer;

impl HyperRenderer {
    pub fn new(window: Arc<Window>) -> Self {
        pollster::block_on(Self::new_async(window))
    }
}
