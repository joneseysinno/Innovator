//! Graph View page — canvas + inspector for the live composed-view graph.

pub mod build;
pub mod force;
pub mod scope;
pub mod spatial;
pub mod template;

pub use build::{build_canvas_pod, build_graph_view, build_inspector_pod, hit_test, sync_graph_view};
