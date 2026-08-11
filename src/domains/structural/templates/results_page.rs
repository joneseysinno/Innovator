use crate::domains::structural::io_kind::IoKind;
use hyper_ui::{
    default_icon_rail_config, PageId, PageNode, Pod, PodId, PodList,
};

use super::PageTemplate;

pub fn results_page(id: PageId) -> PageTemplate {
    let pods = PodList::two(
        Pod::new(PodId(0), "Results")
            .with_height(0.70)
            .with_nav_icon("▦"),
        Pod::new(PodId(1), "Status")
            .with_height(0.30)
            .with_nav_icon("▧"),
    );
    let ios = vec![(PodId(0), IoKind::ResultsTable), (PodId(1), IoKind::Status)];
    let node = PageNode::new(id, pods)
        .with_label("Results", "R")
        .with_extent(super::results_extent())
        .with_icon_rail(Some(default_icon_rail_config()));
    (node, ios)
}
