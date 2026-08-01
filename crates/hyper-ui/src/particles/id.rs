use std::sync::atomic::{AtomicU64, Ordering};

static NEXT_PARTICLE_ID: AtomicU64 = AtomicU64::new(1);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ParticleId(pub u64);

impl ParticleId {
    pub fn fresh() -> Self {
        Self(NEXT_PARTICLE_ID.fetch_add(1, Ordering::Relaxed))
    }
}
