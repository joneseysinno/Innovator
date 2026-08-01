use crate::workspace::analysis::build_pages::build_pages;
use crate::workspace::app_shell::AppShell;
use crate::workspace::empty::build_content::build_content;
use crate::workspace::instance::WorkspaceInstance;
use crate::workspace::kind::WorkspaceKind;
use crate::workspace::tab_strip::build_tab_strip;
use hyper_ui::particles::{Particle, StackParticle, SurfaceParticle};

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
                build_content()
            }
        }
        Some(WorkspaceKind::Empty | WorkspaceKind::PM | WorkspaceKind::Home) | None => {
            build_content()
        }
    };
    column.push(body);

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
        WorkspaceInstance::Empty(_) => build_content(),
    }
}
