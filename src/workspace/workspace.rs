//! App-level workspace — container state + typed body (no trait objects).

use crate::domains::home::HomeWorkspace;
use crate::domains::placeholder::PlaceholderWorkspace;
use crate::domains::structural::StructuralWorkspace;
use crate::workspace::header::WorkspaceHeader;
use crate::workspace::seed::{self, WorkspaceSeed};
use crate::workspace::tab::WorkspaceTab;
use crate::workspace::workspace_id::WorkspaceId;
use hyper_ui::container::{ContainerId, ContainerState, Extent, Visibility};
use hyper_ui::particles::Particle;
use hyper_ui::{PageTree, ParticleId};
use infinite_db::InfiniteDb;

/// One workspace row in the app tab strip.
pub struct Workspace {
    pub state: ContainerState,
    pub open_id: &'static str,
    pub body: WorkspaceBody,
}

pub enum WorkspaceBody {
    Home(HomeWorkspace),
    Structural(StructuralWorkspace),
    Placeholder(PlaceholderWorkspace),
}

impl Workspace {
    /// Build every seed into live workspaces (first run). Seeds are not stored.
    pub fn from_seeds(db: &mut InfiniteDb) -> Vec<Self> {
        seed::ALL
            .iter()
            .enumerate()
            .map(|(i, seed)| Self::from_seed(seed, WorkspaceId(i as u64 + 1), db))
            .collect()
    }

    pub fn from_seed(seed: &WorkspaceSeed, id: WorkspaceId, db: &mut InfiniteDb) -> Self {
        let state = container_state(id, seed.label, seed.icon, seed.intent);
        let body = match seed.open_id {
            "home" => WorkspaceBody::Home(HomeWorkspace::new()),
            "structural_analysis" => WorkspaceBody::Structural(StructuralWorkspace::new(db)),
            _ => WorkspaceBody::Placeholder(PlaceholderWorkspace::from_seed(seed)),
        };
        Self {
            state,
            open_id: seed.open_id,
            body,
        }
    }

    pub fn new_structural_titled(
        id: WorkspaceId,
        title: impl Into<String>,
        db: &mut InfiniteDb,
    ) -> Self {
        let mut ws = Self::from_seed(&seed::STRUCTURAL, id, db);
        ws.state.label = title.into();
        ws.state.intent = Visibility::Hidden;
        ws
    }

    pub fn id(&self) -> WorkspaceId {
        WorkspaceId(self.state.id.0)
    }

    pub fn open_id(&self) -> &'static str {
        self.open_id
    }

    pub fn is_active(&self) -> bool {
        self.state.intent == Visibility::Shown
    }

    pub fn tab(&self) -> WorkspaceTab {
        WorkspaceTab {
            id: self.id(),
            title: self.state.label.clone(),
            icon: static_icon(&self.state.icon),
        }
    }

    pub fn header(&self) -> Option<&WorkspaceHeader> {
        match &self.body {
            WorkspaceBody::Structural(ws) => ws.header.as_ref(),
            _ => None,
        }
    }

    pub fn status_id(&self) -> Option<ParticleId> {
        match &self.body {
            WorkspaceBody::Structural(ws) => ws.status_id(),
            _ => None,
        }
    }

    pub fn page_tree(&self) -> Option<&PageTree> {
        match &self.body {
            WorkspaceBody::Structural(ws) => Some(&ws.page_tree),
            WorkspaceBody::Placeholder(ws) => Some(&ws.page_tree),
            _ => None,
        }
    }

    pub fn page_tree_mut(&mut self) -> Option<&mut PageTree> {
        match &mut self.body {
            WorkspaceBody::Structural(ws) => Some(&mut ws.page_tree),
            WorkspaceBody::Placeholder(ws) => Some(&mut ws.page_tree),
            _ => None,
        }
    }

    pub fn build_content(&mut self) -> Particle {
        match &mut self.body {
            WorkspaceBody::Home(ws) => crate::domains::home::build_content::build_content(ws),
            WorkspaceBody::Structural(ws) => {
                crate::domains::structural::build_pages::build_pages(ws)
            }
            WorkspaceBody::Placeholder(ws) => {
                crate::domains::placeholder::build_content::build_content(ws)
            }
        }
    }

    pub fn structural(&self) -> Option<&StructuralWorkspace> {
        match &self.body {
            WorkspaceBody::Structural(ws) => Some(ws),
            _ => None,
        }
    }

    pub fn structural_mut(&mut self) -> Option<&mut StructuralWorkspace> {
        match &mut self.body {
            WorkspaceBody::Structural(ws) => Some(ws),
            _ => None,
        }
    }

    pub fn placeholder(&self) -> Option<&PlaceholderWorkspace> {
        match &self.body {
            WorkspaceBody::Placeholder(ws) => Some(ws),
            _ => None,
        }
    }

    pub fn placeholder_mut(&mut self) -> Option<&mut PlaceholderWorkspace> {
        match &mut self.body {
            WorkspaceBody::Placeholder(ws) => Some(ws),
            _ => None,
        }
    }

    pub fn home_ws(&self) -> Option<&HomeWorkspace> {
        match &self.body {
            WorkspaceBody::Home(ws) => Some(ws),
            _ => None,
        }
    }

    pub fn home_mut(&mut self) -> Option<&mut HomeWorkspace> {
        match &mut self.body {
            WorkspaceBody::Home(ws) => Some(ws),
            _ => None,
        }
    }

    pub fn show(&mut self) {
        self.state.intent = Visibility::Shown;
    }

    pub fn hide(&mut self) {
        self.state.intent = Visibility::Hidden;
    }
}

fn container_state(
    id: WorkspaceId,
    label: impl Into<String>,
    icon: impl Into<String>,
    intent: Visibility,
) -> ContainerState {
    ContainerState::new(
        ContainerId(id.0),
        label,
        icon,
        intent,
        Extent::preferred(320.0, 1280.0),
    )
}

fn static_icon(icon: &str) -> &'static str {
    match icon {
        "H" | "⌂" => "H",
        "S" | "⬡" => "S",
        "P" | "▦" => "P",
        "E" => "E",
        "D" => "D",
        "C" => "C",
        "R" => "R",
        "A" => "A",
        "○" => "·",
        _ => "·",
    }
}
