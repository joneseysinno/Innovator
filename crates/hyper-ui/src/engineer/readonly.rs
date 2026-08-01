use crate::particles::field::FieldValue;
use crate::particles::{Particle, SourceParticle, StackParticle, SurfaceParticle};

/// Convenience: build a read-only engineer row (field renders as source).
pub fn engineer_input_readonly(label: &str, value: f64, unit: &str) -> Particle {
    let row = StackParticle::row(vec![
        Particle::Source(SourceParticle::secondary(label)),
        Particle::Source(SourceParticle::new(FieldValue::F64(value).display())),
        Particle::Source(SourceParticle::muted(unit)),
    ])
    .with_gap(10.0);
    Particle::Surface(
        SurfaceParticle::new([0.0, 0.0, 0.0, 0.0])
            .with_padding(0.0)
            .with_child(Particle::Stack(row)),
    )
}
