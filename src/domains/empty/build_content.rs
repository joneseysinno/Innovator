use hyper_ui::particles::{Particle, SourceParticle, StackParticle, SurfaceParticle};

/// Placeholder fill when an Empty workspace is active.
pub fn build_content() -> Particle {
    let title = SourceParticle::new("Empty workspace").with_weight(500);
    let hint = SourceParticle::secondary("No header · switch tabs or add Structural Analysis");
    let body = StackParticle::column(vec![Particle::Source(title), Particle::Source(hint)]).with_gap(8.0);
    Particle::Surface(
        SurfaceParticle::new([0.12, 0.13, 0.16, 1.0])
            .with_padding(16.0)
            .with_radius(0.0)
            .with_child(Particle::Stack(body)),
    )
}
