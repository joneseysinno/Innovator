//! ACI 318 analysis logic (Function HyperNode) — Phase 4.

pub mod check_result;
pub mod checks;
pub mod run;

pub use check_result::CheckResult;
pub use run::{run_analysis, AnalysisOutput};
