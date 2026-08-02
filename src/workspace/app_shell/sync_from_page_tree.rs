use crate::domains::structural::StructuralWorkspace;
use hyper_ui::layout::{arrange_particle, LayoutBox};
use hyper_ui::particles::Particle;
use hyper_ui::{IconRailSide, PageId, Rect};

/// Walk the PageTree and assign absolute layouts to page / header / rail / pod particles.
pub fn sync_from_page_tree(
    root: &mut Particle,
    ws: &StructuralWorkspace,
    pages_area: Rect,
) {
    let Some(pages_row) = find_pages_row(root) else {
        return;
    };

    let leaf_rects = ws.page_tree.leaf_rects(pages_area);
    for (page_idx, (page_id, page_rect)) in leaf_rects.iter().enumerate() {
        let Some(page_particle) = pages_row.children.get_mut(page_idx) else {
            continue;
        };
        let Some(page) = ws.page_tree.find(*page_id) else {
            continue;
        };

        page_particle.set_layout(LayoutBox {
            origin: page_rect.origin,
            size: page_rect.size,
        });
        arrange_particle(page_particle, *page_rect);

        sync_page_interior(page_particle, page, *page_rect, ws, *page_id);
    }
}

fn sync_page_interior(
    page_particle: &mut Particle,
    page: &hyper_ui::PageNode,
    page_rect: Rect,
    ws: &StructuralWorkspace,
    page_id: PageId,
) {
    let Particle::Stack(column) = page_particle else {
        return;
    };

    let mut child_idx = 0usize;

    if page.header.is_some() {
        if let Some(header_rect) = page.header_rect(page_rect) {
            if let Some(header) = column.children.get_mut(child_idx) {
                header.set_layout(LayoutBox {
                    origin: header_rect.origin,
                    size: header_rect.size,
                });
                arrange_particle(header, header_rect);
            }
        }
        child_idx += 1;
    }

    let Some(body) = column.children.get_mut(child_idx) else {
        return;
    };

    let content_rect = page.content_rect(page_rect);
    let body_rect = if page.header.is_some() {
        let h = page.header.as_ref().map(|h| h.height).unwrap_or(0.0);
        Rect::from_xywh(
            page_rect.origin.x,
            page_rect.origin.y + h,
            page_rect.size.x,
            (page_rect.size.y - h).max(0.0),
        )
    } else {
        page_rect
    };

    body.set_layout(LayoutBox {
        origin: body_rect.origin,
        size: body_rect.size,
    });
    arrange_particle(body, body_rect);

    let Particle::Stack(body_row) = body else {
        // No rail — body is the content stack directly (shouldn't happen with our builder).
        sync_pod_children(body, page, content_rect, ws, page_id);
        return;
    };

    let mut body_i = 0usize;
    if let Some(rail) = &page.icon_rail {
        if let Some(rail_rect) = page.icon_rail_rect(page_rect) {
            let rail_child_idx = match rail.side {
                IconRailSide::Left => 0,
                IconRailSide::Right => body_row.children.len().saturating_sub(1),
            };
            if let Some(rail_p) = body_row.children.get_mut(rail_child_idx) {
                rail_p.set_layout(LayoutBox {
                    origin: rail_rect.origin,
                    size: rail_rect.size,
                });
                arrange_particle(rail_p, rail_rect);
            }
            body_i = match rail.side {
                IconRailSide::Left => 1,
                IconRailSide::Right => 0,
            };
        }
    }

    if let Some(content) = body_row.children.get_mut(body_i) {
        content.set_layout(LayoutBox {
            origin: content_rect.origin,
            size: content_rect.size,
        });
        arrange_particle(content, content_rect);
        sync_pod_children(content, page, content_rect, ws, page_id);
    }
}

fn sync_pod_children(
    content: &mut Particle,
    page: &hyper_ui::PageNode,
    content_rect: Rect,
    _ws: &StructuralWorkspace,
    _page_id: PageId,
) {
    let leaves = page.pods.layout(content_rect);
    let Particle::Stack(split) = content else {
        // Single empty pod surface.
        if let Some((_, r)) = leaves.first() {
            content.set_layout(LayoutBox {
                origin: r.origin,
                size: r.size,
            });
            arrange_particle(content, *r);
        }
        return;
    };

    // Pod children are in layout order; each may be title-bar + body.
    for (i, (pod_id, rect)) in leaves.iter().enumerate() {
        let Some(child) = split.children.get_mut(i) else {
            continue;
        };
        child.set_layout(LayoutBox {
            origin: rect.origin,
            size: rect.size,
        });
        arrange_particle(child, *rect);

        let collapsed = page
            .pods
            .pods
            .iter()
            .find(|p| p.id == *pod_id)
            .map(|p| p.collapsed)
            .unwrap_or(false);

        if let Particle::Stack(pod_col) = child {
            let title_h = hyper_ui::COLLAPSED_HEIGHT.min(rect.size.y);
            if let Some(title) = pod_col.children.get_mut(0) {
                let title_rect = Rect::from_xywh(rect.origin.x, rect.origin.y, rect.size.x, title_h);
                title.set_layout(LayoutBox {
                    origin: title_rect.origin,
                    size: title_rect.size,
                });
                arrange_particle(title, title_rect);
            }
            if let Some(body) = pod_col.children.get_mut(1) {
                if collapsed {
                    let empty = Rect::from_xywh(rect.origin.x, rect.origin.y + title_h, rect.size.x, 0.0);
                    body.set_layout(LayoutBox {
                        origin: empty.origin,
                        size: empty.size,
                    });
                    arrange_particle(body, empty);
                } else {
                    let body_h = (rect.size.y - title_h).max(0.0);
                    let body_rect =
                        Rect::from_xywh(rect.origin.x, rect.origin.y + title_h, rect.size.x, body_h);
                    body.set_layout(LayoutBox {
                        origin: body_rect.origin,
                        size: body_rect.size,
                    });
                    arrange_particle(body, body_rect);
                }
            }
        }
    }
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
