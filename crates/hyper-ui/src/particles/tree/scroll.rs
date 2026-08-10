use crate::container::ContainerId;
use crate::particles::{Particle, ParticleId};

use super::ParticleTree;

impl ParticleTree {
    /// Apply a scroll delta to a viewport particle. Returns true if handled.
    pub fn scroll_viewport_by(&mut self, id: ParticleId, delta: f32) -> bool {
        let Some(Particle::Viewport(vp)) = self.find_mut(id) else {
            return false;
        };
        vp.scroll_by(delta);
        self.mark_layout(id);
        true
    }

    /// Scroll a viewport so `container` aligns to the top/start.
    pub fn scroll_to_container(&mut self, viewport_id: ParticleId, container: ContainerId) -> bool {
        let Some(Particle::Viewport(vp)) = self.find_mut(viewport_id) else {
            return false;
        };
        if !vp.scroll_to_container(container) {
            return false;
        }
        self.mark_layout(viewport_id);
        true
    }
}
