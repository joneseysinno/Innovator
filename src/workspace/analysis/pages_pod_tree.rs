use hyper_ui::{PodTree, SeamDirection};

/// Page split with nested Navigation, Analysis, and Results pods.
///
/// Leaf ids:
/// - 0 WallList, 3 WallSummary
/// - 4 InputForm, 5 WallView
/// - 6 ResultsTable, 7 Status
pub fn pages_pod_tree() -> PodTree {
    let first = 0.22_f32.clamp(0.1, 0.8);
    let second = 0.48_f32.clamp(0.1, 0.8);
    let rest = (1.0 - first).max(0.2);
    let second_of_rest = (second / rest).clamp(0.1, 0.9);

    PodTree::Split {
        direction: SeamDirection::Vertical,
        ratio: first,
        first: Box::new(PodTree::Split {
            direction: SeamDirection::Horizontal,
            ratio: 0.35,
            first: Box::new(PodTree::Leaf { id: 0 }),
            second: Box::new(PodTree::Leaf { id: 3 }),
        }),
        second: Box::new(PodTree::Split {
            direction: SeamDirection::Vertical,
            ratio: second_of_rest,
            first: Box::new(PodTree::Split {
                direction: SeamDirection::Vertical,
                ratio: 0.30,
                first: Box::new(PodTree::Leaf { id: 4 }),
                second: Box::new(PodTree::Leaf { id: 5 }),
            }),
            second: Box::new(PodTree::Split {
                direction: SeamDirection::Horizontal,
                ratio: 0.70,
                first: Box::new(PodTree::Leaf { id: 6 }),
                second: Box::new(PodTree::Leaf { id: 7 }),
            }),
        }),
    }
}
