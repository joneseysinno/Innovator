use super::super::FieldState;
use super::FieldParticle;

impl FieldParticle {
    pub fn push_char(&mut self, ch: char) {
        if self.read_only || self.state != FieldState::Editing {
            return;
        }
        self.edit_buffer.push(ch);
    }
}
