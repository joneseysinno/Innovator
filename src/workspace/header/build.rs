use crate::workspace::signal::WorkspaceSignal;
use hyper_ui::particles::{
    Particle, ParticleId, SourceParticle, StackParticle, SurfaceParticle, TriggerParticle,
};
use std::collections::HashMap;

/// Per-workspace action header (New Wall / Save / Run / Export).
#[derive(Debug, Clone)]
pub struct WorkspaceHeader {
    pub particle: Particle,
    pub triggers: HashMap<ParticleId, WorkspaceSignal>,
    pub status_id: ParticleId,
}

/// Build the optional workspace header strip.
pub fn build_header() -> WorkspaceHeader {
    let status = SourceParticle::secondary("Ready");
    let status_id = status.id;

    let new_wall = TriggerParticle::new(WorkspaceSignal::NewWall.label());
    let save = TriggerParticle::new(WorkspaceSignal::Save.label());
    let run = TriggerParticle::primary(WorkspaceSignal::RunAnalysis.label());
    let export = TriggerParticle::new(WorkspaceSignal::Export.label());

    let mut triggers = HashMap::new();
    triggers.insert(new_wall.id, WorkspaceSignal::NewWall);
    triggers.insert(save.id, WorkspaceSignal::Save);
    triggers.insert(run.id, WorkspaceSignal::RunAnalysis);
    triggers.insert(export.id, WorkspaceSignal::Export);

    let row = StackParticle::row(vec![
        Particle::Trigger(new_wall),
        Particle::Trigger(save),
        Particle::Trigger(run),
        Particle::Trigger(export),
        Particle::Source(status),
    ])
    .with_gap(10.0);

    let particle = Particle::Surface(
        SurfaceParticle::new([0.16, 0.17, 0.20, 1.0])
            .with_padding(4.0)
            .with_radius(0.0)
            .with_border([0.28, 0.30, 0.34, 1.0], 1.0)
            .with_child(Particle::Stack(row)),
    );

    WorkspaceHeader {
        particle,
        triggers,
        status_id,
    }
}
