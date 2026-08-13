//! Build a GraphViewWorkspace from the `devtools_graph` seed.

use super::state::GraphViewState;
use super::workspace::GraphViewWorkspace;
use crate::workspace::from_seed::page_tree_from_seeds;
use crate::workspace::graph_containers::{
    dual_write_page_tree, insert_uiview, write_components_from_page_seeds,
};
use crate::workspace::seed::WorkspaceSeed;
use hyper_ui::{InMemoryWorldSpatial, PageId};
use hypernode::Graph;

impl GraphViewWorkspace {
    pub fn from_seed(seed: &WorkspaceSeed, graph: &mut Graph) -> Self {
        let mut page_tree = page_tree_from_seeds(seed.pages);
        let node_id = insert_uiview(graph, seed.label);
        dual_write_page_tree(graph, node_id, &mut page_tree);
        write_components_from_page_seeds(graph, &page_tree, seed.pages);
        let focused_page = page_tree
            .leaves()
            .first()
            .map(|p| p.id)
            .unwrap_or(PageId(0));
        Self {
            page_tree,
            page_overrides: hyper_ui::Overrides::new(),
            focused_page,
            node_id,
            state: GraphViewState::default(),
            spatial: InMemoryWorldSpatial::default(),
            graph_view_sink: None,
            particle_sinks: Default::default(),
            view_last_pos: None,
            view_panning: false,
            dragging: None,
            page_viewport_ids: Default::default(),
            page_show_triggers: Default::default(),
            pod_collapse_triggers: Default::default(),
            icon_rail_triggers: Default::default(),
            filter_triggers: Default::default(),
            physarum: None,
        }
    }
}
