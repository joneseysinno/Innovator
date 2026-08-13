//! App-level workspace — container state + typed body (no trait objects).

use crate::domains::graph_view::GraphViewWorkspace;
use crate::domains::home::HomeWorkspace;
use crate::domains::placeholder::PlaceholderWorkspace;
use crate::domains::structural::StructuralWorkspace;
use crate::workspace::graph_containers::count_page_tree_containers;
use crate::workspace::header::WorkspaceHeader;
use crate::workspace::seed::{self, WorkspaceSeed};
use crate::workspace::tab::WorkspaceTab;
use crate::workspace::workspace_id::WorkspaceId;
use hyper_ui::container::{ContainerId, ContainerState, Extent, Visibility};
use hyper_ui::particles::Particle;
use hyper_ui::{PageTree, ParticleId};
use hypernode::{Graph, SpaceClass};
use infinite_db::InfiniteDb;

/// One workspace row in the app tab strip.
pub struct Workspace {
    pub state: ContainerState,
    pub open_id: &'static str,
    pub body: WorkspaceBody,
    /// Graph UIView identity for this workspace container.
    pub node_id: hypernode::NodeId,
}

pub enum WorkspaceBody {
    Home(HomeWorkspace),
    Structural(StructuralWorkspace),
    GraphView(GraphViewWorkspace),
    Placeholder(PlaceholderWorkspace),
}

impl Workspace {
    /// Build every seed into live workspaces (first run). Seeds are not stored.
    pub fn from_seeds(db: &mut InfiniteDb, graph: &mut Graph) -> Vec<Self> {
        let workspaces: Vec<Self> = seed::ALL
            .iter()
            .enumerate()
            .map(|(i, seed)| Self::from_seed(seed, WorkspaceId(i as u64 + 1), db, graph))
            .collect();
        debug_assert_container_parity(graph, &workspaces);
        workspaces
    }

    pub fn from_seed(
        seed: &WorkspaceSeed,
        id: WorkspaceId,
        db: &mut InfiniteDb,
        graph: &mut Graph,
    ) -> Self {
        let state = container_state(id, seed.label, seed.icon, seed.intent);
        let (body, node_id) = match seed.open_id {
            "home" => {
                let ws = HomeWorkspace::from_seed(graph);
                let node_id = ws.node_id;
                (WorkspaceBody::Home(ws), node_id)
            }
            "structural_analysis" => {
                let ws = StructuralWorkspace::new(db, graph);
                let node_id = ws.node_id;
                (WorkspaceBody::Structural(ws), node_id)
            }
            "devtools_graph" => {
                let ws = GraphViewWorkspace::from_seed(seed, graph);
                let node_id = ws.node_id;
                (WorkspaceBody::GraphView(ws), node_id)
            }
            _ => {
                let ws = PlaceholderWorkspace::from_seed(seed, graph);
                let node_id = ws.node_id;
                (WorkspaceBody::Placeholder(ws), node_id)
            }
        };
        Self {
            state,
            open_id: seed.open_id,
            body,
            node_id,
        }
    }

    pub fn new_structural_titled(
        id: WorkspaceId,
        title: impl Into<String>,
        db: &mut InfiniteDb,
        graph: &mut Graph,
    ) -> Self {
        let mut ws = Self::from_seed(&seed::STRUCTURAL, id, db, graph);
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
            WorkspaceBody::Home(ws) => Some(&ws.page_tree),
            WorkspaceBody::Structural(ws) => Some(&ws.page_tree),
            WorkspaceBody::GraphView(ws) => Some(&ws.page_tree),
            WorkspaceBody::Placeholder(ws) => Some(&ws.page_tree),
        }
    }

    pub fn page_tree_mut(&mut self) -> Option<&mut PageTree> {
        match &mut self.body {
            WorkspaceBody::Home(ws) => Some(&mut ws.page_tree),
            WorkspaceBody::Structural(ws) => Some(&mut ws.page_tree),
            WorkspaceBody::GraphView(ws) => Some(&mut ws.page_tree),
            WorkspaceBody::Placeholder(ws) => Some(&mut ws.page_tree),
        }
    }

    pub fn build_content(&mut self, graph: &Graph) -> Particle {
        match &mut self.body {
            WorkspaceBody::Home(ws) => crate::domains::home::build_content::build_content(ws),
            WorkspaceBody::Structural(ws) => {
                crate::domains::structural::build_pages::build_pages(ws, graph)
            }
            WorkspaceBody::GraphView(ws) => {
                // Prefer `build_graph_workspace_content` (passes app root as scope).
                let active = Some(ws.node_id);
                crate::domains::graph_view::build_content::build_content(ws, graph, active)
            }
            WorkspaceBody::Placeholder(ws) => {
                crate::domains::placeholder::build_content::build_content(ws, graph)
            }
        }
    }

    /// Build content when the active body is GraphView — scope root is the app root.
    pub fn build_graph_workspace_content(
        workspaces: &mut [Workspace],
        active_idx: usize,
        graph: &Graph,
        root_id: hypernode::NodeId,
    ) -> Particle {
        match &mut workspaces[active_idx].body {
            WorkspaceBody::GraphView(ws) => {
                crate::domains::graph_view::build_content::build_content(ws, graph, Some(root_id))
            }
            _ => Particle::Source(hyper_ui::particles::SourceParticle::muted("")),
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

    pub fn graph_view(&self) -> Option<&GraphViewWorkspace> {
        match &self.body {
            WorkspaceBody::GraphView(ws) => Some(ws),
            _ => None,
        }
    }

    pub fn graph_view_mut(&mut self) -> Option<&mut GraphViewWorkspace> {
        match &mut self.body {
            WorkspaceBody::GraphView(ws) => Some(ws),
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
        "G" => "G",
        "○" => "·",
        _ => "·",
    }
}

/// Canary: UIView node count matches root + workspace + page + pod + component containers.
fn debug_assert_container_parity(graph: &Graph, workspaces: &[Workspace]) {
    let uiview = graph
        .nodes
        .values()
        .filter(|n| n.space_class == SpaceClass::UIView)
        .count();
    let roots = graph
        .nodes
        .values()
        .filter(|n| n.space_class == SpaceClass::UIView && n.label == "root")
        .count();
    let mut expected = roots;
    for ws in workspaces {
        expected += 1; // workspace node
        if let Some(tree) = ws.page_tree() {
            expected += count_page_tree_containers(tree);
            // Components + particle slots under each pod.
            for page in &tree.pages {
                for pod in &page.pods.pods {
                    let components = crate::workspace::graph_containers::component_labels(
                        graph,
                        pod.node_id,
                    );
                    expected += components.len();
                    for child in crate::workspace::graph_containers::binding_children(
                        graph,
                        pod.node_id,
                    ) {
                        if crate::workspace::graph_containers::particle_slot(graph, child)
                            .is_some()
                        {
                            expected += 1;
                        }
                    }
                }
            }
        }
    }
    debug_assert_eq!(
        uiview, expected,
        "UIView graph nodes ({uiview}) != container count ({expected})"
    );
}
