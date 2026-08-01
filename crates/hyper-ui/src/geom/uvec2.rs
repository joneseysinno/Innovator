#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct UVec2 {
    pub x: u32,
    pub y: u32,
}

impl UVec2 {
    pub const fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}
