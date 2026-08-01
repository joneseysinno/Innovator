use winit::event::{ElementState, MouseButton, WindowEvent};

use crate::geom::{Rect, Vec2};
use crate::input::UiEvent;
use crate::seam::{PodTree, SeamDirection};

use super::SeamRenderer;

impl SeamRenderer {
    pub fn handle_event(
        &mut self,
        event: &WindowEvent,
        cursor: Vec2,
        pods: &mut PodTree,
        area: Rect,
    ) -> Vec<UiEvent> {
        let mut out = Vec::new();
        match event {
            WindowEvent::CursorMoved { .. } => {
                if let Some((idx, last)) = self.drag {
                    let seam = &self.seams[idx];
                    let delta = match seam.direction {
                        SeamDirection::Vertical => cursor.x - last.x,
                        SeamDirection::Horizontal => cursor.y - last.y,
                    };
                    let span = match seam.direction {
                        SeamDirection::Vertical => area.size.x,
                        SeamDirection::Horizontal => area.size.y,
                    };
                    // Update ratio from absolute cursor position
                    let ratio = match seam.direction {
                        SeamDirection::Vertical => ((cursor.x - area.origin.x) / span).clamp(0.1, 0.9),
                        SeamDirection::Horizontal => {
                            ((cursor.y - area.origin.y) / span).clamp(0.1, 0.9)
                        }
                    };
                    pods.set_ratio(idx, ratio);
                    self.rebuild_from_pods(pods, area);
                    if let Some(s) = self.seams.get_mut(idx) {
                        s.dragging = true;
                        s.hovered = true;
                    }
                    self.drag = Some((idx, cursor));
                    out.push(UiEvent::SeamDrag {
                        seam_index: idx,
                        delta,
                    });
                } else {
                    for s in &mut self.seams {
                        s.hovered = s.hit_rect().contains(cursor);
                    }
                }
            }
            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button: MouseButton::Left,
                ..
            } => {
                if let Some(idx) = self.seams.iter().position(|s| s.hit_rect().contains(cursor)) {
                    // Double-click → reset 50/50
                    let now = std::time::Instant::now();
                    let is_double = self
                        .last_click
                        .map(|(i, t)| i == idx && now.duration_since(t).as_millis() < 350)
                        .unwrap_or(false);
                    if is_double {
                        pods.reset_ratio(idx);
                        self.rebuild_from_pods(pods, area);
                        out.push(UiEvent::SeamReset { seam_index: idx });
                        self.last_click = None;
                    } else {
                        self.drag = Some((idx, cursor));
                        self.seams[idx].dragging = true;
                        self.last_click = Some((idx, now));
                    }
                }
            }
            WindowEvent::MouseInput {
                state: ElementState::Released,
                button: MouseButton::Left,
                ..
            } => {
                if let Some((idx, _)) = self.drag.take() {
                    if let Some(s) = self.seams.get_mut(idx) {
                        s.dragging = false;
                    }
                }
            }
            _ => {}
        }
        out
    }
}
