/// A container's demand on its parent's arrangement axis.
///
/// Logical pixels — device-independent. Never a ratio.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Extent {
    /// Hard floor. Below this the container is demoted, never squeezed.
    pub min: f32,
    /// Preferred size when space is not scarce.
    pub ideal: f32,
    /// Share of surplus beyond ideal. 0.0 = never grows past ideal.
    pub weight: f32,
}

impl Extent {
    pub const fn new(min: f32, ideal: f32, weight: f32) -> Self {
        Self { min, ideal, weight }
    }

    /// Fixed demand: `min == ideal`, never grows (`weight == 0`).
    pub const fn fixed(size: f32) -> Self {
        Self {
            min: size,
            ideal: size,
            weight: 0.0,
        }
    }

    /// Preferred size with unit weight for surplus distribution.
    pub const fn preferred(min: f32, ideal: f32) -> Self {
        Self {
            min,
            ideal,
            weight: 1.0,
        }
    }
}
