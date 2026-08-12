//! Page modules — Navigation, Analysis, Results (Phase 2–4).

pub mod analysis;
pub mod generic {
    pub mod template;
}
pub mod navigation;
pub mod placeholder;
pub mod registry;
pub mod results;
pub mod template;

pub use analysis::build_analysis;
pub use navigation::build_navigation;
pub use placeholder::{build_empty_pod, build_page_placeholder};
pub use results::build_results;
