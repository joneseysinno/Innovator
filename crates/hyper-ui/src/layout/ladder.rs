use crate::container::Visibility;

/// How a container degrades under cascade pressure.
///
/// Property of the arrangement, not of the container — pages hide, pods
/// collapse, workspaces hide into tab overflow.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DemotionLadder {
    /// Ordered steps. Demotion walks this list.
    pub steps: &'static [Visibility],
    /// Extent consumed while Collapsed (title bar / rail icon height).
    pub collapsed_extent: f32,
}

impl DemotionLadder {
    /// Next rung after `current`, or `current` if already at the floor of
    /// this ladder.
    pub fn demote(self, current: Visibility) -> Visibility {
        match self.steps.iter().position(|&step| step == current) {
            Some(i) if i + 1 < self.steps.len() => self.steps[i + 1],
            _ => current,
        }
    }

    /// Previous rung toward Shown, or `current` if already at the top.
    pub fn promote(self, current: Visibility) -> Visibility {
        match self.steps.iter().position(|&step| step == current) {
            Some(i) if i > 0 => self.steps[i - 1],
            _ => current,
        }
    }
}

/// Pods collapse to a title bar. No third rung — a page scrolls, so a pod is
/// never hidden by the cascade.
pub const POD_LADDER: DemotionLadder = DemotionLadder {
    steps: &[Visibility::Shown, Visibility::Collapsed],
    collapsed_extent: 26.0,
};

/// Pages do not collapse — they hide into the icon rail.
pub const PAGE_LADDER: DemotionLadder = DemotionLadder {
    steps: &[Visibility::Shown, Visibility::Hidden],
    collapsed_extent: 0.0,
};

/// Workspaces hide into tab-strip overflow.
pub const WORKSPACE_LADDER: DemotionLadder = DemotionLadder {
    steps: &[Visibility::Shown, Visibility::Hidden],
    collapsed_extent: 0.0,
};
