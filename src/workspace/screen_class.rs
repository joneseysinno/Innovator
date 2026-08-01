pub mod from_width;

/// Responsive workspace breakpoint class.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ScreenClass {
    /// All 3 pages visible simultaneously.
    Desktop,
    /// Navigation collapses to icon strip (Phase 6+).
    Tablet,
    /// One page fills the screen (Phase 6+).
    Mobile,
}
