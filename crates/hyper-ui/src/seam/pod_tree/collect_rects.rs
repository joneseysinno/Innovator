use crate::geom::Rect;
use crate::seam::split_rect;

use super::PodTree;

impl PodTree {
    pub(crate) fn collect_rects(&self, area: Rect, out: &mut Vec<(u32, Rect)>) {
        match self {
            Self::Leaf { id } => out.push((*id, area)),
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
