//! Dual-write container hierarchy into the composed-view graph as `UIView` nodes
//! connected by `Binding` edges (containment + deterministic `"order"` prop).
//!
//! PageTree / PodList remain cached render projections of Binding order.

use hyper_ui::{PageNode, PageTree};
use hypernode::{EdgeId, EdgeKind, Graph, HyperEdge, Node, NodeId, PropValue, SpaceClass};
use std::collections::{BTreeMap, HashMap};

const ORDER_PROP: &str = "order";

/// Insert a `SpaceClass::UIView` node and return its id.
pub fn insert_uiview(graph: &mut Graph, label: impl Into<String>) -> NodeId {
    graph.insert_node(Node {
        id: NodeId(0),
        space_class: SpaceClass::UIView,
        label: label.into(),
        world_pos: [0.0, 0.0],
        props: BTreeMap::new(),
    })
}

/// Insert a containment `Binding` edge with deterministic `"order"` prop.
pub fn insert_binding(graph: &mut Graph, parent: NodeId, child: NodeId, order: i64) -> EdgeId {
    graph.insert_edge(HyperEdge {
        id: EdgeId(0),
        kind: EdgeKind::Binding,
        sources: vec![parent],
        targets: vec![child],
        curvature: 0.0,
        label: None,
        props: BTreeMap::from([(ORDER_PROP.into(), PropValue::I64(order))]),
    })
}

/// Ensure every page and pod has a UIView node, then rewrite Workspace→Page
/// Binding edges to match the tree's ordered page cache.
pub fn dual_write_page_tree(graph: &mut Graph, workspace_node: NodeId, tree: &mut PageTree) {
    for page in &mut tree.pages {
        ensure_page_uiview(graph, page);
    }
    graph
        .edges
        .retain(|_, edge| !(edge.kind == EdgeKind::Binding && edge.sources == [workspace_node]));
    for (order, page) in tree.pages.iter().enumerate() {
        insert_binding(graph, workspace_node, page.node_id, order as i64);
    }
}

fn ensure_page_uiview(graph: &mut Graph, page: &mut PageNode) {
    if !graph.nodes.contains_key(&page.node_id) {
        page.node_id = insert_uiview(graph, page.state.label.clone());
    }
    for pod in &mut page.pods.pods {
        if !graph.nodes.contains_key(&pod.node_id) {
            pod.node_id = insert_uiview(graph, pod.title.clone());
        }
    }
    graph
        .edges
        .retain(|_, edge| !(edge.kind == EdgeKind::Binding && edge.sources == [page.node_id]));
    for (pod_order, pod) in page.pods.pods.iter().enumerate() {
        insert_binding(graph, page.node_id, pod.node_id, pod_order as i64);
    }
}

/// Children of `parent` via Binding edges, sorted by `"order"` prop.
pub fn binding_children(graph: &Graph, parent: NodeId) -> Vec<NodeId> {
    let mut kids: Vec<(i64, NodeId)> = graph
        .edges
        .values()
        .filter(|e| e.kind == EdgeKind::Binding && e.sources.contains(&parent))
        .flat_map(|e| {
            let order = match e.props.get(ORDER_PROP) {
                Some(PropValue::I64(v)) => *v,
                _ => i64::MAX,
            };
            e.targets.iter().map(move |t| (order, *t))
        })
        .collect();
    kids.sort_by_key(|(order, id)| (*order, id.0));
    kids.into_iter().map(|(_, id)| id).collect()
}

/// Reorder the page cache to the composed-view Binding child order.
///
/// Pages without a matching Binding remain at the end, preserving their current
/// relative order so transient edits do not silently drop render state.
pub fn sync_page_order_from_bindings(graph: &Graph, workspace_node: NodeId, tree: &mut PageTree) {
    let positions: HashMap<_, _> = binding_children(graph, workspace_node)
        .into_iter()
        .enumerate()
        .map(|(index, node_id)| (node_id, index))
        .collect();
    tree.pages.sort_by_key(|page| {
        (
            positions.get(&page.node_id).copied().unwrap_or(usize::MAX),
            page.id.0,
        )
    });
}

