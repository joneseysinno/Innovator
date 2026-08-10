use crate::geom::{Rect, Vec2};
use crate::layout::Axis;
use crate::particles::Particle;

use super::arrange_stack::arrange_stack;
use super::arrange_surface::arrange_surface;
use super::measure_particle;
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
        Particle::Viewport(p) => {
            let unbounded = match p.axis {
                Axis::Vertical => Vec2::new(rect.size.x, 1_000_000.0),
                Axis::Horizontal => Vec2::new(1_000_000.0, rect.size.y),
            };
            let child_size = p
                .child
                .as_ref()
                .map(|c| measure_particle(c, unbounded))
                .unwrap_or(Vec2::ZERO);
            p.content_extent = match p.axis {
                Axis::Vertical => child_size.y,
                Axis::Horizontal => child_size.x,
            };
            p.clamp_offset();
            if let Some(child) = p.child.as_mut() {
                let child_rect = match p.axis {
                    Axis::Vertical => Rect::from_xywh(
                        rect.origin.x,
                        rect.origin.y - p.offset,
                        rect.size.x,
                        child_size.y.max(rect.size.y),
                    ),
                    Axis::Horizontal => Rect::from_xywh(
                        rect.origin.x - p.offset,
                        rect.origin.y,
                        child_size.x.max(rect.size.x),
                        rect.size.y,
                    ),
                };
                arrange_particle(child, child_rect);
            }
        }
        _ => {}
    }
}
