use crate::seam::PodTree;

use super::{IconRailConfig, PageHeaderConfig, PageId};

/// A single page — a generic spatial container. No content knowledge.
#[derive(Debug, Clone)]
pub struct PageNode {
    pub id: PageId,
    /// Internal pod layout, fixed per template.
    pub pod_tree: PodTree,
    pub header: Option<PageHeaderConfig>,
    pub icon_rail: Option<IconRailConfig>,
}

impl PageNode {
    /// Empty page used as the sibling when splitting.
    pub fn empty(id: PageId) -> Self {
        Self {
            id,
            pod_tree: PodTree::Leaf { id: 0 },
            header: None,
            icon_rail: None,
        }
    }
}
