use crate::seam::SeamDirection;

use super::PodTree;

impl PodTree {
    pub fn two_column(ratio: f32) -> Self {
        Self::Split {
            direction: SeamDirection::Vertical,
            ratio: ratio.clamp(0.1, 0.9),
            first: Box::new(PodTree::Leaf { id: 0 }),
            second: Box::new(PodTree::Leaf { id: 1 }),
        }
    }
}
