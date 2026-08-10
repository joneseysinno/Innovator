/// How much of a container is rendered.
///
/// Derived `Ord` is load-bearing: `Shown < Collapsed < Hidden` is the demotion
/// ladder, so "demote one step" is a successor and "at least as visible as" is
/// a comparison.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Visibility {
    /// Chrome and content both rendered.
    Shown,
    /// Chrome only — title bar, tab, or rail icon. Content not built.
    Collapsed,
    /// Not rendered. Reachable only through a rail, tab, or menu.
    Hidden,
}

#[cfg(test)]
mod tests {
    use super::Visibility;

    #[test]
    fn demotion_ladder_order() {
        assert!(Visibility::Shown < Visibility::Collapsed);
        assert!(Visibility::Collapsed < Visibility::Hidden);
        assert!(Visibility::Shown < Visibility::Hidden);
    }

    #[test]
    fn successor_via_ord() {
        let steps = [Visibility::Shown, Visibility::Collapsed, Visibility::Hidden];
        assert_eq!(steps.windows(2).all(|w| w[0] < w[1]), true);
    }
}
