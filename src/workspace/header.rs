//! Optional per-workspace action header (not app chrome).

pub mod build;
pub mod build_pm;
pub mod height;

pub use build::{build_header, WorkspaceHeader};
pub use build_pm::build_pm_header;
pub use height::HEADER_HEIGHT;
