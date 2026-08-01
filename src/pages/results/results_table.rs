pub mod build;

use hyper_ui::particles::Particle;

/// Check results table pod.
#[derive(Debug, Clone)]
pub struct ResultsTableIO {
    pub particle: Particle,
}

pub use build::build_results_table;
