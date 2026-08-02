use crate::workspace::analysis::io_kind::IoKind;
use hyper_ui::{IconRailConfig, IconRailSide, PageId, PageNode, PodTree};

use super::PageTemplate;

pub fn navigation_page(id: PageId) -> PageTemplate {
    // two_column is vertical; navigation pods are stacked — use horizontal split.
    let pod_tree = PodTree::Split {
        direction: hyper_ui::SeamDirection::Horizontal,
        ratio: 0.35,
        first: Box::new(PodTree::Leaf { id: 0 }),
        second: Box::new(PodTree::Leaf { id: 1 }),
    };
    let ios = vec![(0, IoKind::WallList), (1, IoKind::WallSummary)];
    let node = PageNode {
        id,
        pod_tree,
        header: None,
        icon_rail: Some(IconRailConfig {
            side: IconRailSide::Left,
            width: 34.0,
        }),
    };
    (node, ios)
}
