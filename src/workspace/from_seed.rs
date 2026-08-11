//! Single front door: PageSeed / PodSeed → live PageNode / PageTree.

use crate::workspace::seed::{PageSeed, PodSeed};
use hyper_ui::{
    default_icon_rail_config, PageHeaderConfig, PageHeaderSlots, PageId, PageNode, PageTree, Pod,
    PodId, PodList, SeamDirection,
};

/// Build a vertical [`PageTree`] from authored page seeds.
pub fn page_tree_from_seeds(pages: &[PageSeed]) -> PageTree {
    if pages.is_empty() {
        return PageTree::Leaf(PageNode::empty(PageId(0)));
    }
    let leaves: Vec<PageNode> = pages
        .iter()
        .enumerate()
        .map(|(i, seed)| page_node_from_seed(PageId(i as u32), seed))
        .collect();
    fold_vertical(leaves)
}

/// Convert one [`PageSeed`] into a live [`PageNode`] (pods + metadata, no IO bodies).
pub fn page_node_from_seed(page_id: PageId, seed: &PageSeed) -> PageNode {
    let pods: Vec<Pod> = seed
        .pods
        .iter()
        .enumerate()
        .map(|(i, pod_seed)| pod_from_seed(PodId(i as u32), pod_seed))
        .collect();
    let has_nav = pods.iter().any(|p| p.nav_icon.is_some());
    let mut node = PageNode::new(page_id, PodList::new(pods))
        .with_label(seed.label, seed.icon)
        .with_extent(seed.extent);
    if has_nav {
        node = node.with_icon_rail(Some(default_icon_rail_config()));
    }
    if seed.custom_header {
        node = node.with_header(Some(PageHeaderConfig {
            height: 44.0,
            slots: PageHeaderSlots::Custom,
        }));
    }
    node
}

/// Convert one [`PodSeed`] into a live [`Pod`].
pub fn pod_from_seed(id: PodId, seed: &PodSeed) -> Pod {
    let mut pod = Pod::new(id, seed.label)
        .with_min_height(seed.extent.min)
        .with_height(seed.extent.weight.max(0.01));
    pod.state.extent = seed.extent;
    pod.min_height = seed.extent.min;
    pod.height = seed.extent.weight.max(0.01);
    if !seed.icon.is_empty() {
        pod = pod.with_nav_icon(seed.icon);
    }
    pod
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
