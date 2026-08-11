use crate::domains::structural::io_kind::IoKind;
use hyper_ui::{
    default_icon_rail_config, PageHeaderConfig, PageHeaderSlots, PageId, PageNode, Pod, PodId,
    PodList,
};

use super::PageTemplate;

pub fn analysis_page(id: PageId) -> PageTemplate {
    let header = Some(PageHeaderConfig {
        height: 44.0,
        slots: PageHeaderSlots::Custom,
    });
    // Vertical stack (PodList); uniform chrome + optional rail icons.
    let pods = PodList::two(
        Pod::new(PodId(0), "Input")
            .with_height(0.30)
            .with_nav_icon("▤"),
        Pod::new(PodId(1), "Wall View")
            .with_height(0.70)
            .with_nav_icon("▥"),
    );
    let ios = vec![(PodId(0), IoKind::InputForm), (PodId(1), IoKind::WallView)];
    let node = PageNode::new(id, pods)
        .with_label("Analysis", "A")
        .with_extent(super::analysis_extent())
        .with_header(header)
        .with_icon_rail(Some(default_icon_rail_config()));
    (node, ios)
}
