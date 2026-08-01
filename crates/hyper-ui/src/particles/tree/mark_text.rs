use crate::particles::ParticleId;

use super::ParticleTree;

impl ParticleTree {
    pub fn mark_text(&mut self, id: ParticleId) {
        self.dirty.text.push(id);
        self.dirty.paint.push(id);
    }
}
