use crate::pod::PodList;

use super::{IconRailConfig, PageHeaderConfig, PageId};

/// A single page — a generic spatial container. No content knowledge.
#[derive(Debug, Clone)]
pub struct PageNode {
    pub id: PageId,
    /// Ordered pod stack within this page.
    pub pods: PodList,
    pub header: Option<PageHeaderConfig>,
    pub icon_rail: Option<IconRailConfig>,
}

impl PageNode {
    /// Empty page used as the sibling when splitting.
    pub fn empty(id: PageId) -> Self {
        Self {
            id,
            pods: PodList::default(),
            header: None,
            icon_rail: None,
        }
    }
}
