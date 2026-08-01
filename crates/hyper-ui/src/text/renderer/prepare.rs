use glyphon::{Buffer, Resolution, TextArea};

use crate::text::TextKey;

use super::TextRenderer;

impl TextRenderer {
    pub fn prepare(&mut self, device: &wgpu::Device, queue: &wgpu::Queue) {
        self.viewport.update(
            queue,
            Resolution {
                width: self.width,
                height: self.height,
            },
        );

        let pending = self.pending.clone();
        for item in &pending {
            self.ensure_buffer(&item.key);
        }

        let mut stolen: Vec<(TextKey, Buffer)> = Vec::with_capacity(pending.len());
        for item in &pending {
            let buf = self
                .cache
                .remove(&item.key)
                .unwrap_or_else(|| self.make_buffer(&item.key));
            stolen.push((item.key.clone(), buf));
        }

        let areas: Vec<TextArea> = pending
            .iter()
            .zip(stolen.iter())
            .map(|(item, (_k, buffer))| TextArea {
                buffer,
                left: item.left,
                top: item.top,
                scale: 1.0,
                bounds: item.bounds,
                default_color: item.color,
                custom_glyphs: &[],
            })
            .collect();

        let _ = self.renderer.prepare(
            device,
            queue,
            &mut self.font_system,
            &mut self.atlas,
            &self.viewport,
            areas,
            &mut self.swash_cache,
        );

        for (key, buffer) in stolen {
            self.cache.insert(key, buffer);
        }
    }
}
