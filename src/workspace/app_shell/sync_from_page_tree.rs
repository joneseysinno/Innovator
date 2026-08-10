use hyper_ui::layout::{arrange_particle, LayoutBox};
use hyper_ui::particles::Particle;
use hyper_ui::{IconRailSide, PageId, PageTree, Pod, Rect};

/// Walk the PageTree and assign absolute layouts to page / header / rail / pod particles.
///
/// Works for Structural (`Stack` page chrome wrapping a `Viewport`) and Placeholder
/// (bare `Viewport` page children).
pub fn sync_from_page_tree(root: &mut Particle, page_tree: &mut PageTree, pages_area: Rect) {
    let Some((pages_row_ptr, content_area)) = find_pages_row(root, pages_area) else {
        return;
    };
    // SAFETY: pages_row_ptr points into `root`, which we uniquely borrow for the rest
    // of this function; no other alias is held.
    let pages_row = unsafe { &mut *pages_row_ptr };

    let leaf_rects = page_tree.leaf_rects(content_area);
    for (page_idx, (page_id, page_rect)) in leaf_rects.into_iter().enumerate() {
        let Some(page_particle) = pages_row.children.get_mut(page_idx) else {
            continue;
        };

        page_particle.set_layout(LayoutBox {
            origin: page_rect.origin,
            size: page_rect.size,
        });
        arrange_particle(page_particle, page_rect);
        sync_page_interior(page_particle, page_id, page_rect, page_tree);
    }
}

