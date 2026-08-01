use super::Swarm;
use crate::peer_id::PeerId;

impl Swarm {
    pub fn with_local_id(id: impl Into<String>) -> Self {
        Self {
            local_id: Some(PeerId(id.into())),
        }
    }
}
