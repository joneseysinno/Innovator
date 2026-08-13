//! Graph-view workspace — page tree + page-local layout state + Layer A spatial.

use super::state::GraphViewState;
use hyper_ui::{
    FocusPath, InMemoryWorldSpatial, Overrides, PageId, PageNode, PageTree, ParticleId, PodId,
    ResolveReport, Vec2, Viewport,
};
use hypernode::NodeId;
use std::collections::HashMap;

pub struct GraphViewWorkspace {
    pub page_tree: PageTree,
    pub page_overrides: Overrides,
    pub focused_page: PageId,
    /// Graph UIView identity for this workspace container.
    pub node_id: NodeId,
    pub state: GraphViewState,
    /// Layer A spatial snapshot rebuilt each content build / physics tick.
    pub spatial: InMemoryWorldSpatial,
    /// Canvas sink — pan / zoom / select / pin.
    pub graph_view_sink: Option<ParticleId>,
    /// Hit-test sinks for individual particles (ParticleId → NodeId).
    pub particle_sinks: HashMap<ParticleId, NodeId>,
    pub view_last_pos: Option<Vec2>,
    pub view_panning: bool,
    /// Dragging a pinned/selected node.
    pub dragging: Option<NodeId>,
    pub page_viewport_ids: HashMap<PageId, ParticleId>,
    pub page_show_triggers: HashMap<ParticleId, PageId>,
    pub pod_collapse_triggers: HashMap<ParticleId, PodId>,
    pub icon_rail_triggers: HashMap<ParticleId, (PageId, PodId)>,
    /// Filter / scope chip triggers.
    pub filter_triggers: HashMap<ParticleId, GraphFilterAction>,
    /// Physarum network keyed by NodeId.0 (Phase 5).
    pub physarum: Option<physarum::PhysarumNetwork<u64>>,
}

/// UI actions for filter chips and scope selector.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GraphFilterAction {
    Scope(super::state::GraphScope),
    ToggleSpace(hypernode::SpaceClass),
    ToggleEdge(hypernode::EdgeKind),
}

impl GraphViewWorkspace {
    pub fn focus_path(&self) -> FocusPath {
        FocusPath::new(vec![PageNode::container_id(self.focused_page)])
    }

    pub fn layout_pages(
        &mut self,
        pages_area: hyper_ui::Rect,
        app_focus: &FocusPath,
        viewport: &Viewport,
    ) -> (Vec<(PageId, hyper_ui::Rect)>, ResolveReport) {
        if let Some(page_id) = page_id_on_focus(app_focus, &self.page_tree) {
            self.focused_page = page_id;
        }
        let focus = self.focus_path();
        let (rects, report) =
            self.page_tree
                .layout(pages_area, &focus, &self.page_overrides, viewport);
        if let Some((id, _)) = rects.first() {
            if self
                .page_tree
                .find(self.focused_page)
                .map(|p| p.state.resolved() != hyper_ui::Visibility::Shown)
                .unwrap_or(true)
            {
                self.focused_page = *id;
            }
        }
        (rects, report)
    }

    /// Resolve the graph-canvas pod rect (first pod on the first page).
    pub fn canvas_rect(&self, pages_area: hyper_ui::Rect) -> Option<hyper_ui::Rect> {
        let (page_id, page_rect) = self.page_tree.leaf_rects(pages_area).into_iter().next()?;
        let page = self.page_tree.find(page_id)?;
        let content = page.content_rect(page_rect);
        let leaves = page.pods.layout_rects(content);
        leaves.into_iter().next().map(|(_, r)| r)
    }
}

fn page_id_on_focus(focus: &FocusPath, tree: &PageTree) -> Option<PageId> {
    for id in &focus.chain {
        for leaf in tree.leaves() {
            if PageNode::container_id(leaf.id) == *id {
                return Some(leaf.id);
            }
        }
    }
    None
}
