//! Result returned from workspace event handlers.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandleResult {
    /// Event was consumed; caller must rebuild the particle tree.
    Rebuild,
    /// Event was consumed; no rebuild needed (e.g. status text update only).
    Consumed,
    /// Event was not handled by this workspace.
    Ignored,
}
