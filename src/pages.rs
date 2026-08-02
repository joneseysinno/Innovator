//! Page modules — Navigation, Analysis, Results (Phase 2–4).

pub mod analysis;
pub mod navigation;
pub mod placeholder;
pub mod results;

pub use analysis::build_analysis;
pub use navigation::build_navigation;
pub use placeholder::{build_empty_pod, build_page_placeholder};
pub use results::build_results;
