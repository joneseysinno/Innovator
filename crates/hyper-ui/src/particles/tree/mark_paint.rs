use crate::particles::ParticleId;

use super::ParticleTree;

impl ParticleTree {
    pub fn mark_paint(&mut self, id: ParticleId) {
        self.dirty.paint.push(id);
    }
}
