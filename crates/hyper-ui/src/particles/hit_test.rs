use crate::geom::{Rect, Vec2};
use crate::particles::{Particle, ParticleId};

pub(crate) fn hit_test_rev(particle: &Particle, pos: Vec2) -> Option<ParticleId> {
    // reverse paint order — last child first
    match particle {
        Particle::Stack(p) => {
            for child in p.children.iter().rev() {
                if let Some(id) = hit_test_rev(child, pos) {
                    return Some(id);
                }
            }
        }
        Particle::Surface(p) => {
            if let Some(child) = p.child.as_ref() {
                if let Some(id) = hit_test_rev(child, pos) {
                    return Some(id);
                }
            }
        }
        Particle::Slot(p) => {
            if let Some(child) = p.child.as_ref() {
                if let Some(id) = hit_test_rev(child, pos) {
                    return Some(id);
                }
            }
        }
        Particle::Sink(p) => {
            if let Some(child) = p.child.as_ref() {
                if let Some(id) = hit_test_rev(child, pos) {
                    return Some(id);
                }
            }
        }
        Particle::View(p) => {
            if let Some(child) = p.child.as_ref() {
                if let Some(id) = hit_test_rev(child, pos) {
                    return Some(id);
                }
            }
        }
        _ => {}
    }
    let layout = particle.layout();
    let rect = Rect::new(layout.origin, layout.size);
    if rect.contains(pos) && particle.is_interactive() {
        return Some(particle.id());
    }
    None
}
