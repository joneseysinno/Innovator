use winit::window::CursorIcon;

use crate::seam::SeamDirection;

use super::SeamRenderer;

impl SeamRenderer {
    pub fn cursor_icon(&self) -> Option<CursorIcon> {
        if self.drag.is_some() {
            return self.seams.first().map(|s| match s.direction {
                SeamDirection::Vertical => CursorIcon::ColResize,
                SeamDirection::Horizontal => CursorIcon::RowResize,
            });
        }
        self.seams.iter().find(|s| s.hovered).map(|s| match s.direction {
            SeamDirection::Vertical => CursorIcon::ColResize,
            SeamDirection::Horizontal => CursorIcon::RowResize,
        })
    }
}
