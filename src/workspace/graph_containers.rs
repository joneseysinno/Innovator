//! Dual-write container hierarchy into the composed-view graph as `UIView` nodes
//! connected by `Binding` edges (containment + deterministic `"order"` prop).
//!
//! PageTree / PodList remain cached render projections of Binding order.

use hyper_ui::{PageNode, PageTree};
use hypernode::{EdgeId, EdgeKind, Graph, HyperEdge, Node, NodeId, PropValue, SpaceClass};
use std::collections::{BTreeMap, HashMap, HashSet, VecDeque};

const ORDER_PROP: &str = "order";

/// Depth-from-root cache (`PropValue::I64`). Derived by Binding BFS, never a closed enum.
pub const TIER_PROP: &str = "tier";

/// Per-instance emit declaration. Multi-valued encoding: `PropValue::Text` with
/// tokens joined by [`VALENCE_SEP`] (unit separator). No list variant on `PropValue`.
pub const VALENCE_EMIT_PROP: &str = "valence_emit";

/// Per-instance absorb declaration. Same encoding as [`VALENCE_EMIT_PROP`].
pub const VALENCE_ABSORB_PROP: &str = "valence_absorb";

/// Delimiter for multi-valued valence tokens inside a single `PropValue::Text`.
pub const VALENCE_SEP: char = '\u{1f}';

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

/// App-level root — ordinary UIView at Binding depth 0; parent of every workspace.
pub fn insert_root(graph: &mut Graph) -> NodeId {
    insert_uiview(graph, "root")
}

/// Rewrite Root→Workspace Binding edges to match `workspace_nodes` order.
pub fn bind_workspaces_to_root(graph: &mut Graph, root: NodeId, workspace_nodes: &[NodeId]) {
    graph
        .edges
        .retain(|_, edge| !(edge.kind == EdgeKind::Binding && edge.sources == [root]));
    for (order, &ws) in workspace_nodes.iter().enumerate() {
        insert_binding(graph, root, ws, order as i64);
    }
}

