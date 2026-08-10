use crate::geom::Rect;

use super::{ContainerId, Extent, Visibility};

/// The complete state of a container. Identical at every level.
///
/// `intent` is written only by explicit user action and is persisted.
/// `resolved` and `rect` are outputs of the resolve pass — recomputed every
/// layout, never persisted. Application code can read them but cannot write
/// them; only `pub(crate)` setters exist for the resolve pass.
#[derive(Debug, Clone)]
pub struct ContainerState {
    pub id: ContainerId,
    pub label: String,
    pub icon: String,

    /// The user's choice. Persisted. Device-independent.
    pub intent: Visibility,

    /// Output of the resolve pass. Recomputed every layout. NEVER persisted.
    resolved: Visibility,

    pub extent: Extent,

    /// Assigned by the resolve pass. Transient, like `resolved`.
    rect: Rect,
}

impl ContainerState {
    pub fn new(
        id: ContainerId,
        label: impl Into<String>,
        icon: impl Into<String>,
        intent: Visibility,
        extent: Extent,
    ) -> Self {
        Self {
            id,
            label: label.into(),
            icon: icon.into(),
            intent,
            resolved: intent,
            extent,
            rect: Rect::default(),
        }
    }

    pub fn resolved(&self) -> Visibility {
        self.resolved
    }

    pub fn rect(&self) -> Rect {
        self.rect
    }

    /// Called only by the resolve pass.
    pub(crate) fn set_resolved(&mut self, resolved: Visibility) {
        self.resolved = resolved;
    }

    /// Called only by the resolve pass.
    pub(crate) fn set_rect(&mut self, rect: Rect) {
        self.rect = rect;
    }
}
