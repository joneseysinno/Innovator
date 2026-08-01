use crate::geom::Vec2;
use crate::particles::SourceStyle;

use super::TextRenderer;

impl TextRenderer {
    pub fn queue_source(
        &mut self,
        text: &str,
        origin: Vec2,
        style: SourceStyle,
        size: f32,
        weight: u16,
    ) {
        let color = match style {
            SourceStyle::Primary => [0.92, 0.93, 0.95, 1.0],
            SourceStyle::Secondary => [0.70, 0.72, 0.76, 1.0],
            SourceStyle::Muted => [0.50, 0.52, 0.56, 1.0],
        };
        self.queue_text(text, origin, size, weight, color, None);
    }
}
