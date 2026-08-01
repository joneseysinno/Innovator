use crate::geom::Vec2;
use crate::layout::LayoutBox;
use crate::particles::{Particle, ParticleId};

/// View particle — fills available space; host supplies a custom render callback via IO.
#[derive(Debug, Clone)]
pub struct ViewParticle {
    pub id: ParticleId,
    pub layout: LayoutBox,
    pub child: Option<Box<Particle>>,
    pub label: String,
}

impl ViewParticle {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            id: ParticleId::fresh(),
            layout: LayoutBox::default(),
            child: None,
            label: label.into(),
        }
    }

    pub fn measure(&self, available: Vec2) -> Vec2 {
        // Greedy — fills all available space.
        Vec2::new(available.x.max(0.0), available.y.max(0.0))
    }
}
