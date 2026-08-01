use crate::particles::{Particle, ParticleId};

use super::ParticleTree;

impl ParticleTree {
    pub fn find_mut(&mut self, id: ParticleId) -> Option<&mut Particle> {
        self.root.as_mut()?.find_mut(id)
    }
}
