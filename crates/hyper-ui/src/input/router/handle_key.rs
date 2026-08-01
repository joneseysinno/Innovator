use crate::particles::{FieldState, Particle, ParticleTree};
use winit::event::KeyEvent;
use winit::keyboard::{Key, NamedKey};

use super::InputRouter;
use crate::input::UiEvent;

impl InputRouter {
    pub(crate) fn handle_key(&mut self, event: &KeyEvent, tree: &mut ParticleTree) -> Vec<UiEvent> {
        let mut out = Vec::new();
        if !event.state.is_pressed() {
            return out;
        }

        if event.logical_key == Key::Named(NamedKey::Tab) {
            out.extend(self.focus_next(tree, self.modifiers_shift));
            return out;
        }

        let Some(focus) = self.focused else {
            return out;
        };

        // Clone key data then mutate field without overlapping borrows.
        match &event.logical_key {
            Key::Named(NamedKey::Enter) => {
                if let Some(Particle::Field(field)) = tree.find_mut(focus) {
                    if let Some(value) = field.commit() {
                        tree.mark_text(focus);
                        out.push(UiEvent::FieldCommit { id: focus, value });
                    } else {
                        tree.mark_paint(focus);
                    }
                }
            }
            Key::Named(NamedKey::Escape) => {
                if let Some(Particle::Field(field)) = tree.find_mut(focus) {
                    field.revert();
                    tree.mark_text(focus);
                }
            }
            Key::Named(NamedKey::Backspace) => {
                let raw = if let Some(Particle::Field(field)) = tree.find_mut(focus) {
                    if field.state != FieldState::Editing {
                        field.begin_edit();
                    }
                    field.backspace();
                    Some(field.edit_buffer.clone())
                } else {
                    None
                };
                if let Some(raw) = raw {
                    tree.mark_text(focus);
                    out.push(UiEvent::FieldEditing { id: focus, raw });
                }
            }
            Key::Character(s) => {
                let chars: Vec<char> = s.chars().filter(|ch| !ch.is_control()).collect();
                let raw = if let Some(Particle::Field(field)) = tree.find_mut(focus) {
                    if field.state != FieldState::Editing {
                        field.begin_edit();
                    }
                    for ch in chars {
                        field.push_char(ch);
                    }
                    Some(field.edit_buffer.clone())
                } else {
                    None
                };
                if let Some(raw) = raw {
                    tree.mark_text(focus);
                    out.push(UiEvent::FieldEditing { id: focus, raw });
                }
            }
            _ => {}
        }
        out
    }
}
