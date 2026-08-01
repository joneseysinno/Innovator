use super::super::{FieldState, FieldValue};
use super::FieldParticle;

impl FieldParticle {
    pub fn commit(&mut self) -> Option<FieldValue> {
        if self.read_only {
            return None;
        }
        match &self.committed_value {
            FieldValue::F64(_) => match self.edit_buffer.trim().parse::<f64>() {
                Ok(v) => {
                    if self.min.is_some_and(|m| v < m) || self.max.is_some_and(|m| v > m) {
                        self.state = FieldState::Invalid;
                        return None;
                    }
                    let value = FieldValue::F64(v);
                    self.committed_value = value.clone();
                    self.edit_buffer = value.display();
                    self.state = FieldState::Idle;
                    Some(value)
                }
                Err(_) => {
                    self.state = FieldState::Invalid;
                    None
                }
            },
            FieldValue::Text(_) => {
                let value = FieldValue::Text(self.edit_buffer.clone());
                self.committed_value = value.clone();
                self.state = FieldState::Idle;
                Some(value)
            }
            FieldValue::Bool(_) => {
                let value = match self.edit_buffer.trim().to_ascii_lowercase().as_str() {
                    "true" | "1" | "yes" => FieldValue::Bool(true),
                    "false" | "0" | "no" => FieldValue::Bool(false),
                    _ => {
                        self.state = FieldState::Invalid;
                        return None;
                    }
                };
                self.committed_value = value.clone();
                self.edit_buffer = value.display();
                self.state = FieldState::Idle;
                Some(value)
            }
        }
    }
}
