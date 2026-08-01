/// Axis-aligned world-space rectangle (f64 Hilbert-friendly).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WorldRect {
    pub min: [f64; 2],
    pub max: [f64; 2],
}

impl WorldRect {
    pub fn new(min: [f64; 2], max: [f64; 2]) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, p: [f64; 2]) -> bool {
        p[0] >= self.min[0] && p[0] <= self.max[0] && p[1] >= self.min[1] && p[1] <= self.max[1]
    }
}
