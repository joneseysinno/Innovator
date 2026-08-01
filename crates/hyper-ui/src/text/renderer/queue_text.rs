use glyphon::{Color, TextBounds};

use crate::geom::Vec2;
use crate::text::{PendingText, TextKey};

use super::TextRenderer;

impl TextRenderer {
    pub fn queue_text(
        &mut self,
        text: &str,
        origin: Vec2,
        size: f32,
        weight: u16,
        color: [f32; 4],
        clip: Option<(i32, i32, i32, i32)>,
    ) {
        let key = TextKey {
            text: text.to_string(),
            size_milli: (size * 1000.0) as u32,
            weight,
        };
        let bounds = match clip {
            Some((l, t, r, b)) => TextBounds {
                left: l,
                top: t,
                right: r,
                bottom: b,
            },
            None => TextBounds {
                left: 0,
                top: 0,
                right: self.width as i32,
                bottom: self.height as i32,
            },
        };
        let left = if origin.x.is_finite() { origin.x } else { 0.0 };
        let top = if origin.y.is_finite() { origin.y } else { 0.0 };
        self.pending.push(PendingText {
            key,
            left,
            top,
            bounds,
            color: Color::rgba(
                (color[0] * 255.0) as u8,
                (color[1] * 255.0) as u8,
                (color[2] * 255.0) as u8,
                (color[3] * 255.0) as u8,
            ),
        });
    }
}
