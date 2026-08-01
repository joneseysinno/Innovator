use crate::geom::{Rect, Vec2};

use super::{split_rect, PodTree, SeamDirection, SeamDrawCmd};

pub(crate) fn rebuild_seams(pods: &PodTree, area: Rect, out: &mut Vec<SeamDrawCmd>) {
    match pods {
        PodTree::Leaf { .. } => {}
        PodTree::Split {
            direction,
            ratio,
            first,
            second,
        } => {
            let (a, b) = split_rect(area, *direction, *ratio);
            let (start, end) = match direction {
                SeamDirection::Vertical => (
                    Vec2::new(a.origin.x + a.size.x, a.origin.y),
                    Vec2::new(a.origin.x + a.size.x, a.origin.y + a.size.y),
                ),
                SeamDirection::Horizontal => (
                    Vec2::new(a.origin.x, a.origin.y + a.size.y),
                    Vec2::new(a.origin.x + a.size.x, a.origin.y + a.size.y),
                ),
            };
            out.push(SeamDrawCmd {
                start,
                end,
                direction: *direction,
                hovered: false,
                dragging: false,
            });
            rebuild_seams(first, a, out);
            rebuild_seams(second, b, out);
            let _ = b;
        }
    }
}
