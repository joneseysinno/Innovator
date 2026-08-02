pub mod build;
pub mod field_defs;
pub mod section;

use crate::domains::structural::{AnalysisAction, BuilderFieldSlot};
use hyper_ui::particles::{Particle, ParticleId};
use std::collections::HashMap;

/// Left Analysis pod — standard + custom fields.
#[derive(Debug, Clone)]
pub struct InputFormIO {
    pub particle: Particle,
    pub field_props: HashMap<ParticleId, String>,
    pub u8_fields: HashMap<ParticleId, ()>,
    pub actions: HashMap<ParticleId, AnalysisAction>,
    pub builder_slots: HashMap<ParticleId, BuilderFieldSlot>,
    pub promote_props: HashMap<ParticleId, String>,
}

pub use build::build_input_form;
