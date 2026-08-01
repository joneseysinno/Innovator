use crate::particles::ParticleId;

use super::ParticleTree;

impl ParticleTree {
    pub fn mark_layout(&mut self, id: ParticleId) {
        self.dirty.layout.push(id);
        self.dirty.layout_all = true; // subtree invalidation for now
    }
}
