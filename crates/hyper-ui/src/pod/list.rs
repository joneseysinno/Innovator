use crate::layout::Overrides;

use super::{Pod, PodId};

/// Flat ordered list of pods — a vertical stack within a page.
#[derive(Debug, Clone)]
pub struct PodList {
    pub pods: Vec<Pod>,
    /// Pixel gap between pods (divider thickness).
    pub gap: f32,
    /// Per-size-class size overrides from divider drags.
    pub overrides: Overrides,
    /// Transient scroll offset for the page pod viewport. Not persisted.
    pub scroll_offset: f32,
}

impl Default for PodList {
    fn default() -> Self {
        Self {
            pods: vec![Pod::new(PodId(0), "Pod")],
            gap: 1.0,
            overrides: Overrides::new(),
            scroll_offset: 0.0,
        }
    }
}

impl PodList {
    pub fn new(pods: Vec<Pod>) -> Self {
        Self {
            pods,
            gap: 1.0,
            overrides: Overrides::new(),
            scroll_offset: 0.0,
        }
    }

    pub fn with_gap(mut self, gap: f32) -> Self {
        self.gap = gap.max(0.0);
        self
    }

    /// Two stacked pods with proportional height weights.
    pub fn two(first: Pod, second: Pod) -> Self {
        Self::new(vec![first, second])
    }
}
