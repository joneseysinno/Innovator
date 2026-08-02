use crate::workspace::analysis::io_kind::IoKind;
use hyper_ui::{PageHeaderConfig, PageHeaderSlots, PageId, PageNode, PodTree};

use super::PageTemplate;

pub fn analysis_page(id: PageId) -> PageTemplate {
    let header = Some(PageHeaderConfig {
        height: 44.0,
        slots: PageHeaderSlots::Custom,
    });
    let pod_tree = PodTree::two_column(0.30);
    let ios = vec![(0, IoKind::InputForm), (1, IoKind::WallView)];
    let node = PageNode {
        id,
        pod_tree,
        header,
        icon_rail: None,
    };
    (node, ios)
}
