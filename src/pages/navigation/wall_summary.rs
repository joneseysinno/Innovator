pub mod build;

use hyper_ui::particles::Particle;

/// Read-only summary of the active wall.
#[derive(Debug, Clone)]
pub struct WallSummaryIO {
    pub particle: Particle,
}

pub use build::build_wall_summary;
