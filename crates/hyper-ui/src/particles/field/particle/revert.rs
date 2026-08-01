use super::super::FieldState;
use super::FieldParticle;

impl FieldParticle {
    pub fn revert(&mut self) {
        self.edit_buffer = self.committed_value.display();
        self.state = FieldState::Idle;
    }
}
