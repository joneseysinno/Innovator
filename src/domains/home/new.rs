use super::workspace::HomeWorkspace;
use crate::workspace::from_seed::page_tree_from_seeds;
use crate::workspace::graph_containers::{
    dual_write_page_tree, insert_uiview, write_components_from_page_seeds,
};
use crate::workspace::seed;
use hyper_ui::PageId;
use hypernode::Graph;
use std::collections::HashMap;

impl HomeWorkspace {
    /// Build from `HOME` seeds, dual-writing UIView containers into `graph`.
    pub fn from_seed(graph: &mut Graph) -> Self {
        let mut page_tree = page_tree_from_seeds(seed::HOME.pages);
        let node_id = insert_uiview(graph, seed::HOME.label);
        dual_write_page_tree(graph, node_id, &mut page_tree);
        write_components_from_page_seeds(graph, &page_tree, seed::HOME.pages);
        let focused_page = page_tree
            .leaves()
            .first()
            .map(|p| p.id)
            .unwrap_or(PageId(0));
        Self {
            page_tree,
            page_overrides: hyper_ui::Overrides::new(),
            focused_page,
            page_viewport_ids: HashMap::new(),
            page_show_triggers: HashMap::new(),
            pod_collapse_triggers: HashMap::new(),
            icon_rail_triggers: HashMap::new(),
            launcher_triggers: HashMap::new(),
            node_id,
        }
    }
}
