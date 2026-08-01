use crate::geom::Vec2;
use crate::particles::{Particle, ParticleTree, PointerKind, TriggerState};
use winit::event::{ElementState, MouseButton, WindowEvent};

use super::InputRouter;
use crate::input::hit_kind::HitKind;
use crate::input::UiEvent;

impl InputRouter {
    pub fn route(&mut self, event: &WindowEvent, tree: &mut ParticleTree) -> Vec<UiEvent> {
        let mut out = Vec::new();
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor = Vec2::new(position.x as f32, position.y as f32);
                let hit = tree.hit_test(self.cursor);
                if hit != self.hovered {
                    if let Some(old) = self.hovered {
                        if let Some(Particle::Trigger(t)) = tree.find_mut(old) {
                            if t.state == TriggerState::Hover {
                                t.state = TriggerState::Idle;
                                tree.mark_paint(old);
                            }
                        }
                    }
                    if let Some(new_id) = hit {
                        if let Some(Particle::Trigger(t)) = tree.find_mut(new_id) {
                            if t.state == TriggerState::Idle {
                                t.state = TriggerState::Hover;
                                tree.mark_paint(new_id);
                            }
                        }
                    }
                    self.hovered = hit;
                }
                if let Some(id) = hit {
                    if matches!(tree.find(id), Some(Particle::Sink(_))) {
                        out.push(UiEvent::SinkPointer {
                            id,
                            pos: self.cursor,
                            kind: PointerKind::Move,
                        });
                    }
                }
            }
            WindowEvent::MouseInput { state, button, .. } => {
                if *button != MouseButton::Left {
                    return out;
                }
                match state {
                    ElementState::Pressed => {
                        let hit = tree.hit_test(self.cursor);
                        self.pressed = hit;
                        let kind = hit.and_then(|id| {
                            tree.find(id).map(|p| match p {
                                Particle::Trigger(t) if t.state != TriggerState::Disabled => {
                                    HitKind::Trigger
                                }
                                Particle::Field(f) if !f.read_only => HitKind::Field,
                                Particle::Sink(_) => HitKind::Sink,
                                _ => HitKind::Other,
                            })
                        });
                        match (hit, kind) {
                            (Some(id), Some(HitKind::Trigger)) => {
                                if let Some(Particle::Trigger(t)) = tree.find_mut(id) {
                                    t.state = TriggerState::Active;
                                    tree.mark_paint(id);
                                }
                            }
                            (Some(id), Some(HitKind::Field)) => {
                                let from = self.focused;
                                if from != Some(id) {
                                    self.blur_current(tree);
                                    if let Some(Particle::Field(f)) = tree.find_mut(id) {
                                        f.begin_edit();
                                    }
                                    self.focused = Some(id);
                                    tree.mark_paint(id);
                                    out.push(UiEvent::FocusChanged { from, to: Some(id) });
                                }
                            }
                            (Some(id), Some(HitKind::Sink)) => {
                                out.push(UiEvent::SinkPointer {
                                    id,
                                    pos: self.cursor,
                                    kind: PointerKind::Down,
                                });
                            }
                            _ => {
                                let from = self.focused;
                                if from.is_some() {
                                    self.blur_current(tree);
                                    self.focused = None;
                                    out.push(UiEvent::FocusChanged { from, to: None });
                                }
                            }
                        }
                    }
                    ElementState::Released => {
                        if let Some(id) = self.pressed.take() {
                            let still = tree.hit_test(self.cursor) == Some(id);
                            if let Some(Particle::Trigger(t)) = tree.find_mut(id) {
                                if still && t.state == TriggerState::Active {
                                    out.push(UiEvent::TriggerFired(id));
                                }
                                t.state = if still {
                                    TriggerState::Hover
                                } else {
                                    TriggerState::Idle
                                };
                                tree.mark_paint(id);
                            }
                            if still && matches!(tree.find(id), Some(Particle::Sink(_))) {
                                out.push(UiEvent::SinkPointer {
                                    id,
                                    pos: self.cursor,
                                    kind: PointerKind::Up,
                                });
                            }
                        }
                    }
                }
            }
            WindowEvent::KeyboardInput { event, .. } => {
                out.extend(self.handle_key(event, tree));
            }
            WindowEvent::ModifiersChanged(mods) => {
                self.modifiers_shift = mods.state().shift_key();
            }
            _ => {}
        }
        out
    }
}
