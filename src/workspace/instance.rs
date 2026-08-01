use crate::workspace::analysis::AnalysisWorkspace;
use crate::workspace::analysis_action::AnalysisAction;
use crate::workspace::empty::EmptyWorkspace;
use crate::workspace::field_builder_draft::BuilderFieldSlot;
use crate::workspace::header::WorkspaceHeader;
use crate::workspace::kind::WorkspaceKind;
use crate::workspace::signal::WorkspaceSignal;
use crate::workspace::tab::WorkspaceTab;
use crate::workspace::workspace_id::WorkspaceId;
use hyper_ui::{InMemoryWorldSpatial, ParticleId, PodTree};
use hypernode::NodeId;
use std::collections::HashMap;

/// Hosted workspace instance inside the app shell.
pub enum WorkspaceInstance {
    Analysis(AnalysisWorkspace),
    Empty(EmptyWorkspace),
}

impl WorkspaceInstance {
    pub fn tab(&self) -> &WorkspaceTab {
        match self {
            Self::Analysis(w) => &w.tab,
            Self::Empty(w) => &w.tab,
        }
    }

    pub fn id(&self) -> WorkspaceId {
        self.tab().id
    }

    pub fn kind(&self) -> WorkspaceKind {
        self.tab().kind
    }

    pub fn header(&self) -> Option<&WorkspaceHeader> {
        match self {
            Self::Analysis(w) => w.header.as_ref(),
            Self::Empty(_) => None,
        }
    }

    pub fn pod_tree(&self) -> Option<&PodTree> {
        match self {
            Self::Analysis(w) => Some(&w.pod_tree),
            Self::Empty(_) => None,
        }
    }

    pub fn pod_tree_mut(&mut self) -> Option<&mut PodTree> {
        match self {
            Self::Analysis(w) => Some(&mut w.pod_tree),
            Self::Empty(_) => None,
        }
    }

    pub fn status_id(&self) -> Option<hyper_ui::ParticleId> {
        match self {
            Self::Analysis(w) => w.status_id(),
            Self::Empty(_) => None,
        }
    }

    pub fn wall_sinks(&self) -> Option<&HashMap<ParticleId, NodeId>> {
        match self {
            Self::Analysis(w) => Some(&w.wall_sinks),
            Self::Empty(_) => None,
        }
    }

    pub fn nav_triggers(&self) -> Option<&HashMap<ParticleId, WorkspaceSignal>> {
        match self {
            Self::Analysis(w) => Some(&w.nav_triggers),
            Self::Empty(_) => None,
        }
    }

    pub fn field_props(&self) -> Option<&HashMap<ParticleId, String>> {
        match self {
            Self::Analysis(w) => Some(&w.field_props),
            Self::Empty(_) => None,
        }
    }

    pub fn analysis_actions(&self) -> Option<&HashMap<ParticleId, AnalysisAction>> {
        match self {
            Self::Analysis(w) => Some(&w.analysis_actions),
            Self::Empty(_) => None,
        }
    }

    pub fn builder_slots(&self) -> Option<&HashMap<ParticleId, BuilderFieldSlot>> {
        match self {
            Self::Analysis(w) => Some(&w.builder_slots),
            Self::Empty(_) => None,
        }
    }

    pub fn promote_props(&self) -> Option<&HashMap<ParticleId, String>> {
        match self {
            Self::Analysis(w) => Some(&w.promote_props),
            Self::Empty(_) => None,
        }
    }

    pub fn wall_view_sink(&self) -> Option<ParticleId> {
        match self {
            Self::Analysis(w) => w.wall_view_sink,
            Self::Empty(_) => None,
        }
    }

    pub fn wall_spatial(&self) -> Option<&InMemoryWorldSpatial> {
        match self {
            Self::Analysis(w) => Some(&w.wall_spatial),
            Self::Empty(_) => None,
        }
    }

    pub fn results_triggers(&self) -> Option<&HashMap<ParticleId, WorkspaceSignal>> {
        match self {
            Self::Analysis(w) => Some(&w.results_triggers),
            Self::Empty(_) => None,
        }
    }
}
