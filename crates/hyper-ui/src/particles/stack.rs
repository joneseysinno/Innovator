use crate::geom::Vec2;
use crate::layout::LayoutBox;
use crate::particles::{Particle, ParticleId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StackDirection {
    Row,
    Column,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StackAlign {
    Start,
    Center,
    End,
    Stretch,
}

#[derive(Debug, Clone)]
pub struct StackParticle {
    pub id: ParticleId,
    pub layout: LayoutBox,
    pub children: Vec<Particle>,
    pub direction: StackDirection,
    pub gap: f32,
    pub align: StackAlign,
}

impl StackParticle {
    pub fn row(children: Vec<Particle>) -> Self {
        Self {
            id: ParticleId::fresh(),
            layout: LayoutBox::default(),
            children,
            direction: StackDirection::Row,
            gap: 8.0,
            align: StackAlign::Center,
        }
    }

    pub fn column(children: Vec<Particle>) -> Self {
        Self {
            id: ParticleId::fresh(),
            layout: LayoutBox::default(),
            children,
            direction: StackDirection::Column,
            gap: 8.0,
            align: StackAlign::Stretch,
        }
    }

    pub fn with_gap(mut self, gap: f32) -> Self {
        self.gap = gap;
        self
    }

    pub fn with_align(mut self, align: StackAlign) -> Self {
        self.align = align;
        self
    }

    pub fn measure(&self, available: Vec2) -> Vec2 {
        if self.children.is_empty() {
            return Vec2::ZERO;
        }
        let gaps = self.gap * (self.children.len().saturating_sub(1) as f32);
        match self.direction {
            StackDirection::Row => {
                let mut width = gaps;
                let mut height = 0.0f32;
                let each = if available.x.is_finite() {
                    ((available.x - gaps) / self.children.len() as f32).max(0.0)
                } else {
                    120.0
                };
                for child in &self.children {
                    let sz = crate::layout::measure_particle(
                        child,
                        Vec2::new(each, available.y),
                    );
                    width += sz.x;
                    height = height.max(sz.y);
                }
                Vec2::new(width, height)
            }
            StackDirection::Column => {
                let mut height = gaps;
                let mut width = 0.0f32;
                let avail_x = if available.x.is_finite() {
                    available.x
                } else {
                    400.0
                };
                for child in &self.children {
                    let sz = crate::layout::measure_particle(child, Vec2::new(avail_x, 10_000.0));
                    height += sz.y;
                    width = width.max(sz.x);
                }
                Vec2::new(width.min(avail_x), height)
            }
        }
    }
}
