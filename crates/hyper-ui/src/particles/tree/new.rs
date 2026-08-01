use crate::particles::{DirtyFlags, Particle};

use super::ParticleTree;

impl ParticleTree {
    pub fn new(root: Particle) -> Self {
        let mut tree = Self {
            root: Some(root),
            dirty: DirtyFlags::default(),
            generation: 1,
        };
        tree.mark_all_dirty();
        tree
    }
}
