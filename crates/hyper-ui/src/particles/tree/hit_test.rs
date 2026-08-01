use crate::geom::Vec2;
use crate::particles::hit_test::hit_test_rev;
use crate::particles::ParticleId;

use super::ParticleTree;

impl ParticleTree {
    pub fn hit_test(&self, pos: Vec2) -> Option<ParticleId> {
        self.root.as_ref().and_then(|r| hit_test_rev(r, pos))
    }
}
