mod clear_pending;
mod ensure_buffer;
mod make_buffer;
mod measure;
mod new;
mod prepare;
mod queue_source;
mod queue_text;
mod render_into;
mod resize;
mod trim;

use glyphon::{FontSystem, SwashCache, TextAtlas, Viewport};
use std::collections::HashMap;

use super::{PendingText, TextKey};

pub struct TextRenderer {
    font_system: FontSystem,
    swash_cache: SwashCache,
    viewport: Viewport,
    atlas: TextAtlas,
    renderer: glyphon::TextRenderer,
    /// Content-keyed cache — reused across frames when text is unchanged.
    cache: HashMap<TextKey, glyphon::Buffer>,
    /// Logical viewport size (matches UI layout coordinates).
    width: u32,
    height: u32,
    /// Window scale — sharpens glyphs while positions stay logical.
    scale_factor: f32,
    pending: Vec<PendingText>,
}
