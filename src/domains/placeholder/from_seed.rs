//! Build a PlaceholderWorkspace from a WorkspaceSeed.

use super::{PlaceholderWorkspace, StubIoMap};
use crate::workspace::from_seed::page_tree_from_seeds;
use crate::workspace::graph_containers::{dual_write_page_tree, insert_uiview};
use crate::workspace::seed::{PageSeed, WorkspaceSeed};
use hyper_ui::{PageId, PodId};
use hypernode::Graph;

impl PlaceholderWorkspace {
    pub fn from_seed(seed: &WorkspaceSeed, graph: &mut Graph) -> Self {
        let mut page_tree = page_tree_from_seeds(seed.pages);
        let node_id = insert_uiview(graph, seed.label);
        dual_write_page_tree(graph, node_id, &mut page_tree);
        let stub_ios = stub_ios_from_pages(seed.pages);
        let focused_page = page_tree
            .leaves()
            .first()
            .map(|p| p.id)
            .unwrap_or(PageId(0));
        Self {
            open_id: seed.open_id,
            page_tree,
            stub_ios,
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

fn stub_ios_from_pages(pages: &[PageSeed]) -> StubIoMap {
    let mut stub_ios = StubIoMap::new();
    for (i, page) in pages.iter().enumerate() {
        let page_id = PageId(i as u32);
        for (j, pod) in page.pods.iter().enumerate() {
            let pod_id = PodId(j as u32);
            let labels: Vec<String> = pod.ios.iter().map(|io| io.label.to_string()).collect();
            stub_ios.insert((page_id, pod_id), labels);
        }
    }
    stub_ios
}
