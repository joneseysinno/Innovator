//! Navigation page — WallListIO / WallSummaryIO (Phase 2).

pub mod build;
pub mod wall_list;
pub mod wall_summary;

pub use build::build_navigation;
pub use wall_list::WallListIO;
pub use wall_summary::WallSummaryIO;
