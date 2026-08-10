use crate::domains::structural::io_kind::IoKind;
use hyper_ui::{
    PageHeaderConfig, PageHeaderSlots, PageId, PageNode, Pod, PodId, PodList,
};

use super::PageTemplate;

pub fn analysis_page(id: PageId) -> PageTemplate {
    let header = Some(PageHeaderConfig {
        height: 44.0,
        slots: PageHeaderSlots::Custom,
    });
    // Vertical stack (PodList); former side-by-side two_column becomes stacked pods.
    let pods = PodList::two(
        Pod::new(PodId(0), "Input").with_height(0.30),
        Pod::new(PodId(1), "Wall View").with_height(0.70),
    );
    let ios = vec![(PodId(0), IoKind::InputForm), (PodId(1), IoKind::WallView)];
    let node = PageNode::new(id, pods)
        .with_label("Analysis", "A")
        .with_extent(super::analysis_extent())
        .with_header(header);
    (node, ios)
}
