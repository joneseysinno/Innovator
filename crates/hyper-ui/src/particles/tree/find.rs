use crate::particles::{Particle, ParticleId};

use super::ParticleTree;

impl ParticleTree {
    pub fn find(&self, id: ParticleId) -> Option<&Particle> {
        self.root.as_ref()?.find(id)
    }
}
