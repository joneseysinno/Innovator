//! Results page — ResultsTableIO / StatusIO (Phase 4).

pub mod build;
pub mod results_table;
pub mod status;
pub mod template;

pub use build::build_results;
pub use results_table::ResultsTableIO;
pub use status::StatusIO;