/// Assert Binding walk matches PageTree leaf order and each page's PodList order.
pub fn assert_binding_parity(graph: &Graph, workspace_node: NodeId, tree: &PageTree) {
    let pages = tree.leaves();
    let bound_pages = binding_children(graph, workspace_node);
    assert_eq!(
        bound_pages.len(),
        pages.len(),
        "page Binding count mismatch for workspace {:?}",
        workspace_node
    );
    for (page, &bound) in pages.iter().zip(bound_pages.iter()) {
        assert_eq!(page.node_id, bound, "page node_id vs Binding target");
        let bound_pods = binding_children(graph, page.node_id);
        assert_eq!(
            bound_pods.len(),
            page.pods.pods.len(),
            "pod Binding count mismatch for page {:?}",
            page.node_id
        );
        for (pod, &bound_pod) in page.pods.pods.iter().zip(bound_pods.iter()) {
            assert_eq!(pod.node_id, bound_pod, "pod node_id vs Binding target");
        }
    }
}

/// Count PageTree leaves + pods (for parity canary vs UIView nodes).
pub fn count_page_tree_containers(tree: &PageTree) -> usize {
    tree.leaves()
        .iter()
        .map(|page| 1 + page.pods.pods.len())
        .sum()
}

/// Entity (domain) nodes only — walls, etc. UIView containers excluded.
pub fn entity_nodes(graph: &Graph) -> impl Iterator<Item = &Node> {
    graph
        .nodes
        .values()
        .filter(|n| n.space_class == SpaceClass::Entity)
}

pub fn entity_count(graph: &Graph) -> usize {
    entity_nodes(graph).count()
}

pub fn first_entity_id(graph: &Graph) -> Option<NodeId> {
    entity_nodes(graph).map(|n| n.id).next()
}

pub fn is_entity(graph: &Graph, id: NodeId) -> bool {
    graph
        .nodes
        .get(&id)
        .is_some_and(|n| n.space_class == SpaceClass::Entity)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workspace::from_seed::page_tree_from_seeds;
    use crate::workspace::seed;
    use hyper_ui::{FocusPath, InputClass, Rect, SizeClass, Vec2, Viewport, Visibility};

    #[test]
    fn binding_parity_across_seeded_workspaces() {
        let mut graph = Graph::new();
        let mut with_tree = 0usize;
        for seed in seed::ALL {
            if seed.pages.is_empty() {
                continue;
            }
            with_tree += 1;
            let mut tree = page_tree_from_seeds(seed.pages);
            let ws_node = insert_uiview(&mut graph, seed.label);
            dual_write_page_tree(&mut graph, ws_node, &mut tree);
            assert_binding_parity(&graph, ws_node, &tree);
        }
        assert!(
            with_tree >= 5,
            "expected ≥5 workspaces with pages, got {with_tree}"
        );
    }

    #[test]
    fn cascade_resolution_matches_binding_order_across_seeded_workspaces() {
        let mut graph = Graph::new();
        for seed in seed::ALL {
            if seed.pages.is_empty() {
                continue;
            }
            let mut tree = page_tree_from_seeds(seed.pages);
            let workspace_node = insert_uiview(&mut graph, seed.label);
            dual_write_page_tree(&mut graph, workspace_node, &mut tree);

            let binding_ids = binding_children(&graph, workspace_node);
            assert_eq!(
                tree.pages.iter().map(|page| page.node_id).collect::<Vec<_>>(),
                binding_ids
            );

            for width in [390.0, 600.0, 834.0, 1440.0] {
                let area = Rect::from_xywh(0.0, 0.0, width, 900.0);
                let viewport = Viewport {
                    size: Vec2::new(width, 900.0),
                    scale_factor: 1.0,
                    size_class: SizeClass::from_width(width),
                    input_class: InputClass::Pointer,
                };
                let focus = FocusPath::new(vec![PageNode::container_id(tree.pages[0].id)]);
                let mut direct = tree.clone();
                let mut binding_ordered = tree.clone();
                sync_page_order_from_bindings(&graph, workspace_node, &mut binding_ordered);

                direct.layout(area, &focus, &hyper_ui::Overrides::new(), &viewport);
                binding_ordered.layout(area, &focus, &hyper_ui::Overrides::new(), &viewport);

                let direct_visibility: Vec<_> = direct
                    .pages
                    .iter()
                    .map(|page| (page.id, page.state.resolved()))
                    .collect();
                let binding_visibility: Vec<_> = binding_ordered
                    .pages
                    .iter()
                    .map(|page| (page.id, page.state.resolved()))
                    .collect();
                assert_eq!(direct_visibility, binding_visibility);
                assert!(
                    direct_visibility
                        .iter()
                        .any(|(_, visibility)| *visibility == Visibility::Shown)
                );
            }
        }
    }
}
