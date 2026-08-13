//! Page-local graph-view state — never written to `Node.world_pos`.

use hyper_ui::Vec2;
use hypernode::{EdgeId, EdgeKind, NodeId, SpaceClass};
use std::collections::{HashMap, HashSet};

/// Which subgraph the page draws.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum GraphScope {
    /// Nodes bound under the active (shown) workspace UIView.
    ActiveWorkspace,
    /// BFS from the active workspace node through all edge kinds.
    Reachable,
    /// Everything the composed-view access surface returns.
    #[default]
    Composed,
}

/// Page-local layout / selection / filter state for the graph view.
#[derive(Debug, Clone)]
pub struct GraphViewState {
    /// Force-layout positions (relative embedding, not world space).
    pub positions: HashMap<NodeId, Vec2>,
    /// Synthetic junction points for multi-endpoint hyperedges.
    pub junctions: HashMap<EdgeId, Vec2>,
    /// Pinned nodes — sticky overrides that win over simulated positions.
    pub pinned: HashSet<NodeId>,
    pub selected: Option<NodeId>,
    /// Empty set = show all.
    pub space_classes: HashSet<SpaceClass>,
    /// Empty set = show all.
    pub edge_kinds: HashSet<EdgeKind>,
    pub scope: GraphScope,
    /// Force-layout cooling factor (1 → hot, →0 steady).
    pub alpha: f32,
    /// True after first naive seed placement.
    pub seeded: bool,
}

impl Default for GraphViewState {
    fn default() -> Self {
        Self {
            positions: HashMap::new(),
            junctions: HashMap::new(),
            pinned: HashSet::new(),
            selected: None,
            space_classes: HashSet::new(),
            edge_kinds: HashSet::new(),
            scope: GraphScope::Composed,
            alpha: 1.0,
            seeded: false,
        }
    }
}

impl GraphViewState {
    pub fn shows_space_class(&self, class: SpaceClass) -> bool {
        self.space_classes.is_empty() || self.space_classes.contains(&class)
    }

    pub fn shows_edge_kind(&self, kind: EdgeKind) -> bool {
        self.edge_kinds.is_empty() || self.edge_kinds.contains(&kind)
    }

    pub fn toggle_space_class(&mut self, class: SpaceClass) {
        if self.space_classes.is_empty() {
            self.space_classes = [SpaceClass::UIView, SpaceClass::Entity, SpaceClass::Function, SpaceClass::Carrier]
                .into_iter()
                .collect();
        }
        if !self.space_classes.remove(&class) {
            self.space_classes.insert(class);
        }
    }

    pub fn toggle_edge_kind(&mut self, kind: EdgeKind) {
        if self.edge_kinds.is_empty() {
            self.edge_kinds = [EdgeKind::Binding, EdgeKind::Signal, EdgeKind::Stream, EdgeKind::Wave]
                .into_iter()
                .collect();
        }
        if !self.edge_kinds.remove(&kind) {
            self.edge_kinds.insert(kind);
        }
    }
}
