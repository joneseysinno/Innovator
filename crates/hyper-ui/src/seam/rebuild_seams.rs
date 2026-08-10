use crate::container::Visibility;
use crate::geom::{Rect, Vec2};
use crate::page::{PageSeamId, PageTree};

use super::{SeamDirection, SeamDrawCmd};

/// Rebuild seams between adjacent **Shown** page rects (no stored ratio).
pub(crate) fn rebuild_page_seams(pages: &PageTree, area: Rect, out: &mut Vec<SeamDrawCmd>) {
    let leaves = pages.leaves();
    let shown: Vec<_> = leaves
        .iter()
        .filter(|p| p.state.resolved() == Visibility::Shown)
        .collect();
    if shown.len() < 2 {
        return;
    }

    let widths: Vec<f32> = shown
        .iter()
        .map(|p| {
            let w = p.state.rect().size.x;
            if w > 1.0 {
                w
            } else {
                p.state.extent.min.max(1.0)
            }
        })
        .collect();
    let sum: f32 = widths.iter().sum::<f32>().max(1.0);
    let scale = area.size.x / sum;

    let mut x = area.origin.x;
    for i in 0..shown.len() - 1 {
        let w = widths[i] * scale;
        let second_w = widths[i + 1] * scale;
        let first = Rect::from_xywh(x, area.origin.y, w, area.size.y);
        let split_area = Rect::from_xywh(x, area.origin.y, w + second_w, area.size.y);
        let (start, end) = seam_endpoints(SeamDirection::Vertical, first);
        out.push(SeamDrawCmd {
            start,
            end,
            direction: SeamDirection::Vertical,
            hovered: false,
            dragging: false,
            seam_id: PageSeamId(i as u32),
            split_area,
        });
        x += w;
    }
}

fn seam_endpoints(direction: SeamDirection, first: Rect) -> (Vec2, Vec2) {
    match direction {
        SeamDirection::Vertical => (
            Vec2::new(first.origin.x + first.size.x, first.origin.y),
            Vec2::new(
                first.origin.x + first.size.x,
                first.origin.y + first.size.y,
            ),
        ),
        SeamDirection::Horizontal => (
            Vec2::new(first.origin.x, first.origin.y + first.size.y),
            Vec2::new(
                first.origin.x + first.size.x,
                first.origin.y + first.size.y,
            ),
        ),
    }
}
