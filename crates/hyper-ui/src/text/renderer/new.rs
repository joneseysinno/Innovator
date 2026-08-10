use glyphon::{Cache, FontSystem, SwashCache, TextAtlas, TextRenderer as GlyphonTextRenderer, Viewport};
use std::collections::HashMap;

use super::TextRenderer;

impl TextRenderer {
    pub fn new(device: &wgpu::Device, queue: &wgpu::Queue, format: wgpu::TextureFormat) -> Self {
        let font_system = FontSystem::new();
        let swash_cache = SwashCache::new();
        let cache = Cache::new(device);
        let viewport = Viewport::new(device, &cache);
        let mut atlas = TextAtlas::new(device, queue, &cache, format);
        let renderer =
            GlyphonTextRenderer::new(&mut atlas, device, wgpu::MultisampleState::default(), None);

        Self {
            font_system,
            swash_cache,
            viewport,
            atlas,
            renderer,
            cache: HashMap::new(),
            width: 1,
            height: 1,
            scale_factor: 1.0,
            pending: Vec::new(),
        }
    }
}
