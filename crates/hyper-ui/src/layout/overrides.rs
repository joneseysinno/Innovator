use std::collections::HashMap;

use crate::container::ContainerId;

use super::SizeClass;

/// User size adjustments, scoped to the size class they were made in.
///
/// Values are fractions of the arrangement axis (not pixels). Persisted and
/// synced; applied at resolve step 2, clamped to `min` on read.
#[derive(Debug, Clone, Default)]
pub struct Overrides {
    entries: HashMap<(ContainerId, SizeClass), f32>,
    collapse: HashMap<(ContainerId, SizeClass), bool>,
}

impl Overrides {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set(&mut self, id: ContainerId, class: SizeClass, fraction: f32) {
        self.entries.insert((id, class), fraction.clamp(0.0, 1.0));
    }

    pub fn get(&self, id: ContainerId, class: SizeClass) -> Option<f32> {
        self.entries.get(&(id, class)).copied()
    }

    pub fn remove(&mut self, id: ContainerId, class: SizeClass) -> Option<f32> {
        self.entries.remove(&(id, class))
    }

    /// Sticky pod collapse for `(id, class)` — written only on explicit user toggle.
    pub fn set_collapse(&mut self, id: ContainerId, class: SizeClass, collapsed: bool) {
        self.collapse.insert((id, class), collapsed);
    }

    pub fn get_collapse(&self, id: ContainerId, class: SizeClass) -> Option<bool> {
        self.collapse.get(&(id, class)).copied()
    }

    pub fn remove_collapse(&mut self, id: ContainerId, class: SizeClass) -> Option<bool> {
        self.collapse.remove(&(id, class))
    }

    /// All collapse override entries — for persistence.
    pub fn iter_collapse(&self) -> impl Iterator<Item = (ContainerId, SizeClass, bool)> + '_ {
        self.collapse
            .iter()
            .map(|((id, class), collapsed)| (*id, *class, *collapsed))
    }

    /// All override entries — for persistence.
    pub fn iter(&self) -> impl Iterator<Item = (ContainerId, SizeClass, f32)> + '_ {
        self.entries
            .iter()
            .map(|((id, class), fraction)| (*id, *class, *fraction))
    }

    /// Rebuild fraction overrides from persisted records.
    pub fn from_entries(entries: impl IntoIterator<Item = (ContainerId, SizeClass, f32)>) -> Self {
        let mut out = Self::new();
        for (id, class, fraction) in entries {
            out.set(id, class, fraction);
        }
        out
    }

    /// Rebuild collapse overrides from persisted records.
    pub fn from_collapse_entries(
        entries: impl IntoIterator<Item = (ContainerId, SizeClass, bool)>,
    ) -> Self {
        let mut out = Self::new();
        for (id, class, collapsed) in entries {
            out.set_collapse(id, class, collapsed);
        }
        out
    }

    /// Merge collapse overrides into an existing [`Overrides`] (e.g. after fractions).
    pub fn merge_collapse_entries(
        mut self,
        entries: impl IntoIterator<Item = (ContainerId, SizeClass, bool)>,
    ) -> Self {
        for (id, class, collapsed) in entries {
            self.set_collapse(id, class, collapsed);
        }
        self
    }

    /// Effective preferred size for `id` on this axis: override fraction ×
    /// `axis_available`, else `authored_ideal`, always ≥ `min`.
    pub fn effective_ideal(
        &self,
        id: ContainerId,
        class: SizeClass,
        axis_available: f32,
        min: f32,
        authored_ideal: f32,
    ) -> f32 {
        let raw = match self.get(id, class) {
            Some(fraction) => fraction * axis_available,
            None => authored_ideal,
        };
        raw.max(min)
    }
}
