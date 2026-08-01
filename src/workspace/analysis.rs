//! Analysis workspace (optional header + three pages).
//! Hosts many analysis types over time; starts with special concrete wall.

pub mod build_pages;
pub mod kind;
pub mod new;
pub mod pages_pod_tree;

pub use kind::AnalysisKind;

use crate::engine::AnalysisOutput;
use crate::workspace::analysis_action::AnalysisAction;
use crate::workspace::field_builder_draft::{BuilderFieldSlot, FieldBuilderDraft};
use crate::workspace::header::WorkspaceHeader;
use crate::workspace::page::Page;
use crate::workspace::signal::WorkspaceSignal;
use crate::workspace::size_class::SizeClass;
use crate::workspace::tab::WorkspaceTab;
use hyper_ui::{InMemoryWorldSpatial, ParticleId, PodTree};
use hypernode::{Graph, Node, NodeId};
use std::collections::HashMap;

pub struct AnalysisWorkspace {
    pub tab: WorkspaceTab,
    /// Action header — present for this workspace kind.
    pub header: Option<WorkspaceHeader>,
    pub pages: Vec<Page>,
    pub pod_tree: PodTree,
    /// Which analysis type is active in this workspace.
    pub active_analysis: AnalysisKind,
    /// In-memory wall HyperNodes for this analysis.
    pub graph: Graph,
    pub active_wall: Option<NodeId>,
    /// Sink id → wall NodeId (Navigation WallListIO).
    pub wall_sinks: HashMap<ParticleId, NodeId>,
    /// Trigger id → signal (Navigation "+ New Wall").
    pub nav_triggers: HashMap<ParticleId, WorkspaceSignal>,
    /// Field id → prop key (InputFormIO).
    pub field_props: HashMap<ParticleId, String>,
    /// Field ids that should commit as U8.
    pub u8_fields: HashMap<ParticleId, ()>,
    /// Analysis-page triggers (FieldBuilder, type chips).
    pub analysis_actions: HashMap<ParticleId, AnalysisAction>,
    /// FieldBuilder draft field slots.
    pub builder_slots: HashMap<ParticleId, BuilderFieldSlot>,
    /// Promote trigger → custom prop key.
    pub promote_props: HashMap<ParticleId, String>,
    /// Open inline field builder draft.
    pub field_builder: Option<FieldBuilderDraft>,
    /// Input form SizeClass from pod width.
    pub input_size_class: SizeClass,
    /// Wall view pan/zoom sink.
    pub wall_view_sink: Option<ParticleId>,
    /// Layer A spatial for the active wall section.
    pub wall_spatial: InMemoryWorldSpatial,
    /// Last pointer pos for pan delta.
    pub wall_view_last_pos: Option<hyper_ui::Vec2>,
    pub wall_view_panning: bool,
    /// Latest ResultsNode for the active wall.
    pub last_results: Option<Node>,
    /// Parsed engine summary for StatusIO.
    pub last_analysis: Option<AnalysisOutput>,
    /// Results page triggers (Export PDF).
    pub results_triggers: HashMap<ParticleId, WorkspaceSignal>,
}

impl AnalysisWorkspace {
    pub fn status_id(&self) -> Option<hyper_ui::ParticleId> {
        self.header.as_ref().map(|h| h.status_id)
    }
}
