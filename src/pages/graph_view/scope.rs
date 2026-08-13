//! Resolve which nodes/edges belong in the current graph-view scope.

use crate::domains::graph_view::state::{GraphScope, GraphViewState};
use crate::workspace::graph_containers::binding_children;
use hypernode::{Graph, HyperEdge, Node, NodeId};
use std::collections::{HashSet, VecDeque};

/// Scoped, filtered view of the graph for rendering / layout.
pub struct ScopedGraph<'a> {
    pub nodes: Vec<&'a Node>,
    pub edges: Vec<&'a HyperEdge>,
}

pub fn resolve_scope<'a>(
    graph: &'a Graph,
    state: &GraphViewState,
    active_workspace: Option<NodeId>,
) -> ScopedGraph<'a> {
    let node_ids: HashSet<NodeId> = match state.scope {
        GraphScope::Composed => graph.nodes.keys().copied().collect(),
        GraphScope::ActiveWorkspace => match active_workspace {
            Some(root) => {
                let mut ids = HashSet::new();
                ids.insert(root);
                collect_binding_descendants(graph, root, &mut ids);
                ids
            }
            None => HashSet::new(),
        },
        GraphScope::Reachable => match active_workspace {
            Some(root) => bfs_all_kinds(graph, root),
            None => HashSet::new(),
        },
    };

    let nodes: Vec<&Node> = graph
        .nodes
        .values()
        .filter(|n| node_ids.contains(&n.id) && state.shows_space_class(n.space_class))
        .collect();
    let visible: HashSet<NodeId> = nodes.iter().map(|n| n.id).collect();

    let edges: Vec<&HyperEdge> = graph
        .edges
        .values()
        .filter(|e| {
            state.shows_edge_kind(e.kind)
                && e.sources.iter().any(|s| visible.contains(s))
                && e.targets.iter().any(|t| visible.contains(t))
                // Drop edges whose *kept* endpoints are empty after filter —
                // require at least one visible source AND one visible target.
                && !e.sources.is_empty()
                && !e.targets.is_empty()
        })
        .filter(|e| {
            // Dangling cleanup: every endpoint we draw must be visible; drop
            // if any listed endpoint was filtered out (partial hyperedges go away).
            e.sources.iter().all(|s| visible.contains(s))
                && e.targets.iter().all(|t| visible.contains(t))
        })
        .collect();

    ScopedGraph { nodes, edges }
}

fn collect_binding_descendants(graph: &Graph, parent: NodeId, out: &mut HashSet<NodeId>) {
    for child in binding_children(graph, parent) {
        if out.insert(child) {
            collect_binding_descendants(graph, child, out);
        }
    }
}

fn bfs_all_kinds(graph: &Graph, root: NodeId) -> HashSet<NodeId> {
    let mut seen = HashSet::new();
    let mut q = VecDeque::new();
    seen.insert(root);
    q.push_back(root);
    while let Some(n) = q.pop_front() {
        for edge in graph.edges.values() {
            let incident = edge.sources.contains(&n) || edge.targets.contains(&n);
            if !incident {
                continue;
            }
            for id in edge.sources.iter().chain(edge.targets.iter()) {
                if seen.insert(*id) {
                    q.push_back(*id);
                }
            }
        }
    }
    seen
}
