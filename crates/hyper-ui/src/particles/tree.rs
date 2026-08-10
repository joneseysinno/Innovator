mod clear_dirty;
mod find;
mod find_mut;
mod hit_test;
mod mark_all_dirty;
mod mark_layout;
mod mark_paint;
mod mark_text;
mod new;
mod scroll;

use crate::particles::{DirtyFlags, Particle};

/// Retained particle tree root + dirty tracking.
#[derive(Debug, Default)]
pub struct ParticleTree {
    pub root: Option<Particle>,
    pub dirty: DirtyFlags,
    generation: u64,
}
