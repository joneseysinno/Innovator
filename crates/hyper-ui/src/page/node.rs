use crate::container::{ContainerId, ContainerState, Extent, Visibility};
use crate::pod::PodList;
use hypernode::NodeId;

use super::{IconRailConfig, PageHeaderConfig, PageId};

/// A single page — a generic spatial container. No content knowledge.
#[derive(Debug, Clone)]
pub struct PageNode {
    pub id: PageId,
    /// Graph identity (`SpaceClass::UIView`). `NodeId(0)` until dual-written.
    pub node_id: NodeId,
    /// Shared container primitives.
    pub state: ContainerState,
    /// Ordered pod stack within this page.
    pub pods: PodList,
    pub header: Option<PageHeaderConfig>,
    pub icon_rail: Option<IconRailConfig>,
}

impl PageNode {
    /// Page-level id in the shared [`ContainerId`] space (low 32 bits).
    pub fn container_id(id: PageId) -> ContainerId {
        ContainerId(u64::from(id.0))
    }

    fn initial_state(
        id: PageId,
        label: impl Into<String>,
        icon: impl Into<String>,
        extent: Extent,
    ) -> ContainerState {
        ContainerState::new(
            Self::container_id(id),
            label,
            icon,
            Visibility::Shown,
            extent,
        )
    }

    pub fn new(id: PageId, pods: PodList) -> Self {
        Self {
            id,
            node_id: NodeId(0),
            state: Self::initial_state(id, "Page", "", Extent::preferred(280.0, 600.0)),
            pods,
            header: None,
            icon_rail: None,
        }
    }

    pub fn with_label(mut self, label: impl Into<String>, icon: impl Into<String>) -> Self {
        self.state.label = label.into();
        self.state.icon = icon.into();
        self
    }

    pub fn with_extent(mut self, extent: Extent) -> Self {
        self.state.extent = extent;
        self
    }

    pub fn with_header(mut self, header: Option<PageHeaderConfig>) -> Self {
        self.header = header;
        self
    }

    pub fn with_icon_rail(mut self, icon_rail: Option<IconRailConfig>) -> Self {
        self.icon_rail = icon_rail;
        self
    }

    /// Empty page used as the sibling when splitting.
    pub fn empty(id: PageId) -> Self {
        Self::new(id, PodList::default())
    }
}
