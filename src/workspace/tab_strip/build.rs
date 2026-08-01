use crate::workspace::app_signal::AppSignal;
use crate::workspace::tab::WorkspaceTab;
use crate::workspace::workspace_id::WorkspaceId;
use hyper_ui::particles::{
    Particle, ParticleId, StackParticle, SurfaceParticle, TriggerParticle, TriggerState,
};
use std::collections::HashMap;

/// Tab strip particle subtree and trigger → app signal map.
#[derive(Debug, Clone)]
pub struct TabStripIO {
    pub particle: Particle,
    pub triggers: HashMap<ParticleId, AppSignal>,
}

/// Build `[ tab… ] [ + ]` row. Active tab uses primary styling.
pub fn build_tab_strip(tabs: &[WorkspaceTab], active: WorkspaceId) -> TabStripIO {
    let mut triggers = HashMap::new();
    let mut children = Vec::new();

    for tab in tabs {
        let mut t = if tab.id == active {
            TriggerParticle::primary(tab.title.clone())
        } else {
            TriggerParticle::new(tab.title.clone())
        };
        if tab.id == active {
            t.state = TriggerState::Idle;
        }
        triggers.insert(t.id, AppSignal::SelectWorkspace(tab.id));
        children.push(Particle::Trigger(t));
    }

    let add = TriggerParticle::new("+");
    triggers.insert(add.id, AppSignal::AddWorkspace);
    children.push(Particle::Trigger(add));

    let row = StackParticle::row(children).with_gap(4.0);
    let particle = Particle::Surface(
        SurfaceParticle::new([0.14, 0.15, 0.18, 1.0])
            .with_padding(2.0)
            .with_radius(0.0)
            .with_border([0.24, 0.26, 0.30, 1.0], 1.0)
            .with_child(Particle::Stack(row)),
    );

    TabStripIO { particle, triggers }
}
