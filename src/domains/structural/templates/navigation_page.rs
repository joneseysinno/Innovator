use crate::domains::structural::io_kind::IoKind;
use hyper_ui::{
    default_icon_rail_config, PageId, PageNode, Pod, PodId, PodList,
};

use super::PageTemplate;

pub fn navigation_page(id: PageId) -> PageTemplate {
    let pods = PodList::two(
        Pod::new(PodId(0), "Wall List")
            .with_height(0.35)
            .with_nav_icon("≡"),
        Pod::new(PodId(1), "Summary")
            .with_height(0.65)
            .with_nav_icon("▣"),
    );
    let ios = vec![(PodId(0), IoKind::WallList), (PodId(1), IoKind::WallSummary)];
    let node = PageNode::new(id, pods)
        .with_label("Navigation", "N")
        .with_extent(super::nav_extent())
        .with_icon_rail(Some(default_icon_rail_config()));
    (node, ios)
}
