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
        Particle::Viewport(p) => {
            // Hits outside the clip rect never reach scrolled-out children.
            let clip = Rect::new(p.layout.origin, p.layout.size);
            if !clip.contains(pos) {
                return None;
            }
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

/// Innermost viewport whose clip rect contains `pos` (for wheel / drag scroll).
pub(crate) fn find_viewport_at(particle: &Particle, pos: Vec2) -> Option<ParticleId> {
    match particle {
        Particle::Viewport(p) => {
            let clip = Rect::new(p.layout.origin, p.layout.size);
            if !clip.contains(pos) {
                return None;
            }
            if let Some(child) = p.child.as_ref() {
                if let Some(id) = find_viewport_at(child, pos) {
                    return Some(id);
                }
            }
            Some(p.id)
        }
        Particle::Stack(p) => {
            for child in p.children.iter().rev() {
                if let Some(id) = find_viewport_at(child, pos) {
                    return Some(id);
                }
            }
            None
        }
        Particle::Surface(p) => p.child.as_ref().and_then(|c| find_viewport_at(c, pos)),
        Particle::Slot(p) => p.child.as_ref().and_then(|c| find_viewport_at(c, pos)),
        Particle::Sink(p) => p.child.as_ref().and_then(|c| find_viewport_at(c, pos)),
        Particle::View(p) => p.child.as_ref().and_then(|c| find_viewport_at(c, pos)),
        _ => None,
    }
}
