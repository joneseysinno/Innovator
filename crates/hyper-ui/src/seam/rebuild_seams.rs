use crate::geom::{Rect, Vec2};
use crate::page::{PageSeamId, PageTree};

use super::{split_rect, SeamDirection, SeamDrawCmd};

pub(crate) fn rebuild_page_seams(pages: &PageTree, area: Rect, out: &mut Vec<SeamDrawCmd>) {
    let mut seam_index = 0u32;
    rebuild_page_seams_inner(pages, area, out, &mut seam_index);
}

fn rebuild_page_seams_inner(
    pages: &PageTree,
    area: Rect,
    out: &mut Vec<SeamDrawCmd>,
    seam_index: &mut u32,
) {
    match pages {
        PageTree::Leaf(_) => {}
        PageTree::Split {
            direction,
            ratio,
            first,
            second,
        } => {
            let (a, b) = split_rect(area, *direction, *ratio);
            let (start, end) = seam_endpoints(*direction, a);
            let id = PageSeamId(*seam_index);
            *seam_index += 1;
            out.push(SeamDrawCmd {
                start,
                end,
                direction: *direction,
                hovered: false,
                dragging: false,
                seam_id: id,
                split_area: area,
            });
            rebuild_page_seams_inner(first, a, out, seam_index);
            rebuild_page_seams_inner(second, b, out, seam_index);
        }
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
