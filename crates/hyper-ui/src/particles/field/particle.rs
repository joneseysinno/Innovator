mod background_color;
mod backspace;
mod begin_edit;
mod commit;
mod constructors;
mod measure;
mod push_char;
mod revert;

use crate::layout::LayoutBox;
use crate::particles::ParticleId;

use super::{FieldState, FieldValue};

#[derive(Debug, Clone)]
pub struct FieldParticle {
    pub id: ParticleId,
    pub layout: LayoutBox,
    pub committed_value: FieldValue,
    pub edit_buffer: String,
    pub state: FieldState,
    pub read_only: bool,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub flex: f32,
    pub fixed_width: Option<f32>,
}

impl FieldParticle {
    pub fn display_text(&self) -> &str {
        match self.state {
            FieldState::Editing | FieldState::Invalid => &self.edit_buffer,
            FieldState::Idle => {
                // edit_buffer kept in sync on commit
                &self.edit_buffer
            }
        }
    }
}
