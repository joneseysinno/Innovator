use crate::particles::Particle;

use super::EngineerInput;

impl EngineerInput {
    pub fn into_particle(self) -> Particle {
        self.particle
    }
}
