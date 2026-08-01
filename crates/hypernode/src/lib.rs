//! HyperNode trait and Hilbert spatial indexing.
//!
//! Publishable standalone crate — no knowledge of Innovator or UI.

pub mod edge_kind;
pub mod graph;
pub mod hilbert;
pub mod hyper_edge;
pub mod hyper_node;
pub mod ids;
pub mod node;
pub mod prop_value;
pub mod space_class;

#[cfg(test)]
mod tests;

pub use edge_kind::EdgeKind;
pub use graph::Graph;
pub use hilbert::{hilbert_decode_2d, hilbert_encode_2d, world_to_hilbert_cell};
pub use hyper_edge::HyperEdge;
pub use hyper_node::HyperNode;
pub use ids::{EdgeId, NodeId};
pub use node::Node;
pub use prop_value::PropValue;
pub use space_class::SpaceClass;
