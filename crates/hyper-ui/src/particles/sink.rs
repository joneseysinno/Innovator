use crate::geom::Vec2;
use crate::layout::LayoutBox;
use crate::particles::{Particle, ParticleId};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PointerKind {
    Move,
    Down,
    Up,
    Scroll { delta_y: f32 },
}

#[derive(Debug, Clone)]
pub struct SinkParticle {
    pub id: ParticleId,
    pub layout: LayoutBox,
    pub child: Option<Box<Particle>>,
    pub flex: f32,
}

impl SinkParticle {
    pub fn new() -> Self {
        Self {
            id: ParticleId::fresh(),
            layout: LayoutBox::default(),
            child: None,
            flex: 1.0,
        }
    }

    pub fn with_child(mut self, child: Particle) -> Self {
        self.child = Some(Box::new(child));
        self
    }

    pub fn measure(&self, available: Vec2) -> Vec2 {
        self.child
            .as_ref()
            .map(|c| crate::layout::measure_particle(c, available))
            .unwrap_or(available)
    }
}

impl Default for SinkParticle {
    fn default() -> Self {
        Self::new()
    }
}
