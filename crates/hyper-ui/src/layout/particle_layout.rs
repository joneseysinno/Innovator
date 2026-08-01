use crate::geom::{Rect, Vec2};
use crate::particles::Particle;

pub trait ParticleLayout {
    fn measure(&self, available: Vec2, children: &[Particle]) -> Vec2;
    fn arrange(&mut self, rect: Rect, children: &mut [Particle]);
}
