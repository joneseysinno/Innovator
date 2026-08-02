pub mod build;

use crate::domains::structural::{AnalysisAction, BuilderFieldSlot};
use hyper_ui::particles::{Particle, ParticleId};
use std::collections::HashMap;

/// Inline field builder (no modal).
#[derive(Debug, Clone)]
pub struct FieldBuilderIO {
    pub particle: Particle,
    pub actions: HashMap<ParticleId, AnalysisAction>,
    pub slots: HashMap<ParticleId, BuilderFieldSlot>,
}

pub use build::build_field_builder;
