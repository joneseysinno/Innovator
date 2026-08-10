/// What an arrangement does when child demands exceed the axis budget.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Overflow {
    /// Axis is unbounded. Excess becomes scroll extent. Nothing is demoted.
    Scroll,
    /// Axis is fixed. Excess is resolved by demoting the container furthest
    /// from focus, one step at a time, until it fits.
    Cascade,
}
