use crate::seam::SeamDirection;

use super::PodTree;

impl PodTree {
    /// Three vertical columns: leaf 0 | leaf 1 | leaf 2.
    ///
    /// `first` is the fraction of total width for column 0.
    /// `second` is the fraction of total width for column 1.
    /// Column 2 receives the remainder.
    pub fn three_column(first: f32, second: f32) -> Self {
        let first = first.clamp(0.1, 0.8);
        let second = second.clamp(0.1, 0.8);
        let rest = (1.0 - first).max(0.2);
        let second_of_rest = (second / rest).clamp(0.1, 0.9);
        Self::Split {
            direction: SeamDirection::Vertical,
            ratio: first,
            first: Box::new(PodTree::Leaf { id: 0 }),
            second: Box::new(PodTree::Split {
                direction: SeamDirection::Vertical,
                ratio: second_of_rest,
                first: Box::new(PodTree::Leaf { id: 1 }),
                second: Box::new(PodTree::Leaf { id: 2 }),
            }),
        }
    }
}
