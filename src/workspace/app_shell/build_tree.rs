use crate::workspace::analysis::build_pages::build_pages;
use crate::workspace::app_shell::page_context_menu::build_page_context_menu;
use crate::workspace::app_shell::AppShell;
use crate::workspace::empty::build_content::build_content as build_empty;
use crate::workspace::home::build_content::build_content as build_home;
use crate::workspace::instance::WorkspaceInstance;
use crate::workspace::kind::WorkspaceKind;
use crate::workspace::pm::build_content::build_content as build_pm;
use crate::workspace::tab_strip::build_tab_strip;
use hyper_ui::layout::LayoutBox;
use hyper_ui::particles::{Particle, StackParticle, SurfaceParticle};
use hyper_ui::Rect;

/// Rebuild the full particle tree from tab strip + active workspace.
pub fn build_tree(shell: &mut AppShell) -> Particle {
    let tabs: Vec<_> = shell.workspaces.iter().map(|w| w.tab().clone()).collect();
    shell.tab_strip = build_tab_strip(&tabs, shell.active_id);

    let mut column = vec![shell.tab_strip.particle.clone()];

    let header = shell
        .active()
        .and_then(|a| a.header())
        .map(|h| h.particle.clone());
    if let Some(header) = header {
        column.push(header);
    }

    let kind = shell.active().map(|a| a.kind());
    let body = match kind {
        Some(WorkspaceKind::Analysis) => {
            if let Some(WorkspaceInstance::Analysis(ws)) = shell.active_mut() {
                build_pages(ws)
            } else {
                build_empty()
            }
        }
        Some(WorkspaceKind::Home) => {
            if let Some(WorkspaceInstance::Home(ws)) = shell.active_mut() {
                build_home(ws)
            } else {
                build_empty()
            }
        }
        Some(WorkspaceKind::PM) => build_pm(),
        Some(WorkspaceKind::Empty) | None => build_empty(),
    };
    column.push(body);

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
    }

    shell.has_header = shell
        .active()
        .and_then(|a| a.header())
        .is_some();

    Particle::Surface(
        SurfaceParticle::new([0.10, 0.11, 0.13, 1.0])
            .with_padding(0.0)
            .with_radius(0.0)
            .with_child(Particle::Stack(
                StackParticle::column(column).with_gap(0.0),
            )),
    )
}

/// Helper used when only an immutable borrow of instances is needed for body.
#[allow(dead_code)]
pub fn body_for(instance: &mut WorkspaceInstance) -> Particle {
    match instance {
        WorkspaceInstance::Analysis(ws) => build_pages(ws),
        WorkspaceInstance::Home(ws) => build_home(ws),
        WorkspaceInstance::Pm(_) => build_pm(),
        WorkspaceInstance::Empty(_) => build_empty(),
    }
}
