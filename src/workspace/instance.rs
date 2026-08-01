use crate::workspace::analysis::AnalysisWorkspace;
use crate::workspace::analysis_action::AnalysisAction;
use crate::workspace::app_signal::AppSignal;
use crate::workspace::empty::EmptyWorkspace;
use crate::workspace::field_builder_draft::BuilderFieldSlot;
use crate::workspace::header::WorkspaceHeader;
use crate::workspace::home::HomeWorkspace;
use crate::workspace::kind::WorkspaceKind;
use crate::workspace::pm::PmWorkspace;
use crate::workspace::signal::WorkspaceSignal;
use crate::workspace::tab::WorkspaceTab;
use crate::workspace::workspace_id::WorkspaceId;
use hyper_ui::{InMemoryWorldSpatial, ParticleId, PodTree};
use hypernode::NodeId;
use std::collections::HashMap;

/// Hosted workspace instance inside the app shell.
pub enum WorkspaceInstance {
    Analysis(AnalysisWorkspace),
    Home(HomeWorkspace),
    Pm(PmWorkspace),
    Empty(EmptyWorkspace),
}

impl WorkspaceInstance {
    pub fn tab(&self) -> &WorkspaceTab {
        match self {
            Self::Analysis(w) => &w.tab,
            Self::Home(w) => &w.tab,
            Self::Pm(w) => &w.tab,
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
            Self::Pm(w) => w.header.as_ref(),
            Self::Home(_) | Self::Empty(_) => None,
        }
    }

    pub fn pod_tree(&self) -> Option<&PodTree> {
        match self {
            Self::Analysis(w) => Some(&w.pod_tree),
            Self::Home(_) | Self::Pm(_) | Self::Empty(_) => None,
        }
    }

    pub fn pod_tree_mut(&mut self) -> Option<&mut PodTree> {
        match self {
            Self::Analysis(w) => Some(&mut w.pod_tree),
            Self::Home(_) | Self::Pm(_) | Self::Empty(_) => None,
        }
    }

    pub fn status_id(&self) -> Option<hyper_ui::ParticleId> {
        match self {
            Self::Analysis(w) => w.status_id(),
            Self::Pm(w) => w.status_id(),
            Self::Home(_) | Self::Empty(_) => None,
        }
    }

    pub fn home_actions(&self) -> Option<&HashMap<ParticleId, AppSignal>> {
        match self {
            Self::Home(w) => Some(&w.actions),
            _ => None,
        }
    }

    pub fn wall_sinks(&self) -> Option<&HashMap<ParticleId, NodeId>> {
        match self {
            Self::Analysis(w) => Some(&w.wall_sinks),
            _ => None,
        }
    }

    pub fn nav_triggers(&self) -> Option<&HashMap<ParticleId, WorkspaceSignal>> {
        match self {
            Self::Analysis(w) => Some(&w.nav_triggers),
            _ => None,
        }
    }

    pub fn field_props(&self) -> Option<&HashMap<ParticleId, String>> {
        match self {
            Self::Analysis(w) => Some(&w.field_props),
            _ => None,
        }
    }

    pub fn analysis_actions(&self) -> Option<&HashMap<ParticleId, AnalysisAction>> {
        match self {
            Self::Analysis(w) => Some(&w.analysis_actions),
            _ => None,
        }
    }

    pub fn builder_slots(&self) -> Option<&HashMap<ParticleId, BuilderFieldSlot>> {
        match self {
            Self::Analysis(w) => Some(&w.builder_slots),
            _ => None,
        }
    }

    pub fn promote_props(&self) -> Option<&HashMap<ParticleId, String>> {
        match self {
            Self::Analysis(w) => Some(&w.promote_props),
            _ => None,
        }
    }

    pub fn wall_view_sink(&self) -> Option<ParticleId> {
        match self {
            Self::Analysis(w) => w.wall_view_sink,
            _ => None,
        }
    }

    pub fn wall_spatial(&self) -> Option<&InMemoryWorldSpatial> {
        match self {
            Self::Analysis(w) => Some(&w.wall_spatial),
            _ => None,
        }
    }

    pub fn results_triggers(&self) -> Option<&HashMap<ParticleId, WorkspaceSignal>> {
        match self {
            Self::Analysis(w) => Some(&w.results_triggers),
            _ => None,
        }
    }
}
