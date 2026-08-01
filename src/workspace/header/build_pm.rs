use super::build::WorkspaceHeader;
use hyper_ui::particles::{Particle, SourceParticle, StackParticle, SurfaceParticle};
use std::collections::HashMap;

/// Light PM header — title + status, no wall analysis actions.
pub fn build_pm_header() -> WorkspaceHeader {
    let title = SourceParticle::new("Project Management").with_weight(500);
    let status = SourceParticle::secondary("Ready");
    let status_id = status.id;

    let row = StackParticle::row(vec![Particle::Source(title), Particle::Source(status)])
        .with_gap(12.0);

    let particle = Particle::Surface(
        SurfaceParticle::new([0.16, 0.17, 0.20, 1.0])
            .with_padding(4.0)
            .with_radius(0.0)
            .with_border([0.28, 0.30, 0.34, 1.0], 1.0)
            .with_child(Particle::Stack(row)),
    );

    WorkspaceHeader {
        particle,
        triggers: HashMap::new(),
        status_id,
    }
}
