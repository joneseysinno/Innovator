use crate::geom::Rect;
use crate::seam::split_rect;

use super::{PageId, PageTree};

impl PageTree {
    /// Collect `(PageId, Rect)` for every leaf by walking `split_rect`.
    pub fn leaf_rects(&self, area: Rect) -> Vec<(PageId, Rect)> {
        let mut out = Vec::new();
        self.collect_rects(area, &mut out);
        out
    }

    fn collect_rects(&self, area: Rect, out: &mut Vec<(PageId, Rect)>) {
        match self {
            Self::Leaf(page) => out.push((page.id, area)),
            Self::Split {
                direction,
                ratio,
                first,
                second,
            } => {
                let (a, b) = split_rect(area, *direction, *ratio);
                first.collect_rects(a, out);
                second.collect_rects(b, out);
            }
        }
    }
}
