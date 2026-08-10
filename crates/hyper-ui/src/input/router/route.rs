use crate::geom::Vec2;
use crate::layout::InputClass;
use crate::particles::{Particle, ParticleTree, PointerKind, TriggerState};
use winit::event::{ElementState, MouseButton, MouseScrollDelta, TouchPhase, WindowEvent};

use super::InputRouter;
use crate::input::hit_kind::HitKind;
use crate::input::UiEvent;

impl InputRouter {
    pub fn route(&mut self, event: &WindowEvent, tree: &mut ParticleTree) -> Vec<UiEvent> {
        let mut out = Vec::new();
        match event {
            WindowEvent::CursorMoved { position, .. } => {
                self.cursor = Vec2::new(position.x as f32, position.y as f32);
                if let Some((vp_id, last)) = self.scroll_drag {
                    let axis_pos = self.cursor.y;
                    let delta = last - axis_pos;
                    if tree.scroll_viewport_by(vp_id, delta) {
                        self.scroll_drag = Some((vp_id, axis_pos));
                    }
                    return out;
                }
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
            WindowEvent::MouseWheel { delta, .. } => {
                let dy = match delta {
                    MouseScrollDelta::LineDelta(_, y) => y * 40.0,
                    MouseScrollDelta::PixelDelta(p) => p.y as f32,
                };
                // Positive wheel → content moves down (negative offset delta in
                // our "offset reveals lower content" convention: invert).
                if let Some(vp_id) = tree.viewport_at(self.cursor) {
                    tree.scroll_viewport_by(vp_id, -dy);
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
                                Particle::Viewport(_) => HitKind::Viewport,
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
                            (Some(id), Some(HitKind::Viewport)) => {
                                // Pointer drag-scroll always; touch drag also
                                // starts when InputClass is Touch/Hybrid via Touch events.
                                self.scroll_drag = Some((id, self.cursor.y));
                            }
                            _ => {
                                // Touch-class: drag-scroll even when pressing on
                                // non-interactive chrome inside a viewport.
                                if matches!(
                                    self.input_class,
                                    InputClass::Touch | InputClass::Hybrid
                                ) {
                                    if let Some(vp_id) = tree.viewport_at(self.cursor) {
                                        self.scroll_drag = Some((vp_id, self.cursor.y));
                                    }
                                }
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
                        self.scroll_drag = None;
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
            WindowEvent::Touch(touch) => {
                let pos = Vec2::new(touch.location.x as f32, touch.location.y as f32);
                self.cursor = pos;
                // First touch promotes Pointer → Hybrid (never demotes).
                if self.input_class == InputClass::Pointer {
                    self.input_class = InputClass::Hybrid;
                }
                match touch.phase {
                    TouchPhase::Started => {
                        if let Some(vp_id) = tree.viewport_at(pos) {
                            self.scroll_drag = Some((vp_id, pos.y));
                        }
                    }
                    TouchPhase::Moved => {
                        if let Some((vp_id, last)) = self.scroll_drag {
                            let delta = last - pos.y;
                            if tree.scroll_viewport_by(vp_id, delta) {
                                self.scroll_drag = Some((vp_id, pos.y));
                            }
                        }
                    }
                    TouchPhase::Ended | TouchPhase::Cancelled => {
                        self.scroll_drag = None;
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
