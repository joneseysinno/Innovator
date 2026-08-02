//! Analysis workspace (optional header + page tree of spatial containers).
//! Hosts many analysis types over time; starts with special concrete wall.

pub mod build_icon_rail;
pub mod build_page_header;
pub mod build_pages;
pub mod io_kind;
pub mod kind;
pub mod new;
pub mod page_signal;
pub mod templates;

pub use io_kind::IoKind;
pub use kind::AnalysisKind;
pub use page_signal::PageSignal;

use crate::engine::AnalysisOutput;
use crate::workspace::analysis_action::AnalysisAction;
use crate::workspace::field_builder_draft::{BuilderFieldSlot, FieldBuilderDraft};
use crate::workspace::header::WorkspaceHeader;
use crate::workspace::signal::WorkspaceSignal;
use crate::workspace::size_class::SizeClass;
use crate::workspace::tab::WorkspaceTab;
use hyper_ui::{InMemoryWorldSpatial, PageId, PageTree, ParticleId, PodId};
use hypernode::{Graph, Node, NodeId};
use std::collections::HashMap;

pub struct AnalysisWorkspace {
    pub tab: WorkspaceTab,
    /// Action header — present for this workspace kind.
    pub header: Option<WorkspaceHeader>,
    /// Page-level binary split tree.
    pub page_tree: PageTree,
    /// IO assignment: PageId → Vec<(PodId, IoKind)>.
    pub page_ios: HashMap<PageId, Vec<(PodId, IoKind)>>,
    /// Next PageId for split-created pages.
    pub next_page_id: u32,
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
    /// Icon rail triggers: ParticleId → (page_id, pod_id).
    pub icon_rail_triggers: HashMap<ParticleId, (PageId, PodId)>,
    /// Pod title-bar triggers: ParticleId → PodId (toggle collapse).
    pub pod_collapse_triggers: HashMap<ParticleId, PodId>,
    /// Page header split triggers: ParticleId → PageId.
    pub page_split_triggers: HashMap<ParticleId, PageId>,
    /// Analysis page header status source (live ratios).
    pub analysis_header_status_id: Option<ParticleId>,
}

impl AnalysisWorkspace {
    pub fn status_id(&self) -> Option<hyper_ui::ParticleId> {
        self.header.as_ref().map(|h| h.status_id)
    }

    /// Resolve a pod rect for an IoKind across the page tree.
    pub fn io_rect(
        &self,
        pages_area: hyper_ui::Rect,
        kind: IoKind,
    ) -> Option<hyper_ui::Rect> {
        for (page_id, page_rect) in self.page_tree.leaf_rects(pages_area) {
            let page = self.page_tree.find(page_id)?;
            let content = page.content_rect(page_rect);
            let ios = self.page_ios.get(&page_id)?;
            let leaves = page.pods.layout(content);
            for (pod_id, io) in ios {
                if *io == kind {
                    if let Some((_, r)) = leaves.iter().find(|(id, _)| *id == *pod_id) {
                        return Some(*r);
                    }
                }
            }
        }
        None
    }
}
