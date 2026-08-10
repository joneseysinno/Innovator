use crate::workspace::app_shell::layout_areas::layout_areas;
use crate::workspace::header::HEADER_HEIGHT;
use crate::workspace::tab_strip::TAB_STRIP_HEIGHT;
use hyper_ui::layout::{arrange_particle, LayoutBox};
use hyper_ui::particles::Particle;
use hyper_ui::{Rect, Vec2};

/// Pin app tab strip and active workspace host so the workspace fills the window
/// under the tabs. Header (workspace chrome) and pages live inside that host.
pub fn sync_chrome_layouts(root: &mut Particle, window: Rect, has_header: bool) {
    let areas = layout_areas(window, has_header);

    let Particle::Surface(surface) = root else {
        return;
    };
    let Some(Particle::Stack(column)) = surface.child.as_deref_mut() else {
        return;
    };

    // App chrome: tab strip.
    if let Some(tabs) = column.children.first_mut() {
        let r = areas.tabs;
        tabs.set_layout(LayoutBox {
            origin: r.origin,
            size: Vec2::new(r.size.x, TAB_STRIP_HEIGHT.min(r.size.y)),
        });
        arrange_particle(tabs, r);
    }

    // Workspace host — must fill window under the tab strip.
    let Some(workspace) = column.children.get_mut(1) else {
        return;
    };
    if !matches!(workspace, Particle::View(_)) {
        return;
    }
    workspace.set_layout(LayoutBox {
        origin: areas.workspace.origin,
        size: areas.workspace.size,
    });
    arrange_particle(workspace, areas.workspace);

    // Inside the host: pin header + page region when header is present.
    let Particle::View(host) = workspace else {
        return;
    };
    let Some(inner) = host.child.as_deref_mut() else {
        return;
    };

    if has_header {
        let Particle::Stack(ws_col) = inner else {
            return;
        };
        if let Some(header) = ws_col.children.get_mut(0) {
            if let Some(header_rect) = areas.header {
                header.set_layout(LayoutBox {
                    origin: header_rect.origin,
                    size: Vec2::new(header_rect.size.x, HEADER_HEIGHT.min(header_rect.size.y)),
                });
                arrange_particle(header, header_rect);
            }
        }
        if let Some(pages) = ws_col.children.get_mut(1) {
            pages.set_layout(LayoutBox {
                origin: areas.pages.origin,
                size: areas.pages.size,
            });
            arrange_particle(pages, areas.pages);
        }
    } else {
        // No header: body (pages View / Home View) already arranged with the host.
        // Re-pin explicitly so measure/arrange mismatches cannot leave a short body.
        inner.set_layout(LayoutBox {
            origin: areas.pages.origin,
            size: areas.pages.size,
        });
        arrange_particle(inner, areas.pages);
    }
}
