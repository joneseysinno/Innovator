use crate::seam::SeamDirection;

use super::PodTree;

impl Default for PodTree {
    fn default() -> Self {
        Self::Split {
            direction: SeamDirection::Vertical,
            ratio: 0.5,
            first: Box::new(PodTree::Leaf { id: 0 }),
            second: Box::new(PodTree::Leaf { id: 1 }),
        }
    }
}
