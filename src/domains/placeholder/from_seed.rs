//! Build a PlaceholderWorkspace from a WorkspaceSeed.

use super::{PlaceholderWorkspace, StubIoMap};
use crate::workspace::seed::{PageSeed, PodSeed, WorkspaceSeed};
use hyper_ui::{
    default_icon_rail_config, Extent, PageId, PageNode, PageTree, Pod, PodId, PodList,
    SeamDirection,
};

impl PlaceholderWorkspace {
    pub fn from_seed(seed: &WorkspaceSeed) -> Self {
        let (page_tree, stub_ios) = build_page_tree(seed.pages);
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
        }
    }
}

fn build_page_tree(pages: &[PageSeed]) -> (PageTree, StubIoMap) {
    let mut stub_ios = StubIoMap::new();
    if pages.is_empty() {
        return (PageTree::Leaf(PageNode::empty(PageId(0))), stub_ios);
    }

    let mut leaves = Vec::with_capacity(pages.len());
    for (i, page) in pages.iter().enumerate() {
        let page_id = PageId(i as u32);
        let (node, stubs) = build_page(page_id, page);
        stub_ios.extend(stubs);
        leaves.push(node);
    }

    let tree = fold_vertical(leaves);
    (tree, stub_ios)
}

fn build_page(page_id: PageId, seed: &PageSeed) -> (PageNode, StubIoMap) {
    let mut stubs = StubIoMap::new();
    let mut pods = Vec::with_capacity(seed.pods.len());
    for (i, pod_seed) in seed.pods.iter().enumerate() {
        let pod_id = PodId(i as u32);
        pods.push(build_pod(pod_id, pod_seed));
        let labels: Vec<String> = pod_seed.ios.iter().map(|io| io.label.to_string()).collect();
        stubs.insert((page_id, pod_id), labels);
    }
    let has_nav = pods.iter().any(|p| p.nav_icon.is_some());
    let mut node = PageNode::new(page_id, PodList::new(pods))
        .with_label(seed.label, seed.icon)
        .with_extent(seed.extent);
    if has_nav {
        node = node.with_icon_rail(Some(default_icon_rail_config()));
    }
    (node, stubs)
}

fn build_pod(id: PodId, seed: &PodSeed) -> Pod {
    let ideal_ref = 480.0;
    let height = (seed.extent.ideal / ideal_ref).max(0.01);
    let mut pod = Pod::new(id, seed.label)
        .with_min_height(seed.extent.min)
        .with_height(height)
        .with_extent_override(seed.extent);
    if !seed.icon.is_empty() {
        pod = pod.with_nav_icon(seed.icon);
    }
    pod
}

trait PodExtentExt {
    fn with_extent_override(self, extent: Extent) -> Self;
}

impl PodExtentExt for Pod {
    fn with_extent_override(mut self, extent: Extent) -> Self {
        self.state.extent = extent;
        self.min_height = extent.min;
        self.height = extent.weight.max(0.01);
        self
    }
}

fn fold_vertical(mut leaves: Vec<PageNode>) -> PageTree {
    match leaves.len() {
        0 => PageTree::Leaf(PageNode::empty(PageId(0))),
        1 => PageTree::Leaf(leaves.remove(0)),
        _ => {
            let first = leaves.remove(0);
            let rest = fold_vertical(leaves);
            PageTree::Split {
                direction: SeamDirection::Vertical,
                first: Box::new(PageTree::Leaf(first)),
                second: Box::new(rest),
            }
        }
    }
}
