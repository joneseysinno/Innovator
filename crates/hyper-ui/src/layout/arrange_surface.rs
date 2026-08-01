use crate::geom::{Rect, Vec2};
use crate::particles::SurfaceParticle;

use super::{arrange_particle, measure_particle};

pub(super) fn arrange_surface(surface: &mut SurfaceParticle, rect: Rect) {
    let inner = rect.with_padding(surface.padding);
    if let Some(child) = surface.child.as_mut() {
        let desired = measure_particle(child, inner.size);
        let child_rect = Rect::new(
            inner.origin,
            Vec2::new(
                desired.x.min(inner.size.x),
                desired.y.min(inner.size.y).max(inner.size.y), // stretch vertically in forms
            ),
        );
        // Prefer filling the padded area for column content.
        arrange_particle(child, Rect::new(inner.origin, inner.size));
        let _ = child_rect;
    }
}
