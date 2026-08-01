use super::super::FieldState;
use super::FieldParticle;

impl FieldParticle {
    pub fn backspace(&mut self) {
        if self.read_only || self.state != FieldState::Editing {
            return;
        }
        self.edit_buffer.pop();
    }
}
