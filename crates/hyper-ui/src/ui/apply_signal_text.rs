use crate::particles::{Particle, ParticleId, ParticleTree};

/// Apply a Signal update from a background thread to a source particle by id.
pub fn apply_signal_text(tree: &mut ParticleTree, source_id: ParticleId, new_text: String) {
    if let Some(Particle::Source(s)) = tree.find_mut(source_id) {
        s.set_text(new_text);
        tree.mark_text(source_id);
    }
}