/// Append one workspace Binding under root (runtime `add_workspace`).
pub fn bind_workspace_to_root(graph: &mut Graph, root: NodeId, workspace: NodeId) {
    let order = binding_children(graph, root).len() as i64;
    insert_binding(graph, root, workspace, order);
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

/// Replace Pod→Component Binding children with one UIView per label (IoSeed tier).
pub fn write_pod_components(graph: &mut Graph, pod: NodeId, labels: &[&str]) {
    // Drop prior component children (containment Bindings from this pod that
    // target UIViews). Role-tagged Bindings (e.g. active_wall) stay.
    let drop_targets: HashSet<NodeId> = binding_children(graph, pod)
        .into_iter()
        .filter(|&id| {
            graph
                .nodes
                .get(&id)
                .is_some_and(|n| n.space_class == SpaceClass::UIView)
        })
        .collect();
    // Also drop particle-slot grandchildren under those components.
    let mut drop_all = drop_targets.clone();
    for &comp in &drop_targets {
        for child in binding_children(graph, comp) {
            drop_all.insert(child);
        }
    }
    graph.edges.retain(|_, edge| {
        !(edge.kind == EdgeKind::Binding
            && (edge.sources == [pod]
                || edge.sources.iter().any(|s| drop_targets.contains(s)))
            && edge.targets.iter().any(|t| drop_all.contains(t))
            && edge.props.get(ORDER_PROP).is_some()
            && !edge.props.contains_key("role"))
    });
    for id in &drop_all {
        let still_bound = graph.edges.values().any(|e| e.targets.contains(id));
        if !still_bound {
            graph.nodes.remove(id);
        }
    }
    for (order, label) in labels.iter().enumerate() {
        let component = insert_uiview(graph, (*label).to_string());
        insert_binding(graph, pod, component, order as i64);
        // Phase 4a: stable particle-slot identity under the component (depth 5).
        let particle = insert_uiview(graph, format!("particle:{label}"));
        insert_binding(graph, component, particle, 0);
        // Phase 4b: default Source-like valence (emit text, absorb nothing).
        if let Some(node) = graph.nodes.get_mut(&particle) {
            if let Some(v) = encode_valence(&["text"]) {
                node.props.insert(VALENCE_EMIT_PROP.into(), v);
            }
        }
    }
}

/// Dual-write IoSeed labels under each pod for a seeded page tree (order-aligned).
pub fn write_components_from_page_seeds(
    graph: &mut Graph,
    tree: &PageTree,
    pages: &[crate::workspace::seed::PageSeed],
) {
    for (page, seed_page) in tree.pages.iter().zip(pages.iter()) {
        for (pod, seed_pod) in page.pods.pods.iter().zip(seed_page.pods.iter()) {
            let labels: Vec<&str> = seed_pod.ios.iter().map(|io| io.label).collect();
            write_pod_components(graph, pod.node_id, &labels);
        }
    }
}

/// Labels of component UIViews bound under a pod, in Binding order.
///
/// Skips particle-slot children (labels prefixed `particle:`).
pub fn component_labels(graph: &Graph, pod: NodeId) -> Vec<String> {
    binding_children(graph, pod)
        .into_iter()
        .filter_map(|id| {
            graph.nodes.get(&id).and_then(|n| {
                (n.space_class == SpaceClass::UIView && !n.label.starts_with("particle:"))
                    .then(|| n.label.clone())
            })
        })
        .collect()
}

/// Particle-slot UIView bound under a component, if present.
pub fn particle_slot(graph: &Graph, component: NodeId) -> Option<NodeId> {
    binding_children(graph, component).into_iter().find(|&id| {
        graph
            .nodes
            .get(&id)
            .is_some_and(|n| n.space_class == SpaceClass::UIView && n.label.starts_with("particle:"))
    })
}

/// Count particle-slot nodes under every component of a page tree.
pub fn count_particle_slots(graph: &Graph, tree: &PageTree) -> usize {
    tree.pages
        .iter()
        .flat_map(|page| page.pods.pods.iter())
        .map(|pod| {
            binding_children(graph, pod.node_id)
                .into_iter()
                .filter(|&c| particle_slot(graph, c).is_some())
                .count()
        })
        .sum()
}

/// Assert Workspace→Page→Pod→Component Binding parity against page seeds.
pub fn assert_component_parity(
    graph: &Graph,
    workspace_node: NodeId,
    tree: &PageTree,
    pages: &[crate::workspace::seed::PageSeed],
) {
    assert_binding_parity(graph, workspace_node, tree);
    for (page, seed_page) in tree.pages.iter().zip(pages.iter()) {
        for (pod, seed_pod) in page.pods.pods.iter().zip(seed_page.pods.iter()) {
            let labels = component_labels(graph, pod.node_id);
            let expected: Vec<String> = seed_pod.ios.iter().map(|io| io.label.to_string()).collect();
            assert_eq!(
                labels, expected,
                "component labels mismatch for pod {:?} ({})",
                pod.node_id, pod.title
            );
        }
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

/// BFS over `Binding` edges from `root`, writing depth as [`TIER_PROP`].
pub fn derive_tiers(graph: &mut Graph, root: NodeId) {
    let mut queue = VecDeque::from([(root, 0i64)]);
    let mut seen = HashSet::from([root]);
    while let Some((node, depth)) = queue.pop_front() {
        let children = binding_children(graph, node);
        if let Some(n) = graph.nodes.get_mut(&node) {
            n.props
                .insert(TIER_PROP.into(), PropValue::I64(depth));
        }
        for child in children {
            if seen.insert(child) {
                queue.push_back((child, depth + 1));
            }
        }
    }
}

/// Re-walk Binding depth from `root` and assert every cached [`TIER_PROP`] matches.
pub fn assert_tier_consistency(graph: &Graph, root: NodeId) {
    let mut queue = VecDeque::from([(root, 0i64)]);
    let mut seen = HashSet::from([root]);
    while let Some((node, depth)) = queue.pop_front() {
        let cached = graph.nodes.get(&node).and_then(|n| n.props.get(TIER_PROP));
        match cached {
            Some(PropValue::I64(v)) => {
                assert_eq!(
                    *v, depth,
                    "tier mismatch at {:?}: cached {v}, walked {depth}",
                    node
                );
            }
            other => panic!(
                "missing or non-I64 {} at {:?}: {:?}",
                TIER_PROP, node, other
            ),
        }
        for child in binding_children(graph, node) {
            if seen.insert(child) {
                queue.push_back((child, depth + 1));
            }
        }
    }
}

/// Encode valence tokens into a single `PropValue::Text` (or omit when empty).
pub fn encode_valence(tokens: &[&str]) -> Option<PropValue> {
    if tokens.is_empty() {
        None
    } else {
        Some(PropValue::Text(tokens.join(&VALENCE_SEP.to_string())))
    }
}

/// Wire Home→workspace navigation as `Signal` edges (label = open_id).
pub fn wire_home_nav_signals(
    graph: &mut Graph,
    home: NodeId,
    targets: &[(NodeId, &'static str)],
) {
    graph.edges.retain(|_, edge| {
        !(edge.kind == EdgeKind::Signal
            && edge.sources == [home]
            && edge.label.as_deref().is_some_and(|l| {
                targets.iter().any(|(_, open_id)| *open_id == l)
            }))
    });
    for &(target, open_id) in targets {
        graph.insert_edge(HyperEdge {
            id: EdgeId(0),
            kind: EdgeKind::Signal,
            sources: vec![home],
            targets: vec![target],
            curvature: 0.0,
            label: Some(open_id.into()),
            props: BTreeMap::new(),
        });
    }
}

/// Resolve a Home OpenWorkspace intent by walking outbound `Signal` edges.
pub fn resolve_home_nav_target(
    graph: &Graph,
    home: NodeId,
    open_id: &str,
) -> Option<NodeId> {
    graph.edges.values().find_map(|edge| {
        (edge.kind == EdgeKind::Signal
            && edge.sources == [home]
            && edge.label.as_deref() == Some(open_id))
        .then(|| edge.targets.first().copied())
        .flatten()
    })
}

/// Decode valence tokens from a prop value.
pub fn decode_valence(value: Option<&PropValue>) -> Vec<String> {
    match value {
        Some(PropValue::Text(s)) if !s.is_empty() => {
            s.split(VALENCE_SEP).map(str::to_owned).collect()
        }
        _ => Vec::new(),
    }
}

/// Counts emitters / absorbers / unmatched valence tokens under a space root.
///
/// Reports only — never rejects unbalanced wiring.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ValenceReport {
    pub emitters: usize,
    pub absorbers: usize,
    /// Tokens declared as emit with no matching absorb declaration in the space.
    pub unmatched_emits: Vec<String>,
    /// Tokens declared as absorb with no matching emit declaration in the space.
    pub unmatched_absorbs: Vec<String>,
}

pub fn valence_report(graph: &Graph, space_root: NodeId) -> ValenceReport {
    let mut reachable = HashSet::from([space_root]);
    let mut queue = VecDeque::from([space_root]);
    while let Some(n) = queue.pop_front() {
        for child in binding_children(graph, n) {
            if reachable.insert(child) {
                queue.push_back(child);
            }
        }
    }

    let mut emit_counts: HashMap<String, usize> = HashMap::new();
    let mut absorb_counts: HashMap<String, usize> = HashMap::new();
    let mut emitters = 0usize;
    let mut absorbers = 0usize;

    for id in &reachable {
        let Some(node) = graph.nodes.get(id) else {
            continue;
        };
        let emits = decode_valence(node.props.get(VALENCE_EMIT_PROP));
        let absorbs = decode_valence(node.props.get(VALENCE_ABSORB_PROP));
        if !emits.is_empty() {
            emitters += 1;
            for t in emits {
                *emit_counts.entry(t).or_default() += 1;
            }
        }
        if !absorbs.is_empty() {
            absorbers += 1;
            for t in absorbs {
                *absorb_counts.entry(t).or_default() += 1;
            }
        }
    }

    let mut unmatched_emits = Vec::new();
    for (token, count) in &emit_counts {
        let absorbed = absorb_counts.get(token).copied().unwrap_or(0);
        if *count > absorbed {
            unmatched_emits.push(token.clone());
        }
    }
    unmatched_emits.sort();

    let mut unmatched_absorbs = Vec::new();
    for (token, count) in &absorb_counts {
        let emitted = emit_counts.get(token).copied().unwrap_or(0);
        if *count > emitted {
            unmatched_absorbs.push(token.clone());
        }
    }
    unmatched_absorbs.sort();

    ValenceReport {
        emitters,
        absorbers,
        unmatched_emits,
        unmatched_absorbs,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workspace::from_seed::page_tree_from_seeds;
    use crate::workspace::seed;
    use hyper_ui::{FocusPath, InputClass, Rect, SizeClass, Vec2, Viewport, Visibility};

    #[test]
    fn derive_tiers_three_level_binding_tree() {
        let mut graph = Graph::new();
        let root = insert_uiview(&mut graph, "root");
        let mid = insert_uiview(&mut graph, "mid");
        let leaf = insert_uiview(&mut graph, "leaf");
        insert_binding(&mut graph, root, mid, 0);
        insert_binding(&mut graph, mid, leaf, 0);

        derive_tiers(&mut graph, root);
        assert_tier_consistency(&graph, root);

        assert_eq!(
            graph.nodes[&root].props.get(TIER_PROP),
            Some(&PropValue::I64(0))
        );
        assert_eq!(
            graph.nodes[&mid].props.get(TIER_PROP),
            Some(&PropValue::I64(1))
        );
        assert_eq!(
            graph.nodes[&leaf].props.get(TIER_PROP),
            Some(&PropValue::I64(2))
        );
    }

    #[test]
    fn valence_roundtrip_delimiter_encoding() {
        let encoded = encode_valence(&["pointer", "text"]).unwrap();
        assert_eq!(
            decode_valence(Some(&encoded)),
            vec!["pointer".to_string(), "text".to_string()]
        );
        assert!(encode_valence(&[]).is_none());
        assert!(decode_valence(None).is_empty());
    }

    #[test]
    fn valence_report_detects_imbalance() {
        let mut graph = Graph::new();
        let root = insert_uiview(&mut graph, "comp");
        let emitter = insert_uiview(&mut graph, "particle:src");
        let absorber = insert_uiview(&mut graph, "particle:sink");
        insert_binding(&mut graph, root, emitter, 0);
        insert_binding(&mut graph, root, absorber, 1);
        if let Some(n) = graph.nodes.get_mut(&emitter) {
            n.props.insert(
                VALENCE_EMIT_PROP.into(),
                encode_valence(&["text"]).unwrap(),
            );
        }
        if let Some(n) = graph.nodes.get_mut(&absorber) {
            n.props.insert(
                VALENCE_ABSORB_PROP.into(),
                encode_valence(&["pointer"]).unwrap(),
            );
        }

        let report = valence_report(&graph, root);
        assert_eq!(report.emitters, 1);
        assert_eq!(report.absorbers, 1);
        assert_eq!(report.unmatched_emits, vec!["text".to_string()]);
        assert_eq!(report.unmatched_absorbs, vec!["pointer".to_string()]);

        // Balance text by giving the absorber a text absorb too.
        if let Some(n) = graph.nodes.get_mut(&absorber) {
            n.props.insert(
                VALENCE_ABSORB_PROP.into(),
                encode_valence(&["text"]).unwrap(),
            );
        }
        let balanced = valence_report(&graph, root);
        assert!(balanced.unmatched_emits.is_empty());
        assert!(balanced.unmatched_absorbs.is_empty());
    }

    #[test]
    fn binding_parity_across_seeded_workspaces() {
        let dir = std::env::temp_dir().join(format!(
            "innovator_parity_{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("temp dir");
        let mut db = infinite_db::InfiniteDb::open(&dir).expect("db");
        crate::walls::ensure_walls_space(&mut db);
        crate::results::ensure_results_space(&mut db);
        let mut graph = Graph::new();
        let workspaces = crate::workspace::Workspace::from_seeds(&mut db, &mut graph);
        let _ = std::fs::remove_dir_all(&dir);

        let with_tree = workspaces
            .iter()
            .filter(|ws| ws.page_tree().is_some_and(|t| !t.pages.is_empty()))
            .count();
        // All nine seeds (including Home) dual-write a page tree.
        assert_eq!(with_tree, seed::ALL.len());

        for ws in &workspaces {
            let tree = ws.page_tree().expect("every seeded workspace has a page tree");
            let seed = seed::ALL
                .iter()
                .find(|s| s.open_id == ws.open_id)
                .expect("seed for workspace");
            assert_component_parity(&graph, ws.node_id, tree, seed.pages);
        }
    }

    #[test]
    fn root_binding_reaches_every_workspace_on_seed_path() {
        let dir = std::env::temp_dir().join(format!(
            "innovator_root_{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("temp dir");
        let mut db = infinite_db::InfiniteDb::open(&dir).expect("db");
        crate::walls::ensure_walls_space(&mut db);
        crate::results::ensure_results_space(&mut db);
        let mut graph = Graph::new();
        let root = insert_root(&mut graph);
        let workspaces = crate::workspace::Workspace::from_seeds(&mut db, &mut graph);
        let ws_nodes: Vec<_> = workspaces.iter().map(|w| w.node_id).collect();
        bind_workspaces_to_root(&mut graph, root, &ws_nodes);
        let _ = std::fs::remove_dir_all(&dir);

        let bound = binding_children(&graph, root);
        assert_eq!(bound, ws_nodes);

        // Every Binding-reachable node from root is a UIView; unbound domain
        // entities (walls) remain outside the container tree until Phase 7.
        let mut reachable = HashSet::from([root]);
        let mut queue = VecDeque::from([root]);
        while let Some(n) = queue.pop_front() {
            for child in binding_children(&graph, n) {
                let is_uiview = graph
                    .nodes
                    .get(&child)
                    .is_some_and(|node| node.space_class == SpaceClass::UIView);
                if is_uiview && reachable.insert(child) {
                    queue.push_back(child);
                }
            }
        }
        for ws in &workspaces {
            assert!(
                reachable.contains(&ws.node_id),
                "workspace {:?} not reachable from root",
                ws.open_id
            );
        }
        let uiview_count = graph
            .nodes
            .values()
            .filter(|n| n.space_class == SpaceClass::UIView)
            .count();
        assert_eq!(
            reachable.len(),
            uiview_count,
            "UIView Binding component from root should include every UIView"
        );
    }

    #[test]
    fn particle_slots_reach_tier_depth_five() {
        let dir = std::env::temp_dir().join(format!(
            "innovator_tier5_{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("temp dir");
        let mut db = infinite_db::InfiniteDb::open(&dir).expect("db");
        crate::walls::ensure_walls_space(&mut db);
        crate::results::ensure_results_space(&mut db);
        let mut graph = Graph::new();
        let root = insert_root(&mut graph);
        let workspaces = crate::workspace::Workspace::from_seeds(&mut db, &mut graph);
        let ws_nodes: Vec<_> = workspaces.iter().map(|w| w.node_id).collect();
        bind_workspaces_to_root(&mut graph, root, &ws_nodes);
        let _ = std::fs::remove_dir_all(&dir);

        derive_tiers(&mut graph, root);
        assert_tier_consistency(&graph, root);

        let mut max_tier = 0i64;
        let mut particle_slots = 0usize;
        for node in graph.nodes.values() {
            if let Some(PropValue::I64(t)) = node.props.get(TIER_PROP) {
                max_tier = max_tier.max(*t);
            }
            if node.label.starts_with("particle:") {
                particle_slots += 1;
                assert_eq!(
                    node.props.get(TIER_PROP),
                    Some(&PropValue::I64(5)),
                    "particle slot should sit at tier 5"
                );
            }
        }
        assert!(
            max_tier >= 5,
            "expected Binding depth ≥5 after particle slots, got {max_tier}"
        );
        let expected_slots: usize = workspaces
            .iter()
            .filter_map(|ws| ws.page_tree())
            .map(|tree| count_particle_slots(&graph, tree))
            .sum();
        assert_eq!(particle_slots, expected_slots);
        assert!(particle_slots > 0);
    }

    #[test]
    fn home_launchable_signals_are_inbound_from_home() {
        let dir = std::env::temp_dir().join(format!(
            "innovator_nav_{}",
            std::process::id()
        ));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("temp dir");
        let mut db = infinite_db::InfiniteDb::open(&dir).expect("db");
        crate::walls::ensure_walls_space(&mut db);
        crate::results::ensure_results_space(&mut db);
        let mut graph = Graph::new();
        let root = insert_root(&mut graph);
        let workspaces = crate::workspace::Workspace::from_seeds(&mut db, &mut graph);
        let ws_nodes: Vec<_> = workspaces.iter().map(|w| w.node_id).collect();
        bind_workspaces_to_root(&mut graph, root, &ws_nodes);
        let home = workspaces
            .iter()
            .find(|w| w.open_id == "home")
            .expect("home");
        let targets: Vec<_> = seed::LAUNCHABLE
            .iter()
            .filter_map(|s| {
                workspaces
                    .iter()
                    .find(|w| w.open_id == s.open_id)
                    .map(|w| (w.node_id, s.open_id))
            })
            .collect();
        wire_home_nav_signals(&mut graph, home.node_id, &targets);
        let _ = std::fs::remove_dir_all(&dir);

        for seed in seed::LAUNCHABLE {
            let inbound: Vec<_> = graph
                .edges
                .values()
                .filter(|e| {
                    e.kind == EdgeKind::Signal
                        && e.targets.iter().any(|t| {
                            workspaces
                                .iter()
                                .any(|w| w.node_id == *t && w.open_id == seed.open_id)
                        })
                        && e.sources == [home.node_id]
                })
                .collect();
            assert_eq!(
                inbound.len(),
                1,
                "expected exactly one Home Signal to {}",
                seed.open_id
            );
            assert_eq!(
                resolve_home_nav_target(&graph, home.node_id, seed.open_id),
                workspaces
                    .iter()
                    .find(|w| w.open_id == seed.open_id)
                    .map(|w| w.node_id)
            );
        }
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
