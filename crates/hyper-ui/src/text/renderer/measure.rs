use crate::geom::Vec2;
use crate::text::TextKey;

use super::TextRenderer;

impl TextRenderer {
    pub fn measure(&mut self, text: &str, size: f32, weight: u16) -> Vec2 {
        let key = TextKey {
            text: text.to_string(),
            size_milli: (size * 1000.0) as u32,
            weight,
        };
        self.ensure_buffer(&key);
        if let Some(buf) = self.cache.get(&key) {
            let mut w = 0.0f32;
            for run in buf.layout_runs() {
                w = w.max(run.line_w);
            }
            return Vec2::new(w.max(8.0), size * 1.35);
        }
        Vec2::new(text.chars().count() as f32 * size * 0.55, size * 1.35)
    }
}
