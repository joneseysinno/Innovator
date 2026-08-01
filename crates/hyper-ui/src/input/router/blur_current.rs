use crate::particles::{FieldState, Particle, ParticleTree};

use super::InputRouter;

impl InputRouter {
    pub(crate) fn blur_current(&mut self, tree: &mut ParticleTree) {
        if let Some(id) = self.focused {
            if let Some(Particle::Field(f)) = tree.find_mut(id) {
                if f.state == FieldState::Editing {
                    let _ = f.commit();
                }
                tree.mark_paint(id);
            }
        }
    }
}
