use crate::workspace::page::Page;
use hyper_ui::layout::{arrange_particle, LayoutBox};
use hyper_ui::particles::Particle;
use hyper_ui::Rect;

const LEAF_WALL_LIST: u32 = 0;
const LEAF_WALL_SUMMARY: u32 = 3;
const LEAF_INPUT_FORM: u32 = 4;
const LEAF_WALL_VIEW: u32 = 5;
const LEAF_RESULTS_TABLE: u32 = 6;
const LEAF_STATUS: u32 = 7;

/// Snap page panels and nested pods to leaf rects.
pub fn sync_page_layouts(root: &mut Particle, leaves: &[(u32, Rect)]) {
    let Some(pages_row) = find_pages_row(root) else {
        return;
    };

    for page in Page::all() {
        match page {
            Page::Navigation => {
                sync_two_pod_page(
                    pages_row,
                    page.leaf_id() as usize,
                    leaves,
                    LEAF_WALL_LIST,
                    LEAF_WALL_SUMMARY,
                );
            }
            Page::Analysis => {
                sync_two_pod_page(
                    pages_row,
                    page.leaf_id() as usize,
                    leaves,
                    LEAF_INPUT_FORM,
                    LEAF_WALL_VIEW,
                );
            }
            Page::Results => {
                sync_two_pod_page(
                    pages_row,
                    page.leaf_id() as usize,
                    leaves,
                    LEAF_RESULTS_TABLE,
                    LEAF_STATUS,
                );
            }
        }
    }
}

fn sync_two_pod_page(
    pages_row: &mut hyper_ui::particles::StackParticle,
    page_idx: usize,
    leaves: &[(u32, Rect)],
    leaf_a: u32,
    leaf_b: u32,
) {
    let Some(a) = leaf_rect(leaves, leaf_a) else {
        return;
    };
    let Some(b) = leaf_rect(leaves, leaf_b) else {
        return;
    };
    let page_rect = union_rects(a, b);
    let Some(page) = pages_row.children.get_mut(page_idx) else {
        return;
    };
    page.set_layout(LayoutBox {
        origin: page_rect.origin,
        size: page_rect.size,
    });
    arrange_particle(page, page_rect);

    if let Particle::Stack(split) = page {
        if let Some(first) = split.children.get_mut(0) {
            first.set_layout(LayoutBox {
                origin: a.origin,
                size: a.size,
            });
            arrange_particle(first, a);
        }
        if let Some(second) = split.children.get_mut(1) {
            second.set_layout(LayoutBox {
                origin: b.origin,
                size: b.size,
            });
            arrange_particle(second, b);
        }
    }
}

fn leaf_rect(leaves: &[(u32, Rect)], id: u32) -> Option<Rect> {
    leaves.iter().find(|(lid, _)| *lid == id).map(|(_, r)| *r)
}

fn union_rects(a: Rect, b: Rect) -> Rect {
    let min_x = a.origin.x.min(b.origin.x);
    let min_y = a.origin.y.min(b.origin.y);
    let max_x = (a.origin.x + a.size.x).max(b.origin.x + b.size.x);
    let max_y = (a.origin.y + a.size.y).max(b.origin.y + b.size.y);
    Rect::from_xywh(min_x, min_y, max_x - min_x, max_y - min_y)
}

fn find_pages_row(root: &mut Particle) -> Option<&mut hyper_ui::particles::StackParticle> {
    let Particle::Surface(surface) = root else {
        return None;
    };
    let Some(Particle::Stack(column)) = surface.child.as_deref_mut() else {
        return None;
    };
    let pages_host = column.children.iter_mut().rev().find_map(|c| match c {
        Particle::View(v) => Some(v),
        _ => None,
    })?;
    match pages_host.child.as_deref_mut() {
        Some(Particle::Stack(row)) => Some(row),
        _ => None,
    }
}
