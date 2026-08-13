//! Build a PlaceholderWorkspace from a WorkspaceSeed.

use super::PlaceholderWorkspace;
use crate::workspace::from_seed::page_tree_from_seeds;
use crate::workspace::graph_containers::{
    dual_write_page_tree, insert_uiview, write_components_from_page_seeds,
};
use crate::workspace::seed::WorkspaceSeed;
use hyper_ui::PageId;
use hypernode::Graph;

impl PlaceholderWorkspace {
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
            open_id: seed.open_id,
            page_tree,
            page_overrides: hyper_ui::Overrides::new(),
            focused_page,
            page_viewport_ids: Default::default(),
            page_show_triggers: Default::default(),
            pod_collapse_triggers: Default::default(),
            icon_rail_triggers: Default::default(),
            node_id,
        }
    }
}
