use crate::geom::{Rect, Vec2};
use crate::page_tree::PageSeamId;

use super::SeamDirection;

#[derive(Debug, Clone)]
pub struct SeamDrawCmd {
    pub start: Vec2,
    pub end: Vec2,
    pub direction: SeamDirection,
    pub hovered: bool,
    pub dragging: bool,
    /// True = page boundary; supports split/merge context menu.
    pub is_page_seam: bool,
    /// Which split node in [`crate::page_tree::PageTree`].
    pub page_seam_id: Option<PageSeamId>,
    /// Local area of the split this seam divides (for ratio drag).
    pub split_area: Rect,
}

impl SeamDrawCmd {
    pub fn line_rect(&self) -> (Vec2, Vec2) {
        match self.direction {
            SeamDirection::Vertical => {
                let x = self.start.x;
                let y = self.start.y.min(self.end.y);
                let h = (self.end.y - self.start.y).abs().max(1.0);
                (Vec2::new(x, y), Vec2::new(1.0, h))
            }
            SeamDirection::Horizontal => {
                let y = self.start.y;
                let x = self.start.x.min(self.end.x);
                let w = (self.end.x - self.start.x).abs().max(1.0);
                (Vec2::new(x, y), Vec2::new(w, 1.0))
            }
        }
    }

    pub fn hit_rect(&self) -> Rect {
        let (origin, size) = self.line_rect();
        match self.direction {
            SeamDirection::Vertical => Rect::from_xywh(origin.x - 3.0, origin.y, 6.0, size.y),
            SeamDirection::Horizontal => Rect::from_xywh(origin.x, origin.y - 3.0, size.x, 6.0),
        }
    }
}
