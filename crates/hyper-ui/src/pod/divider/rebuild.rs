use crate::geom::Rect;

use super::super::PodId;
use super::{PodDivider, PodDividerRenderer};

impl PodDividerRenderer {
    /// Replace dividers from a single page layout.
    pub fn rebuild(&mut self, layout: &[(PodId, Rect)], gap: f32, area_height: f32) {
        self.clear();
        self.append(layout, gap, area_height);
    }

    /// Append dividers for one page (multi-page pass).
    pub fn append(&mut self, layout: &[(PodId, Rect)], gap: f32, area_height: f32) {
        if layout.len() < 2 || gap <= 0.0 {
            return;
        }
        let hit = gap.max(4.0);
        for pair in layout.windows(2) {
            let (above_id, above) = pair[0];
            let (below_id, _) = pair[1];
            let y = above.origin.y + above.size.y;
            let rect = Rect::from_xywh(
                above.origin.x,
                y + (gap - hit) * 0.5,
                above.size.x,
                hit,
            );
            self.dividers.push(PodDivider {
                above: above_id,
                below: below_id,
                rect,
                hovered: false,
                dragging: false,
            });
            self.area_heights.push(area_height);
        }
    }
}
