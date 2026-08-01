use super::ParticleTree;

impl ParticleTree {
    pub fn mark_all_dirty(&mut self) {
        self.dirty.layout_all = true;
        self.dirty.paint_all = true;
        self.dirty.text_all = true;
        self.generation = self.generation.wrapping_add(1);
    }
}
