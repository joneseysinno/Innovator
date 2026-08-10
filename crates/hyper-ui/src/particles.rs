//! Layer B — retained-mode UI particle tree.

mod dirty;
mod find;
mod hit_test;
mod id;
mod particle;
mod slot;
mod tree;

pub mod field;
pub mod signal;
pub mod sink;
pub mod source;
pub mod stack;
pub mod surface;
pub mod trigger;
pub mod view;
pub mod viewport;

pub use dirty::DirtyFlags;
pub use field::{FieldParticle, FieldState, FieldValue, NumericValue};
pub use id::ParticleId;
pub use particle::Particle;
pub use signal::SignalParticle;
pub use sink::{PointerKind, SinkParticle};
pub use slot::SlotParticle;
pub use source::{SourceParticle, SourceStyle};
pub use stack::{StackAlign, StackDirection, StackParticle};
pub use surface::SurfaceParticle;
pub use tree::ParticleTree;
pub use trigger::{TriggerParticle, TriggerState};
pub use view::ViewParticle;
pub use viewport::ViewportParticle;
