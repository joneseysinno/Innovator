mod collect_rects;
mod default;
mod leaf_rects;
mod reset_ratio;
mod set_ratio;
mod set_ratio_inner;
mod three_column;
mod two_column;

use super::SeamDirection;

/// Binary split tree for pod regions.
#[derive(Debug, Clone)]
pub enum PodTree {
    Leaf {
        id: u32,
    },
    Split {
        direction: SeamDirection,
        ratio: f32,
        first: Box<PodTree>,
        second: Box<PodTree>,
    },
}
