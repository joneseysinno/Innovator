use crate::domains::structural::io_kind::IoKind;
use hyper_ui::{
    IconRailConfig, IconRailSide, PageId, PageNode, Pod, PodId, PodList,
};

use super::PageTemplate;

pub fn navigation_page(id: PageId) -> PageTemplate {
    let pods = PodList::two(
        Pod::new(PodId(0), "Wall List").with_height(0.35),
        Pod::new(PodId(1), "Summary").with_height(0.65),
    );
    let ios = vec![(PodId(0), IoKind::WallList), (PodId(1), IoKind::WallSummary)];
    let node = PageNode {
        id,
        pods,
        header: None,
        icon_rail: Some(IconRailConfig {
            side: IconRailSide::Left,
            width: 34.0,
        }),
    };
    (node, ios)
}
