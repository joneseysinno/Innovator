use crate::devtools::build_overlay;
use crate::workspace::app_shell::page_context_menu::build_page_context_menu;
use crate::workspace::app_shell::page_template_menu::build_page_template_menu;
use crate::workspace::app_shell::AppShell;
use crate::workspace::tab_strip::build_tab_strip;
use hyper_ui::layout::LayoutBox;
use hyper_ui::particles::{
    Particle, SourceParticle, StackParticle, SurfaceParticle, ViewParticle,
};
use hyper_ui::Rect;

/// Rebuild the full particle tree from tab strip + active workspace host.
///
/// Root column: `[ tab_strip, workspace_host, …overlays ]`.
/// Workspace host fills the window under the tab strip and owns header + pages.
pub fn build_tree(shell: &mut AppShell) -> Particle {
    let tabs: Vec<_> = shell.workspaces.iter().map(|w| w.tab()).collect();
    let active_id = shell
        .active()
        .map(|w| w.id())
        .unwrap_or(crate::workspace::WorkspaceId(0));
    shell.tab_strip = build_tab_strip(&tabs, active_id);

    shell.has_header = shell.active().and_then(|a| a.header()).is_some();

    let header = shell
        .active()
        .and_then(|a| a.header())
        .map(|h| h.particle.clone());

    let body = {
        let idx = shell.workspaces.iter().position(|w| w.is_active());
        match idx {
            Some(idx) if shell.workspaces[idx].graph_view().is_some() => {
                crate::workspace::workspace::Workspace::build_graph_workspace_content(
                    &mut shell.workspaces,
                    idx,
                    &shell.graph,
                    shell.root_id,
                )
            }
            Some(idx) => shell.workspaces[idx].build_content(&shell.graph),
            None => empty_body(),
        }
    };

    let workspace = wrap_workspace_host(header, body);

    let mut column = vec![shell.tab_strip.particle.clone(), workspace];

    if let Some(menu) = shell.pending_context_menu.clone() {
        let built = build_page_context_menu(&menu);
        shell.context_menu_triggers = built.triggers;
        let mut particle = built.particle;
        let rect = Rect::from_xywh(menu.cursor.x, menu.cursor.y, 180.0, 200.0);
        particle.set_layout(LayoutBox {
            origin: rect.origin,
            size: rect.size,
        });
        column.push(particle);
    } else if let Some(menu) = shell.pending_template_menu.clone() {
        let built = build_page_template_menu(&menu);
        shell.context_menu_triggers = built.triggers;
        let mut particle = built.particle;
        let rect = Rect::from_xywh(menu.cursor.x, menu.cursor.y, 160.0, 120.0);
        particle.set_layout(LayoutBox {
            origin: rect.origin,
            size: rect.size,
        });
        column.push(particle);
    }

    if shell.overlay_open {
        column.push(build_overlay(shell));
    }

    Particle::Surface(
        SurfaceParticle::new([0.10, 0.11, 0.13, 1.0])
            .with_padding(0.0)
            .with_radius(0.0)
            .with_child(Particle::Stack(
                StackParticle::column(column).with_gap(0.0),
            )),
    )
}

/// Workspace host: fills under the tab strip. Optional header + pages/body inside.
fn wrap_workspace_host(header: Option<Particle>, body: Particle) -> Particle {
    let inner = match header {
        Some(header) => Particle::Stack(
            StackParticle::column(vec![header, body]).with_gap(0.0),
        ),
        None => body,
    };
    let mut host = ViewParticle::new("workspace");
    host.child = Some(Box::new(inner));
    Particle::View(host)
}

fn empty_body() -> Particle {
    Particle::Source(SourceParticle::secondary("No active workspace"))
}
