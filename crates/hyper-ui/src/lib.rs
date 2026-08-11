//! Retained-mode wgpu UI library for spatial hypergraph applications.
//!
//! Two layers share one wgpu device / queue / surface:
//! - **Layer A** ([`renderer`]): scene camera, node/edge pipelines, spatial culling
//! - **Layer B** ([`particles`] + [`ui`]): particle tree, layout, input, text
//!
//! Container hierarchy: [`workspace`] → [`page`] → [`pod`].

pub mod container;
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

pub use container::{ContainerId, ContainerState, Extent, FocusPath, Visibility};
pub use geom::{letterbox_rect, Rect, UVec2, Vec2, WorldRect};
pub use input::{InputRouter, UiEvent};
pub use layout::{
    resolve, Axis, DemotionLadder, InputClass, LayoutBox, LayoutEngine, Overflow, Overrides,
    ResolveReport, SizeClass, Viewport, CLASS_SLOP, PAGE_LADDER, POD_LADDER, PROMOTE_SLOP,
    UNDERFLOW_FACTOR, WORKSPACE_LADDER,
};
pub use page::{
    sync_from_page_tree, IconRailConfig, IconRailSide, PageHeaderConfig, PageHeaderSlots, PageId,
    PageNode, PageSeamId, PageSide, PageTree,
};
pub use particles::{
    DirtyFlags, FieldParticle, FieldValue, Particle, ParticleId, ParticleTree, PointerKind,
    SinkParticle, SourceParticle, StackParticle, SurfaceParticle, TriggerParticle, ViewParticle,
    ViewportParticle,
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
