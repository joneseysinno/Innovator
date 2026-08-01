use crate::geom::Rect;
use crate::layout::LayoutEngine;

use super::UiRenderer;

impl UiRenderer {
    pub fn layout(&mut self, viewport: Rect) {
        if let Some(root) = self.tree.root.as_mut() {
            LayoutEngine::layout(root, viewport);
        }
        self.tree.clear_dirty();
    }
}
