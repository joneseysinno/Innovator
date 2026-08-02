use crate::workspace::analysis::io_kind::IoKind;
use hyper_ui::{PageId, PageNode, Pod, PodId, PodList};

use super::PageTemplate;

pub fn results_page(id: PageId) -> PageTemplate {
    let pods = PodList::two(
        Pod::new(PodId(0), "Results").with_height(0.70),
        Pod::new(PodId(1), "Status").with_height(0.30),
    );
    let ios = vec![(PodId(0), IoKind::ResultsTable), (PodId(1), IoKind::Status)];
    let node = PageNode {
        id,
        pods,
        header: None,
        icon_rail: None,
    };
    (node, ios)
}
