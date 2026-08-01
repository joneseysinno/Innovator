pub mod build;
pub mod build_section;

use hyper_ui::particles::{Particle, ParticleId};
use hyper_ui::InMemoryWorldSpatial;

/// Right Analysis pod — live cross-section (Layer A).
#[derive(Debug, Clone)]
pub struct WallViewIO {
    pub particle: Particle,
    pub sink_id: ParticleId,
    pub spatial: InMemoryWorldSpatial,
}

pub use build::build_wall_view;
pub use build_section::build_section_spatial;
