use crate::geom::{Rect, Vec2};
use crate::page_tree::{PageSeamId, PageTree};

use super::{split_rect, PodTree, SeamDirection, SeamDrawCmd};

pub(crate) fn rebuild_seams(pods: &PodTree, area: Rect, out: &mut Vec<SeamDrawCmd>) {
    rebuild_pod_seams(pods, area, out);
}

fn rebuild_pod_seams(pods: &PodTree, area: Rect, out: &mut Vec<SeamDrawCmd>) {
    match pods {
        PodTree::Leaf { .. } => {}
        PodTree::Split {
            direction,
            ratio,
            first,
            second,
        } => {
            let (a, b) = split_rect(area, *direction, *ratio);
            let (start, end) = seam_endpoints(*direction, a);
            out.push(SeamDrawCmd {
                start,
                end,
                direction: *direction,
                hovered: false,
                dragging: false,
                is_page_seam: false,
                page_seam_id: None,
                split_area: area,
            });
            rebuild_pod_seams(first, a, out);
            rebuild_pod_seams(second, b, out);
        }
    }
}

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
                is_page_seam: true,
                page_seam_id: Some(id),
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
