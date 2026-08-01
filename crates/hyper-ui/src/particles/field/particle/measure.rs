use crate::geom::Vec2;

use super::FieldParticle;

impl FieldParticle {
    pub fn measure(&self, available: Vec2) -> Vec2 {
        let h = 36.0;
        let w = self.fixed_width.unwrap_or_else(|| {
            if available.x.is_finite() {
                available.x.max(80.0)
            } else {
                120.0
            }
        });
        Vec2::new(w, h)
    }
}
