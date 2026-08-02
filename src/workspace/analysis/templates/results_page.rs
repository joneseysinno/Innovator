use crate::workspace::analysis::io_kind::IoKind;
use hyper_ui::{PageId, PageNode, PodTree, SeamDirection};

use super::PageTemplate;

pub fn results_page(id: PageId) -> PageTemplate {
    let pod_tree = PodTree::Split {
        direction: SeamDirection::Horizontal,
        ratio: 0.70,
        first: Box::new(PodTree::Leaf { id: 0 }),
        second: Box::new(PodTree::Leaf { id: 1 }),
    };
    let ios = vec![(0, IoKind::ResultsTable), (1, IoKind::Status)];
    let node = PageNode {
        id,
        pod_tree,
        header: None,
        icon_rail: None,
    };
    (node, ios)
}
