use crate::layout::LayoutBox;
use crate::particles::find::{find_mut_recursive, find_recursive};
use crate::particles::{
    FieldParticle, ParticleId, SignalParticle, SinkParticle, SlotParticle, SourceParticle,
    StackParticle, SurfaceParticle, TriggerParticle, ViewParticle, ViewportParticle,
};

/// Retained particle tree node.
#[derive(Debug, Clone)]
pub enum Particle {
    Surface(SurfaceParticle),
    Stack(StackParticle),
    Slot(SlotParticle),
    Source(SourceParticle),
    Field(FieldParticle),
    Trigger(TriggerParticle),
    Sink(SinkParticle),
    View(ViewParticle),
    Signal(SignalParticle),
    Viewport(ViewportParticle),
}

impl Particle {
    pub fn id(&self) -> ParticleId {
        match self {
            Self::Surface(p) => p.id,
            Self::Stack(p) => p.id,
            Self::Slot(p) => p.id,
            Self::Source(p) => p.id,
            Self::Field(p) => p.id,
            Self::Trigger(p) => p.id,
            Self::Sink(p) => p.id,
            Self::View(p) => p.id,
            Self::Signal(p) => p.id,
            Self::Viewport(p) => p.id,
        }
    }

    pub fn layout(&self) -> LayoutBox {
        match self {
            Self::Surface(p) => p.layout,
            Self::Stack(p) => p.layout,
            Self::Slot(p) => p.layout,
            Self::Source(p) => p.layout,
            Self::Field(p) => p.layout,
            Self::Trigger(p) => p.layout,
            Self::Sink(p) => p.layout,
            Self::View(p) => p.layout,
            Self::Signal(p) => p.layout,
            Self::Viewport(p) => p.layout,
        }
    }

    pub fn set_layout(&mut self, layout: LayoutBox) {
        match self {
            Self::Surface(p) => p.layout = layout,
            Self::Stack(p) => p.layout = layout,
            Self::Slot(p) => p.layout = layout,
            Self::Source(p) => p.layout = layout,
            Self::Field(p) => p.layout = layout,
            Self::Trigger(p) => p.layout = layout,
            Self::Sink(p) => p.layout = layout,
            Self::View(p) => p.layout = layout,
            Self::Signal(p) => p.layout = layout,
            Self::Viewport(p) => p.layout = layout,
        }
    }

    pub fn is_interactive(&self) -> bool {
        matches!(
            self,
            Self::Field(_) | Self::Trigger(_) | Self::Sink(_) | Self::Viewport(_)
        )
    }

    pub fn find_mut(&mut self, id: ParticleId) -> Option<&mut Particle> {
        find_mut_recursive(self, id)
    }

    pub fn find(&self, id: ParticleId) -> Option<&Particle> {
        find_recursive(self, id)
    }
}
