use crate::geom::Vec2;
use crate::particles::Particle;

pub fn measure_particle(particle: &Particle, available: Vec2) -> Vec2 {
    match particle {
        Particle::Surface(p) => p.measure(available),
        Particle::Stack(p) => p.measure(available),
        Particle::Slot(p) => p
            .child
            .as_ref()
            .map(|c| measure_particle(c, available))
            .unwrap_or(Vec2::ZERO),
        Particle::Source(p) => p.measure(available),
        Particle::Field(p) => p.measure(available),
        Particle::Trigger(p) => p.measure(available),
        Particle::Sink(p) => p.measure(available),
        Particle::View(p) => p.measure(available),
        Particle::Signal(_) => Vec2::ZERO,
    }
}
