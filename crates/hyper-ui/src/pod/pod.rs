use crate::container::{ContainerId, ContainerState, Extent, Visibility};
use crate::layout::POD_LADDER;

use super::PodId;

/// Title-bar height used when a pod is collapsed — matches [`POD_LADDER`].
pub const COLLAPSED_HEIGHT: f32 = POD_LADDER.collapsed_extent;

/// Reference height used to turn flex weights into preferred pixel ideals.
const IDEAL_REFERENCE_HEIGHT: f32 = 480.0;

/// A single pod — a collapsible content slot. No knowledge of its content.
#[derive(Debug, Clone)]
pub struct Pod {
    pub id: PodId,
    /// Shared container primitives.
    pub state: ContainerState,
    pub collapsed: bool,
    /// Minimum height when expanded.
    pub min_height: f32,
    /// Preferred height weight when expanded (maps into extent.weight / ideal).
    pub height: f32,
    /// Shown in the title bar when collapsed.
    pub title: String,
}

impl Pod {
    /// Pod-level id in the shared [`ContainerId`] space (bit 32 set).
    pub fn container_id(id: PodId) -> ContainerId {
        ContainerId((1u64 << 32) | u64::from(id.0))
    }

    pub fn new(id: PodId, title: impl Into<String>) -> Self {
        let title = title.into();
        let min_height = 80.0;
        let height = 1.0;
        let ideal = (height * IDEAL_REFERENCE_HEIGHT).max(min_height);
        let state = ContainerState::new(
            Self::container_id(id),
            title.clone(),
            String::new(),
            Visibility::Shown,
            Extent::new(min_height, ideal, height),
        );
        Self {
            id,
            state,
            collapsed: false,
            min_height,
            height,
            title,
        }
    }

    pub fn with_height(mut self, height: f32) -> Self {
        self.height = height.max(0.01);
        self.state.extent.weight = self.height;
        self.state.extent.ideal = (self.height * IDEAL_REFERENCE_HEIGHT).max(self.min_height);
        self
    }

    pub fn with_min_height(mut self, min_height: f32) -> Self {
        self.min_height = min_height.max(0.0);
        self.state.extent.min = self.min_height;
        self.state.extent.ideal = self.state.extent.ideal.max(self.min_height);
        self
    }

    /// Keep `collapsed` and `state.intent` aligned. Writes intent only.
    pub fn set_collapsed(&mut self, collapsed: bool) {
        self.collapsed = collapsed;
        self.state.intent = if collapsed {
            Visibility::Collapsed
        } else {
            Visibility::Shown
        };
    }

    pub(crate) fn sync_state_from_fields(&mut self) {
        self.state.intent = if self.collapsed {
            Visibility::Collapsed
        } else {
            Visibility::Shown
        };
        self.state.label = self.title.clone();
        self.state.extent.min = self.min_height;
        self.state.extent.weight = self.height.max(0.01);
        if self.state.extent.ideal < self.min_height {
            self.state.extent.ideal = self.min_height;
        }
    }
}
