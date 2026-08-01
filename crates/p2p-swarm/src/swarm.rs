pub mod new;
pub mod with_local_id;

use crate::peer_id::PeerId;

/// Minimal swarm handle. Exchange APIs arrive in a later phase.
#[derive(Debug, Default)]
pub struct Swarm {
    pub local_id: Option<PeerId>,
}
