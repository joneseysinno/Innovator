use crate::geom::Rect;
use crate::particles::Particle;

use super::arrange_stack::arrange_stack;
use super::arrange_surface::arrange_surface;
use super::LayoutBox;

pub fn arrange_particle(particle: &mut Particle, rect: Rect) {
    particle.set_layout(LayoutBox {
        origin: rect.origin,
        size: rect.size,
    });

    match particle {
        Particle::Surface(p) => arrange_surface(p, rect),
        Particle::Stack(p) => arrange_stack(p, rect),
        Particle::Slot(p) => {
            if let Some(child) = p.child.as_mut() {
                arrange_particle(child, rect);
            }
        }
        Particle::Sink(p) => {
            if let Some(child) = p.child.as_mut() {
                arrange_particle(child, rect);
            }
        }
        Particle::View(p) => {
            if let Some(child) = p.child.as_mut() {
                arrange_particle(child, rect);
            }
        }
        _ => {}
    }
}
