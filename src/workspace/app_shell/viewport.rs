//! Layout viewport helpers — logical size, preview letterbox, hysteresis.

use super::AppShell;
use crate::devtools::preview::letterbox_rect;
use hyper_ui::{InputClass, Rect, SizeClass, Vec2, Viewport};

impl AppShell {
    /// Full-window logical rect, or letterboxed preset when F9 preview is active.
    pub fn layout_area(&self) -> Rect {
        match self.preview.size() {
            None => self.window_area,
            Some(logical) => letterbox_rect(self.window_area, logical),
        }
    }

    /// Viewport for resolve — hysteretic size class + preview input class.
    pub fn resolve_viewport(&mut self) -> Viewport {
        let area = self.layout_area();
        let input = self.preview.input_class(self.input_class);
        // Prefer authored preset logical size when letterbox is 1:1.
        let size = match self.preview.size() {
            Some(logical)
                if (area.size.x - logical.x).abs() < 1.0
                    && (area.size.y - logical.y).abs() < 1.0 =>
            {
                logical
            }
            _ => Vec2::new(area.size.x.max(1.0), area.size.y.max(1.0)),
        };
        let vp = Viewport::new(size, self.scale_factor, Some(self.size_class), input);
        self.size_class = vp.size_class;
        vp
    }

    /// Apply a physical inner size + scale factor → logical window_area.
    pub fn set_physical_size(&mut self, width: u32, height: u32, scale_factor: f32) {
        self.physical_width = width;
        self.physical_height = height;
        self.scale_factor = scale_factor.max(0.01);
        let lw = width as f32 / self.scale_factor;
        let lh = height as f32 / self.scale_factor;
        self.window_area = Rect::from_xywh(0.0, 0.0, lw, lh);
    }

    pub fn promote_input_to_hybrid(&mut self) {
        if matches!(self.input_class, InputClass::Pointer) {
            self.input_class = InputClass::Hybrid;
        }
    }

    pub fn cycle_preview(&mut self) {
        self.preview = self.preview.next();
        // Reset size-class hysteresis against the new viewport.
        self.size_class = SizeClass::from_width(
            self.preview
                .size()
                .map(|s| s.x)
                .unwrap_or(self.window_area.size.x)
                .max(1.0),
        );
    }
}
