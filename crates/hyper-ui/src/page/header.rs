/// Declarative config for a page's optional header bar.
#[derive(Debug, Clone)]
pub struct PageHeaderConfig {
    /// Typically 32–64 px; variable per page.
    pub height: f32,
    /// What the application populates.
    pub slots: PageHeaderSlots,
}

/// Header content policy — geometry only in the library.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageHeaderSlots {
    /// Application builds the header particle subtree freely.
    Custom,
    /// No pre-built slots — application injects its own particle.
    None,
}
