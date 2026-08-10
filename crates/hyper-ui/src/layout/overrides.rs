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

    /// All override entries — for persistence.
    pub fn iter(&self) -> impl Iterator<Item = (ContainerId, SizeClass, f32)> + '_ {
        self.entries
            .iter()
            .map(|((id, class), fraction)| (*id, *class, *fraction))
    }

    /// Rebuild from persisted records.
    pub fn from_entries(entries: impl IntoIterator<Item = (ContainerId, SizeClass, f32)>) -> Self {
        let mut out = Self::new();
        for (id, class, fraction) in entries {
            out.set(id, class, fraction);
        }
        out
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
