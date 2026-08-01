use crate::geom::{Rect, Vec2};
use crate::particles::Particle;

use super::{arrange_particle, measure_particle};

pub struct LayoutEngine;

impl LayoutEngine {
    pub fn layout(root: &mut Particle, viewport: Rect) {
        let desired = measure_particle(root, viewport.size);
        let size = Vec2::new(
            desired.x.min(viewport.size.x).max(0.0),
            // column stacks often want their natural height; clamp to viewport
            desired.y.min(viewport.size.y).max(0.0),
        );
        // For root, fill the viewport so surfaces stretch.
        let final_size = Vec2::new(viewport.size.x, viewport.size.y.max(size.y));
        arrange_particle(root, Rect::new(viewport.origin, final_size));
    }
}
