//! Optional per-workspace action header (not app chrome).

pub mod build;
pub mod height;

pub use build::{build_header, WorkspaceHeader};
pub use height::HEADER_HEIGHT;
