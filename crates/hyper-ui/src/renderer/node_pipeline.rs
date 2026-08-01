//! Instanced SDF rounded-rect pipeline for scene nodes and UI surfaces.

mod globals;
mod instance;
mod pipeline;

pub use instance::NodeInstance;
pub use pipeline::NodePipeline;

pub(crate) use globals::Globals;
