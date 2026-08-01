//! P2P transport for spatial hypergraph applications.
//!
//! Stub crate — full swarm wiring lands in a future phase.

pub mod peer_id;
pub mod swarm;

pub use peer_id::PeerId;
pub use swarm::Swarm;
