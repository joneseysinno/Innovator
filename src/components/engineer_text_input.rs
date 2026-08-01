use hyper_ui::particles::{
    FieldParticle, Particle, ParticleId, SourceParticle, StackParticle, SurfaceParticle,
};

/// Label + text field + optional unit (for name / custom text props).
pub fn engineer_text_input(label: &str, value: &str, unit: &str) -> EngineerTextInput {
    let label_p = SourceParticle::secondary(label);
    let mut field = FieldParticle::text(value);
    field.flex = 1.0;
    let unit_p = SourceParticle::muted(unit);

    let label_id = label_p.id;
    let field_id = field.id;
    let unit_id = unit_p.id;

    let mut row_children = vec![Particle::Source(label_p), Particle::Field(field)];
    if !unit.is_empty() {
        row_children.push(Particle::Source(unit_p));
    }

    let row = StackParticle::row(row_children).with_gap(10.0);
    let surface = SurfaceParticle::new([0.0, 0.0, 0.0, 0.0])
        .with_padding(0.0)
        .with_radius(0.0)
        .with_child(Particle::Stack(row));

    EngineerTextInput {
        particle: Particle::Surface(surface),
        label_id,
        field_id,
        unit_id,
    }
}

#[derive(Debug, Clone)]
pub struct EngineerTextInput {
    pub particle: Particle,
    pub label_id: ParticleId,
    pub field_id: ParticleId,
    pub unit_id: ParticleId,
}

impl EngineerTextInput {
    pub fn into_particle(self) -> Particle {
        self.particle
    }
}
