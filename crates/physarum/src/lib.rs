//! Physarum / slime-mold conductivity-reinforcement network.
//!
//! Generic over any `N: Eq + Hash + Copy` node id. No knowledge of hypergraphs,
//! UI, or Innovator domains — flux is supplied by the caller.

mod network;
mod step;

pub use network::PhysarumNetwork;
