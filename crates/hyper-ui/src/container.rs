//! Shared container primitives — identity, visibility, extent, focus.
//!
//! Domain-free. Identical at workspace, page, and pod levels.

mod extent;
mod focus;
mod id;
mod state;
mod visibility;

pub use extent::Extent;
pub use focus::FocusPath;
pub use id::ContainerId;
pub use state::ContainerState;
pub use visibility::Visibility;