fn sync_page_interior(
    page_particle: &mut Particle,
    page_id: PageId,
    page_rect: Rect,
    page_tree: &mut PageTree,
) {
    let header = page_tree.find(page_id).and_then(|p| p.header.clone());
    let icon_rail = page_tree.find(page_id).and_then(|p| p.icon_rail.clone());
    let content_rect = page_tree
        .find(page_id)
        .map(|p| p.content_rect(page_rect))
        .unwrap_or(page_rect);

    // Placeholder pages are a bare Viewport (no header / rail chrome).
    if matches!(page_particle, Particle::Viewport(_)) {
        sync_pod_children(page_particle, page_id, content_rect, page_tree);
        return;
    }

    let Particle::Stack(column) = page_particle else {
        return;
    };

    let mut child_idx = 0usize;

    if header.is_some() {
        if let Some(page) = page_tree.find(page_id) {
            if let Some(header_rect) = page.header_rect(page_rect) {
                if let Some(header_p) = column.children.get_mut(child_idx) {
                    header_p.set_layout(LayoutBox {
                        origin: header_rect.origin,
                        size: header_rect.size,
                    });
                    arrange_particle(header_p, header_rect);
                }
            }
        }
        child_idx += 1;
    }

    let Some(body) = column.children.get_mut(child_idx) else {
        return;
    };

    let body_rect = if header.is_some() {
        let h = header.as_ref().map(|h| h.height).unwrap_or(0.0);
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
        sync_pod_children(body, page_id, content_rect, page_tree);
        return;
    };

    let mut body_i = 0usize;
    if let Some(rail) = &icon_rail {
        if let Some(page) = page_tree.find(page_id) {
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
    }

    if let Some(content) = body_row.children.get_mut(body_i) {
        content.set_layout(LayoutBox {
            origin: content_rect.origin,
            size: content_rect.size,
        });
        sync_pod_children(content, page_id, content_rect, page_tree);
    }
}

fn sync_pod_children(
    content: &mut Particle,
    page_id: PageId,
    content_rect: Rect,
    page_tree: &mut PageTree,
) {
    let Some(page) = page_tree.find_mut(page_id) else {
        return;
    };
    let (leaves, report) = page.pods.layout(content_rect);
    let scroll_offset = page.pods.scroll_offset;
    let anchors: Vec<_> = leaves
        .iter()
        .map(|(id, r)| (Pod::container_id(*id), r.origin.y - content_rect.origin.y))
        .collect();
    let collapsed: Vec<bool> = page.pods.pods.iter().map(|p| p.collapsed).collect();

    // Configure scroll viewport if present.
    let stack = if let Particle::Viewport(vp) = content {
        vp.offset = scroll_offset;
        vp.content_extent = content_rect.size.y + report.scroll_extent;
        vp.set_anchors(anchors);
        vp.clamp_offset();
        page.pods.scroll_offset = vp.offset;
        match vp.child.as_deref_mut() {
            Some(Particle::Stack(s)) => s,
            _ => {
                arrange_particle(content, content_rect);
                return;
            }
        }
    } else if let Particle::Stack(s) = content {
        s
    } else {
        if let Some((_, r)) = leaves.first() {
            content.set_layout(LayoutBox {
                origin: r.origin,
                size: r.size,
            });
            arrange_particle(content, *r);
        }
        return;
    };

    // Lay out pods in content space (origin at content_rect; viewport offsets later).
    for (i, ((_pod_id, rect), is_collapsed)) in leaves.iter().zip(collapsed.iter()).enumerate() {
        let Some(child) = stack.children.get_mut(i) else {
            continue;
        };
        let local = Rect::from_xywh(
            content_rect.origin.x,
            content_rect.origin.y + (rect.origin.y - content_rect.origin.y),
            rect.size.x,
            rect.size.y,
        );
        child.set_layout(LayoutBox {
            origin: local.origin,
            size: local.size,
        });
        arrange_particle(child, local);

        // Structural: bare Stack(title, body). Placeholder: Surface → Stack(title, body).
        let pod_col = match child {
            Particle::Stack(s) => Some(s),
            Particle::Surface(surf) => match surf.child.as_deref_mut() {
                Some(Particle::Stack(s)) => Some(s),
                _ => None,
            },
            _ => None,
        };
        if let Some(pod_col) = pod_col {
            let title_h = hyper_ui::COLLAPSED_HEIGHT.min(rect.size.y);
            if let Some(title) = pod_col.children.get_mut(0) {
                let title_rect =
                    Rect::from_xywh(local.origin.x, local.origin.y, rect.size.x, title_h);
                title.set_layout(LayoutBox {
                    origin: title_rect.origin,
                    size: title_rect.size,
                });
                arrange_particle(title, title_rect);
            }
            if let Some(body) = pod_col.children.get_mut(1) {
                let body_h = if *is_collapsed {
                    0.0
                } else {
                    (rect.size.y - title_h).max(0.0)
                };
                let body_rect =
                    Rect::from_xywh(local.origin.x, local.origin.y + title_h, rect.size.x, body_h);
                body.set_layout(LayoutBox {
                    origin: body_rect.origin,
                    size: body_rect.size,
                });
                arrange_particle(body, body_rect);
            }
        }
    }

    // Final viewport arrange applies scroll offset to the child stack.
    if matches!(content, Particle::Viewport(_)) {
        arrange_particle(content, content_rect);
    }
}

fn find_pages_row(
    root: &mut Particle,
    pages_area: Rect,
) -> Option<(*mut hyper_ui::particles::StackParticle, Rect)> {
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
    let Particle::Stack(outer) = pages_host.child.as_deref_mut()? else {
        return None;
    };

    // [page_rail, pages_row] when Hidden pages exist; else outer is the pages row.
    if outer.children.len() == 2 {
        if let Particle::Surface(rail) = &mut outer.children[0] {
            let rail_w = 34.0;
            rail.layout = LayoutBox {
                origin: pages_area.origin,
                size: hyper_ui::Vec2::new(rail_w, pages_area.size.y),
            };
        }
        let content = Rect::from_xywh(
            pages_area.origin.x + 34.0,
            pages_area.origin.y,
            (pages_area.size.x - 34.0).max(0.0),
            pages_area.size.y,
        );
        if matches!(outer.children.get(1), Some(Particle::Stack(_))) {
            let inner = match &mut outer.children[1] {
                Particle::Stack(s) => s as *mut _,
                _ => unreachable!(),
            };
            return Some((inner, content));
        }
    }
    Some((outer as *mut _, pages_area))
}
