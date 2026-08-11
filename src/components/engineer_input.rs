use hyper_ui::particles::{
    FieldParticle, Particle, ParticleId, SourceParticle, StackParticle, SurfaceParticle,
};

/// Build an engineer input row: label + numeric field + unit.
///
/// `on_commit` is application-level; the field id can be read from the returned
/// tree so the host can wire `UiEvent::FieldCommit` → Signal hyperedge.
pub fn engineer_input(label: &str, value: f64, unit: &str) -> EngineerInput {
    let label_p = SourceParticle::secondary(label);
    let mut field = FieldParticle::f64(value);
    field.flex = 1.0;
    let unit_p = SourceParticle::muted(unit);

    let label_id = label_p.id;
    let field_id = field.id;
    let unit_id = unit_p.id;

    let row = StackParticle::row(vec![
        Particle::Source(label_p),
        Particle::Field(field),
        Particle::Source(unit_p),
    ])
    .with_gap(10.0);

    let surface = SurfaceParticle::new([0.0, 0.0, 0.0, 0.0])
        .with_padding(0.0)
        .with_radius(0.0)
        .with_child(Particle::Stack(row));

    EngineerInput {
        particle: Particle::Surface(surface),
        label_id,
        field_id,
        unit_id,
    }
}

/// Handle returned by [`engineer_input`] so hosts can bind commit handlers.
#[derive(Debug, Clone)]
pub struct EngineerInput {
    pub particle: Particle,
    pub label_id: ParticleId,
    pub field_id: ParticleId,
    pub unit_id: ParticleId,
}

impl EngineerInput {
    pub fn into_particle(self) -> Particle {
        self.particle
    }
}
