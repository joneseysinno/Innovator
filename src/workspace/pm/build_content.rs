use hyper_ui::particles::{Particle, SourceParticle, StackParticle, SurfaceParticle};

/// Stub landing for Project Management.
pub fn build_content() -> Particle {
    let title = SourceParticle::new("Project Management").with_weight(500);
    let hint = SourceParticle::secondary("Projects · coming soon");
    let detail = SourceParticle::muted("Track deliverables, schedules, and team assignments here.");

    let body = StackParticle::column(vec![
        Particle::Source(title),
        Particle::Source(hint),
        Particle::Source(detail),
    ])
    .with_gap(8.0);

    Particle::Surface(
        SurfaceParticle::new([0.12, 0.13, 0.16, 1.0])
            .with_padding(16.0)
            .with_radius(0.0)
            .with_border([0.26, 0.28, 0.32, 1.0], 1.0)
            .with_child(Particle::Stack(body)),
    )
}
