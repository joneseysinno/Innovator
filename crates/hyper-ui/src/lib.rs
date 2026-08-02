//! Retained-mode wgpu UI library for spatial hypergraph applications.
//!
//! Two layers share one wgpu device / queue / surface:
//! - **Layer A** ([`renderer`]): scene camera, node/edge pipelines, spatial culling
//! - **Layer B** ([`particles`] + [`ui`]): particle tree, layout, input, text
//!
//! Container hierarchy: [`workspace`] → [`page`] → [`pod`].

pub mod engineer;
pub mod geom;
pub mod input;
pub mod layout;
pub mod page;
pub mod particles;
pub mod pod;
pub mod renderer;
pub mod seam;
pub mod text;
pub mod ui;
pub mod workspace;

pub use engineer::{engineer_input, engineer_input_readonly, EngineerInput};
pub use geom::{Rect, UVec2, Vec2, WorldRect};
pub use input::{InputRouter, UiEvent};
pub use layout::{LayoutBox, LayoutEngine};
pub use page::{
    IconRailConfig, IconRailSide, PageHeaderConfig, PageHeaderSlots, PageId, PageNode, PageSeamId,
    PageSide, PageTree,
};
pub use particles::{
    DirtyFlags, FieldParticle, FieldValue, Particle, ParticleId, ParticleTree, PointerKind,
    SinkParticle, SourceParticle, StackParticle, SurfaceParticle, TriggerParticle, ViewParticle,
};
pub use pod::{Pod, PodDivider, PodDividerRenderer, PodId, PodList, COLLAPSED_HEIGHT};
pub use renderer::{
    cull_nodes_from_infinite_db, EdgeDrawCmd, EdgeKindGpu, HyperRenderer, InMemorySpatial,
    InMemoryWorldSpatial, SceneCamera, SceneNode, SceneRenderer, SpatialSource, WorldEdge,
};
pub use seam::{SeamDirection, SeamDrawCmd, SeamRatioAction, SeamRenderer};
pub use text::TextRenderer;
pub use ui::{apply_signal_text, UiRenderer};
pub use workspace::WorkspaceShell;
