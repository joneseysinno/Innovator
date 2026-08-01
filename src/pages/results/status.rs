pub mod build;

use crate::workspace::signal::WorkspaceSignal;
use hyper_ui::particles::{Particle, ParticleId};
use std::collections::HashMap;

/// Status / summary pod with Export PDF trigger.
#[derive(Debug, Clone)]
pub struct StatusIO {
    pub particle: Particle,
    pub triggers: HashMap<ParticleId, WorkspaceSignal>,
}

pub use build::build_status;
