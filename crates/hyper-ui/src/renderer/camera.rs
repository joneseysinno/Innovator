//! Scene camera: world ↔ screen transforms, pan, zoom.

use crate::geom::{UVec2, Vec2, WorldRect};

#[derive(Debug, Clone)]
pub struct SceneCamera {
    /// World-space center of the viewport.
    pub center: Vec2,
    /// Pixels per world unit.
    pub zoom: f32,
    /// Current viewport size in pixels.
    pub screen_px: UVec2,
}

impl Default for SceneCamera {
    fn default() -> Self {
        Self {
            center: Vec2::ZERO,
            zoom: 40.0,
            screen_px: UVec2::new(800, 600),
        }
    }
}

impl SceneCamera {
    pub fn new(screen_px: UVec2) -> Self {
        Self {
            screen_px,
            ..Self::default()
        }
    }

    pub fn world_to_screen(&self, p: Vec2) -> Vec2 {
        let half = Vec2::new(self.screen_px.x as f32 * 0.5, self.screen_px.y as f32 * 0.5);
        Vec2::new(
            (p.x - self.center.x) * self.zoom + half.x,
            (p.y - self.center.y) * self.zoom + half.y,
        )
    }

    pub fn screen_to_world(&self, p: Vec2) -> Vec2 {
        let half = Vec2::new(self.screen_px.x as f32 * 0.5, self.screen_px.y as f32 * 0.5);
        Vec2::new(
            (p.x - half.x) / self.zoom + self.center.x,
            (p.y - half.y) / self.zoom + self.center.y,
        )
    }

    pub fn pan(&mut self, delta_screen: Vec2) {
        self.center.x -= delta_screen.x / self.zoom;
        self.center.y -= delta_screen.y / self.zoom;
    }

    pub fn zoom_at(&mut self, anchor_screen: Vec2, factor: f32) {
        let before = self.screen_to_world(anchor_screen);
        self.zoom = (self.zoom * factor).clamp(2.0, 400.0);
        let after = self.screen_to_world(anchor_screen);
        self.center.x += before.x - after.x;
        self.center.y += before.y - after.y;
    }

    pub fn set_screen_size(&mut self, size: UVec2) {
        self.screen_px = size;
    }

    /// Visible world rect with a small margin for culling.
    pub fn visible_world_rect(&self) -> WorldRect {
        let margin_px = 64.0;
        let tl = self.screen_to_world(Vec2::new(-margin_px, -margin_px));
        let br = self.screen_to_world(Vec2::new(
            self.screen_px.x as f32 + margin_px,
            self.screen_px.y as f32 + margin_px,
        ));
        WorldRect::new(
            [tl.x.min(br.x) as f64, tl.y.min(br.y) as f64],
            [tl.x.max(br.x) as f64, tl.y.max(br.y) as f64],
        )
    }
}
