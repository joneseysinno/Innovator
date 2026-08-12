use crate::layout::{Overrides, SizeClass};

use super::{Pod, PodId, PodList};

/// Responsive default collapse — no persisted state.
///
/// Compact viewports start pods collapsed to preserve vertical space; Medium
/// and wider expand all pods until the user toggles. Explicit overrides always
/// win (see [`resolved_collapse`]).
pub fn default_collapse(_pod: PodId, size_class: SizeClass) -> bool {
    matches!(size_class, SizeClass::Compact)
}

/// Resolved collapse for layout: override if present, else [`default_collapse`].
pub fn resolved_collapse(pod_id: PodId, size_class: SizeClass, overrides: &Overrides) -> bool {
    overrides
        .get_collapse(Pod::container_id(pod_id), size_class)
        .unwrap_or_else(|| default_collapse(pod_id, size_class))
}

impl PodList {
    /// Apply resolved collapse for every pod at `size_class`.
    pub fn apply_resolved_collapse(&mut self, size_class: SizeClass) {
        let ids: Vec<_> = self.pods.iter().map(|p| p.id).collect();
        for id in ids {
            let collapsed = resolved_collapse(id, size_class, &self.overrides);
            if let Some(pod) = self.pods.iter_mut().find(|p| p.id == id) {
                pod.set_collapsed(collapsed);
            }
        }
    }

    /// Collapse a pod and record a sticky override for `size_class`.
    pub fn collapse(&mut self, id: PodId, size_class: SizeClass) {
        self.overrides
            .set_collapse(Pod::container_id(id), size_class, true);
        if let Some(pod) = self.pods.iter_mut().find(|p| p.id == id) {
            pod.set_collapsed(true);
        }
    }

    /// Expand a pod and record a sticky override for `size_class`.
    pub fn expand(&mut self, id: PodId, size_class: SizeClass) {
        self.overrides
            .set_collapse(Pod::container_id(id), size_class, false);
        if let Some(pod) = self.pods.iter_mut().find(|p| p.id == id) {
            pod.set_collapsed(false);
        }
    }

    /// Toggle collapse and record a sticky override for `size_class`.
    pub fn toggle(&mut self, id: PodId, size_class: SizeClass) {
        let next = !resolved_collapse(id, size_class, &self.overrides);
        self.overrides
            .set_collapse(Pod::container_id(id), size_class, next);
        if let Some(pod) = self.pods.iter_mut().find(|p| p.id == id) {
            pod.set_collapsed(next);
        }
    }
}
