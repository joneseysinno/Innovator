use crate::workspace::header::HEADER_HEIGHT;
use crate::workspace::tab_strip::TAB_STRIP_HEIGHT;
use hyper_ui::layout::{arrange_particle, LayoutBox};
use hyper_ui::particles::Particle;
use hyper_ui::{Rect, Vec2};

/// Pin tab strip (+ optional header) to fixed chrome heights.
pub fn sync_chrome_layouts(root: &mut Particle, window: Rect, has_header: bool) {
    let Particle::Surface(surface) = root else {
        return;
    };
    let Some(Particle::Stack(column)) = surface.child.as_deref_mut() else {
        return;
    };

    let mut y = window.origin.y;
    if let Some(tabs) = column.children.first_mut() {
        let r = Rect::from_xywh(window.origin.x, y, window.size.x, TAB_STRIP_HEIGHT);
        tabs.set_layout(LayoutBox {
            origin: r.origin,
            size: Vec2::new(r.size.x, TAB_STRIP_HEIGHT),
        });
        arrange_particle(tabs, r);
        y += TAB_STRIP_HEIGHT;
    }

    if has_header {
        if let Some(header) = column.children.get_mut(1) {
            // Skip if index 1 is the pages view (shouldn't happen when has_header)
            if !matches!(header, Particle::View(_)) {
                let r = Rect::from_xywh(window.origin.x, y, window.size.x, HEADER_HEIGHT);
                header.set_layout(LayoutBox {
                    origin: r.origin,
                    size: Vec2::new(r.size.x, HEADER_HEIGHT),
                });
                arrange_particle(header, r);
            }
        }
    }
}
