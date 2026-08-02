//! Retained-mode wgpu UI library for spatial hypergraph applications.
//!
//! Two layers share one wgpu device / queue / surface:
//! - **Layer A** ([`renderer`]): scene camera, node/edge pipelines, spatial culling
//! - **Layer B** ([`particles`] + [`ui`]): particle tree, layout, input, text

pub mod engineer;
pub mod geom;
pub mod input;
pub mod layout;
pub mod page_tree;
pub mod particles;
pub mod renderer;
pub mod seam;
pub mod text;
pub mod ui;

pub use engineer::{engineer_input, engineer_input_readonly, EngineerInput};
pub use geom::{Rect, UVec2, Vec2, WorldRect};
pub use input::{InputRouter, UiEvent};
pub use layout::{LayoutBox, LayoutEngine};
pub use page_tree::{
    IconRailConfig, IconRailSide, PageHeaderConfig, PageHeaderSlots, PageId, PageNode, PageSeamId,
    PageSide, PageTree,
};
pub use particles::{
    DirtyFlags, FieldParticle, FieldValue, Particle, ParticleId, ParticleTree, PointerKind,
    SinkParticle, SourceParticle, StackParticle, SurfaceParticle, TriggerParticle, ViewParticle,
};
pub use renderer::{
    cull_nodes_from_infinite_db, EdgeDrawCmd, EdgeKindGpu, HyperRenderer, InMemorySpatial,
    InMemoryWorldSpatial, SceneCamera, SceneNode, SceneRenderer, SpatialSource, WorldEdge,
};
pub use seam::{PodTree, SeamDirection, SeamDrawCmd, SeamRatioAction, SeamRenderer};
pub use text::TextRenderer;
pub use ui::{apply_signal_text, UiRenderer};
