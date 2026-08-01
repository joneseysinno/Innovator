use crate::layout::LayoutBox;
use crate::particles::{Particle, ParticleId};

/// Placeholder slot for deferred content.
#[derive(Debug, Clone)]
pub struct SlotParticle {
    pub id: ParticleId,
    pub layout: LayoutBox,
    pub child: Option<Box<Particle>>,
    pub flex: f32,
}

impl SlotParticle {
    pub fn new() -> Self {
        Self {
            id: ParticleId::fresh(),
            layout: LayoutBox::default(),
            child: None,
            flex: 1.0,
        }
    }
}

impl Default for SlotParticle {
    fn default() -> Self {
        Self::new()
    }
}
