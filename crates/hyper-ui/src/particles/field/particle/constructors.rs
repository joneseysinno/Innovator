use crate::layout::LayoutBox;
use crate::particles::ParticleId;

use super::super::{FieldState, FieldValue};
use super::FieldParticle;

impl FieldParticle {
    pub fn f64(value: f64) -> Self {
        let committed = FieldValue::F64(value);
        Self {
            id: ParticleId::fresh(),
            layout: LayoutBox::default(),
            edit_buffer: committed.display(),
            committed_value: committed,
            state: FieldState::Idle,
            read_only: false,
            min: None,
            max: None,
            flex: 1.0,
            fixed_width: None,
        }
    }

    pub fn text(value: impl Into<String>) -> Self {
        let s = value.into();
        Self {
            id: ParticleId::fresh(),
            layout: LayoutBox::default(),
            edit_buffer: s.clone(),
            committed_value: FieldValue::Text(s),
            state: FieldState::Idle,
            read_only: false,
            min: None,
            max: None,
            flex: 1.0,
            fixed_width: None,
        }
    }

    pub fn with_range(mut self, min: f64, max: f64) -> Self {
        self.min = Some(min);
        self.max = Some(max);
        self
    }

    pub fn read_only(mut self) -> Self {
        self.read_only = true;
        self
    }
}
