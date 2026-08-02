/// Which edge of the page content area hosts the icon rail.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum IconRailSide {
    Left,
    Right,
}

/// Narrow column pinned beside the page's pod content.
#[derive(Debug, Clone)]
pub struct IconRailConfig {
    pub side: IconRailSide,
    /// Typically 32–36 px.
    pub width: f32,
}
