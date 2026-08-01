use crate::particles::{Particle, ParticleId};

pub(crate) fn find_recursive(particle: &Particle, id: ParticleId) -> Option<&Particle> {
    if particle.id() == id {
        return Some(particle);
    }
    match particle {
        Particle::Surface(p) => p.child.as_ref().and_then(|c| find_recursive(c, id)),
        Particle::Stack(p) => {
            for child in &p.children {
                if let Some(found) = find_recursive(child, id) {
                    return Some(found);
                }
            }
            None
        }
        Particle::Slot(p) => p.child.as_ref().and_then(|c| find_recursive(c, id)),
        Particle::Sink(p) => p.child.as_ref().and_then(|c| find_recursive(c, id)),
        Particle::View(p) => p.child.as_ref().and_then(|c| find_recursive(c, id)),
        _ => None,
    }
}

pub(crate) fn find_mut_recursive(particle: &mut Particle, id: ParticleId) -> Option<&mut Particle> {
    if particle.id() == id {
        return Some(particle);
    }
    match particle {
        Particle::Surface(p) => p.child.as_mut().and_then(|c| find_mut_recursive(c, id)),
        Particle::Stack(p) => {
            for child in &mut p.children {
                if let Some(found) = find_mut_recursive(child, id) {
                    return Some(found);
                }
            }
            None
        }
        Particle::Slot(p) => p.child.as_mut().and_then(|c| find_mut_recursive(c, id)),
        Particle::Sink(p) => p.child.as_mut().and_then(|c| find_mut_recursive(c, id)),
        Particle::View(p) => p.child.as_mut().and_then(|c| find_mut_recursive(c, id)),
        _ => None,
    }
}
