use super::super::FieldState;
use super::FieldParticle;

impl FieldParticle {
    pub fn background_color(&self, focused: bool) -> [f32; 4] {
        if self.read_only {
            return [0.16, 0.17, 0.20, 1.0];
        }
        match self.state {
            FieldState::Invalid => [0.35, 0.12, 0.12, 1.0],
            FieldState::Editing if focused => [0.14, 0.16, 0.22, 1.0],
            _ if focused => [0.14, 0.16, 0.22, 1.0],
            _ => [0.18, 0.19, 0.22, 1.0],
        }
    }
}
