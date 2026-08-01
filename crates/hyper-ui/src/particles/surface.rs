use crate::geom::Vec2;
use crate::layout::LayoutBox;
use crate::particles::{Particle, ParticleId};

#[derive(Debug, Clone)]
pub struct SurfaceParticle {
    pub id: ParticleId,
    pub layout: LayoutBox,
    pub child: Option<Box<Particle>>,
    pub color: [f32; 4],
    pub border_color: [f32; 4],
    pub border_radius: f32,
    pub border_width: f32,
    pub padding: f32,
    pub clip: bool,
}

impl SurfaceParticle {
    pub fn new(color: [f32; 4]) -> Self {
        Self {
            id: ParticleId::fresh(),
            layout: LayoutBox::default(),
            child: None,
            color,
            border_color: [0.0, 0.0, 0.0, 0.0],
            border_radius: 4.0,
            border_width: 0.0,
            padding: 8.0,
            clip: false,
        }
    }

    pub fn with_child(mut self, child: Particle) -> Self {
        self.child = Some(Box::new(child));
        self
    }

    pub fn with_padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    pub fn with_radius(mut self, radius: f32) -> Self {
        self.border_radius = radius;
        self
    }

    pub fn with_border(mut self, color: [f32; 4], width: f32) -> Self {
        self.border_color = color;
        self.border_width = width;
        self
    }

    pub fn measure(&self, available: Vec2) -> Vec2 {
        let pad = self.padding * 2.0;
        let inner = Vec2::new((available.x - pad).max(0.0), (available.y - pad).max(0.0));
        let child_size = self
            .child
            .as_ref()
            .map(|c| crate::layout::measure_particle(c, inner))
            .unwrap_or(Vec2::ZERO);
        Vec2::new(child_size.x + pad, child_size.y + pad)
    }
}
