use super::ParticleTree;

impl ParticleTree {
    pub fn clear_dirty(&mut self) {
        self.dirty.clear();
    }
}
