use super::PodId;

/// Title-bar height used when a pod is collapsed.
pub const COLLAPSED_HEIGHT: f32 = 24.0;

/// A single pod — a collapsible content slot. No knowledge of its content.
#[derive(Debug, Clone)]
pub struct Pod {
    pub id: PodId,
    pub collapsed: bool,
    /// Minimum height when expanded.
    pub min_height: f32,
    /// Preferred height weight when expanded (scaled to fill remaining space).
    pub height: f32,
    /// Shown in the title bar when collapsed.
    pub title: String,
}

impl Pod {
    pub fn new(id: PodId, title: impl Into<String>) -> Self {
        Self {
            id,
            collapsed: false,
            min_height: 80.0,
            height: 1.0,
            title: title.into(),
        }
    }

    pub fn with_height(mut self, height: f32) -> Self {
        self.height = height.max(0.01);
        self
    }

    pub fn with_min_height(mut self, min_height: f32) -> Self {
        self.min_height = min_height.max(0.0);
        self
    }
}
