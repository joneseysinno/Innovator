use crate::layout::LayoutBox;
use crate::particles::ParticleId;

/// Signal particle — invisible carrier that marks source particles dirty when a
/// hyperedge Signal arrives from a background thread / engine.
#[derive(Debug, Clone)]
pub struct SignalParticle {
    pub id: ParticleId,
    pub layout: LayoutBox,
    pub name: String,
    pub payload: Option<String>,
}

impl SignalParticle {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            id: ParticleId::fresh(),
            layout: LayoutBox::default(),
            name: name.into(),
            payload: None,
        }
    }
}
