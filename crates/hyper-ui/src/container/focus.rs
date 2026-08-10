use super::ContainerId;

/// The chain of containers from workspace down to the focused pod.
/// Every id on this path has focus distance 0.
#[derive(Debug, Clone, Default)]
pub struct FocusPath {
    pub chain: Vec<ContainerId>,
}

impl FocusPath {
    pub fn new(chain: Vec<ContainerId>) -> Self {
        Self { chain }
    }

    pub fn contains(&self, id: ContainerId) -> bool {
        self.chain.contains(&id)
    }

    /// Focus distance of the child at `index` within an arrangement's
    /// `siblings`.
    ///
    /// - Child on the focus path → `0`
    /// - Otherwise → `|index - focused_sibling_index|`
    /// - No focused sibling in this arrangement → `index + 1`
    pub fn distance(&self, index: usize, siblings: &[ContainerId]) -> usize {
        if index < siblings.len() && self.contains(siblings[index]) {
            return 0;
        }
        match siblings.iter().position(|id| self.contains(*id)) {
            Some(focused) => index.abs_diff(focused),
            None => index.saturating_add(1),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ContainerId, FocusPath};

    fn ids(n: u64) -> Vec<ContainerId> {
        (0..n).map(ContainerId).collect()
    }

    #[test]
    fn on_focus_path_is_zero() {
        let siblings = ids(3);
        let focus = FocusPath::new(vec![ContainerId(1)]);
        assert_eq!(focus.distance(1, &siblings), 0);
    }

    #[test]
    fn distance_from_focused_sibling() {
        let siblings = ids(4);
        let focus = FocusPath::new(vec![ContainerId(1)]);
        assert_eq!(focus.distance(0, &siblings), 1);
        assert_eq!(focus.distance(1, &siblings), 0);
        assert_eq!(focus.distance(2, &siblings), 1);
        assert_eq!(focus.distance(3, &siblings), 2);
    }

    #[test]
    fn no_focused_sibling_uses_index_plus_one() {
        let siblings = ids(3);
        let focus = FocusPath::default();
        assert_eq!(focus.distance(0, &siblings), 1);
        assert_eq!(focus.distance(1, &siblings), 2);
        assert_eq!(focus.distance(2, &siblings), 3);
    }

    #[test]
    fn focus_on_ancestor_not_in_siblings_counts_as_none() {
        let siblings = ids(3);
        // Focus path points at an id outside this arrangement.
        let focus = FocusPath::new(vec![ContainerId(99)]);
        assert_eq!(focus.distance(0, &siblings), 1);
        assert_eq!(focus.distance(2, &siblings), 3);
    }
}
