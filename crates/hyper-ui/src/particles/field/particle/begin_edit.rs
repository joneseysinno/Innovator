use super::super::FieldState;
use super::FieldParticle;

impl FieldParticle {
    pub fn begin_edit(&mut self) {
        if self.read_only {
            return;
        }
        self.state = FieldState::Editing;
        self.edit_buffer = self.committed_value.display();
    }
}
