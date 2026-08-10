//! Layout persistence — container tree + Overrides. Never resolved/rect.

pub mod apply;
pub mod path;
pub mod save;
pub mod types;

#[cfg(test)]
mod tests;

pub(crate) use apply::restore_workspaces;
pub use path::LAYOUT_PATH;
pub(crate) use save::save_layout;
pub use types::PersistedSession;


use std::path::Path;

/// Load a saved session, or `None` if missing/corrupt (caller falls back to seeds).
pub fn load_layout(path: impl AsRef<Path>) -> Option<PersistedSession> {
    let data = std::fs::read_to_string(path).ok()?;
    serde_json::from_str(&data).ok()
}
