use crate::geom::Rect;

use super::PodTree;

impl PodTree {
    pub fn leaf_rects(&self, area: Rect) -> Vec<(u32, Rect)> {
        let mut out = Vec::new();
        self.collect_rects(area, &mut out);
        out
    }
}
