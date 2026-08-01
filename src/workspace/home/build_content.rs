use super::HomeWorkspace;
use crate::workspace::app_signal::AppSignal;
use crate::workspace::kind::WorkspaceKind;
use hyper_ui::particles::{
    Particle, SourceParticle, StackParticle, SurfaceParticle, TriggerParticle,
};
use std::collections::HashMap;

/// Build the Home dashboard and wire OpenWorkspace actions onto the workspace.
pub fn build_content(ws: &mut HomeWorkspace) -> Particle {
    let mut actions = HashMap::new();

    let title = SourceParticle::new("Innovator").with_weight(500);
    let subtitle = SourceParticle::secondary("Home dashboard — open a workspace to begin");

    let analysis = TriggerParticle::primary(WorkspaceKind::Analysis.default_title());
    actions.insert(
        analysis.id,
        AppSignal::OpenWorkspace(WorkspaceKind::Analysis),
    );

    let pm = TriggerParticle::new(WorkspaceKind::PM.default_title());
    actions.insert(pm.id, AppSignal::OpenWorkspace(WorkspaceKind::PM));

    let actions_row = StackParticle::row(vec![
        Particle::Trigger(analysis),
        Particle::Trigger(pm),
    ])
    .with_gap(12.0);

    let body = StackParticle::column(vec![
        Particle::Source(title),
        Particle::Source(subtitle),
        Particle::Stack(actions_row),
    ])
    .with_gap(14.0);

    ws.actions = actions;

    Particle::Surface(
        SurfaceParticle::new([0.12, 0.13, 0.16, 1.0])
            .with_padding(24.0)
            .with_radius(0.0)
            .with_border([0.26, 0.28, 0.32, 1.0], 1.0)
            .with_child(Particle::Stack(body)),
    )
}
