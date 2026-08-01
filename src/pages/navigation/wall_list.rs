pub mod build;
pub mod build_row;

use hyper_ui::particles::{Particle, ParticleId};
use hypernode::NodeId;
use std::collections::HashMap;

use crate::workspace::signal::WorkspaceSignal;

/// Wall list pod — selectable rows + New Wall trigger.
#[derive(Debug, Clone)]
pub struct WallListIO {
    pub particle: Particle,
    pub sinks: HashMap<ParticleId, NodeId>,
    pub triggers: HashMap<ParticleId, WorkspaceSignal>,
}

pub use build::build_wall_list;
