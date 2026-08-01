use crate::seam::SeamDrawCmd;

use super::SeamRenderer;

impl SeamRenderer {
    pub fn draw_commands(&self) -> &[SeamDrawCmd] {
        &self.seams
    }
}
