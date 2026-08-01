use crate::geom::{Rect, Vec2};

/// Layout output after measure/arrange.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct LayoutBox {
    pub origin: Vec2,
    pub size: Vec2,
}

impl LayoutBox {
    pub fn rect(&self) -> Rect {
        Rect::new(self.origin, self.size)
    }
}
