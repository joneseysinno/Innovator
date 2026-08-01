use super::Vec2;

#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Rect {
    pub origin: Vec2,
    pub size: Vec2,
}

impl Rect {
    pub fn new(origin: Vec2, size: Vec2) -> Self {
        Self { origin, size }
    }

    pub fn from_xywh(x: f32, y: f32, w: f32, h: f32) -> Self {
        Self {
            origin: Vec2::new(x, y),
            size: Vec2::new(w, h),
        }
    }

    pub fn contains(&self, p: Vec2) -> bool {
        p.x >= self.origin.x
            && p.y >= self.origin.y
            && p.x <= self.origin.x + self.size.x
            && p.y <= self.origin.y + self.size.y
    }

    pub fn max(&self) -> Vec2 {
        self.origin + self.size
    }

    pub fn inflate(&self, amount: f32) -> Self {
        Self {
            origin: Vec2::new(self.origin.x - amount, self.origin.y - amount),
            size: Vec2::new(self.size.x + amount * 2.0, self.size.y + amount * 2.0),
        }
    }

    pub fn with_padding(&self, pad: f32) -> Self {
        Self {
            origin: Vec2::new(self.origin.x + pad, self.origin.y + pad),
            size: Vec2::new(
                (self.size.x - pad * 2.0).max(0.0),
                (self.size.y - pad * 2.0).max(0.0),
            ),
        }
    }
}
