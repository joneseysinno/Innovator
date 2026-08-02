use winit::event::{ElementState, MouseButton, WindowEvent};

use crate::geom::Vec2;
use crate::input::UiEvent;
use crate::seam::SeamDirection;

use super::SeamRenderer;

/// Ratio mutation applied by seam drag / double-click reset.
#[derive(Debug, Clone, Copy)]
pub enum SeamRatioAction {
    Set(f32),
    Reset,
}

impl SeamRenderer {
    /// Detect seam interaction and return events plus an optional ratio action.
    /// Caller applies the action to its page tree, then rebuilds this renderer.
    pub fn handle_event_with(
        &mut self,
        event: &WindowEvent,
        cursor: Vec2,
    ) -> (Vec<UiEvent>, Option<(usize, SeamRatioAction)>) {
        let mut out = Vec::new();
        let mut action = None;
        match event {
            WindowEvent::CursorMoved { .. } => {
                if let Some((idx, last)) = self.drag {
                    let seam = &self.seams[idx];
                    let area = seam.split_area;
                    let delta = match seam.direction {
                        SeamDirection::Vertical => cursor.x - last.x,
                        SeamDirection::Horizontal => cursor.y - last.y,
                    };
                    let span = match seam.direction {
                        SeamDirection::Vertical => area.size.x,
                        SeamDirection::Horizontal => area.size.y,
                    };
                    let ratio = match seam.direction {
                        SeamDirection::Vertical => {
                            ((cursor.x - area.origin.x) / span).clamp(0.1, 0.9)
                        }
                        SeamDirection::Horizontal => {
                            ((cursor.y - area.origin.y) / span).clamp(0.1, 0.9)
                        }
                    };
                    action = Some((idx, SeamRatioAction::Set(ratio)));
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
                    let now = std::time::Instant::now();
                    let is_double = self
                        .last_click
                        .map(|(i, t)| i == idx && now.duration_since(t).as_millis() < 350)
                        .unwrap_or(false);
                    if is_double {
                        action = Some((idx, SeamRatioAction::Reset));
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
            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button: MouseButton::Right,
                ..
            } => {
                if let Some(idx) = self
                    .seams
                    .iter()
                    .position(|s| s.hit_rect().contains(cursor))
                {
                    let seam = &self.seams[idx];
                    out.push(UiEvent::PageSeamRightClick {
                        seam_id: seam.seam_id,
                        cursor,
                        direction: seam.direction,
                    });
                }
            }
            _ => {}
        }
        (out, action)
    }

    /// After the caller applies a ratio action and rebuilds, restore drag visuals.
    pub fn mark_dragging(&mut self, idx: usize) {
        if let Some(s) = self.seams.get_mut(idx) {
            s.dragging = true;
            s.hovered = true;
        }
    }
}
