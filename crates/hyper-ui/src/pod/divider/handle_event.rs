use winit::event::{ElementState, MouseButton, WindowEvent};
use winit::window::CursorIcon;

use crate::geom::Vec2;
use crate::input::UiEvent;

use super::PodDividerRenderer;

impl PodDividerRenderer {
    pub fn handle_event(&mut self, event: &WindowEvent, cursor: Vec2) -> Vec<UiEvent> {
        let mut out = Vec::new();
        match event {
            WindowEvent::CursorMoved { .. } => {
                if let Some((idx, last_y)) = self.drag {
                    let delta = cursor.y - last_y;
                    let above = self.dividers[idx].above;
                    out.push(UiEvent::PodDividerDrag { above, delta });
                    self.drag = Some((idx, cursor.y));
                } else {
                    for d in &mut self.dividers {
                        d.hovered = d.rect.contains(cursor);
                    }
                }
            }
            WindowEvent::MouseInput {
                state: ElementState::Pressed,
                button: MouseButton::Left,
                ..
            } => {
                if let Some(idx) = self
                    .dividers
                    .iter()
                    .position(|d| d.rect.contains(cursor))
                {
                    let now = std::time::Instant::now();
                    let is_double = self
                        .last_click
                        .map(|(i, t)| i == idx && now.duration_since(t).as_millis() < 350)
                        .unwrap_or(false);
                    if is_double {
                        let above = self.dividers[idx].above;
                        out.push(UiEvent::PodDividerEqualize { above });
                        self.last_click = None;
                    } else {
                        self.drag = Some((idx, cursor.y));
                        self.dividers[idx].dragging = true;
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
                    if let Some(d) = self.dividers.get_mut(idx) {
                        d.dragging = false;
                    }
                }
            }
            _ => {}
        }
        out
    }

    pub fn cursor_icon(&self) -> Option<CursorIcon> {
        if self.drag.is_some() || self.dividers.iter().any(|d| d.hovered) {
            Some(CursorIcon::RowResize)
        } else {
            None
        }
    }
}
