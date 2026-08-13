//! Cross-container Analysis-page data-flow edges.
//!
//! Containment Bindings carry an `"order"` prop. The non-containment Bindings
//! here are explicitly role-tagged so graph queries can distinguish them.

use super::template_ids::{INPUT_FORM, WALL_LIST};
use super::StructuralWorkspace;
use hyper_ui::TemplateId;
use hypernode::{EdgeId, EdgeKind, Graph, HyperEdge, Node, NodeId, PropValue, SpaceClass};
use std::collections::BTreeMap;

const ROLE_PROP: &str = "role";
const ACTIVE_WALL_ROLE: &str = "active_wall";
const RESULTS_ROLE: &str = "results";
const LISTED_WALL_ROLE: &str = "listed_wall";
const ORDER_PROP: &str = "order";

/// Locate a pod UIView by its assigned template within this workspace.
pub fn pod_node_id(workspace: &StructuralWorkspace, template: TemplateId) -> Option<NodeId> {
    workspace.page_tree.pages.iter().find_map(|page| {
        page.pods.pods.iter().find_map(|pod| {
            (workspace.pod_templates.get(&(page.id, pod.id)) == Some(&template))
                .then_some(pod.node_id)
        })
    })
}

/// Refresh Wall List: one multi-target Stream + Binding containment for each wall.
pub fn wire_wall_list_streams(graph: &mut Graph, wall_list_pod: NodeId) {
    graph.edges.retain(|_, edge| {
        !(edge.kind == EdgeKind::Stream && edge.sources == [wall_list_pod])
            && !(edge.kind == EdgeKind::Binding
                && edge.sources == [wall_list_pod]
                && edge.props.get(ROLE_PROP) == Some(&PropValue::Text(LISTED_WALL_ROLE.into())))
    });

    let walls: Vec<_> = graph
        .nodes
        .values()
        .filter(|node| node.space_class == SpaceClass::Entity)
        .map(|node| node.id)
        .collect();

    if !walls.is_empty() {
        // One hyperedge: Wall List streams to every wall.
        insert_edge(
            graph,
            EdgeKind::Stream,
            vec![wall_list_pod],
            walls.clone(),
            None,
            BTreeMap::new(),
        );
    }

    // Bind domain entities into the container tree under the Wall List pod.
    for (order, wall) in walls.into_iter().enumerate() {
        insert_edge(
            graph,
            EdgeKind::Binding,
            vec![wall_list_pod],
            vec![wall],
            None,
            BTreeMap::from([
                (ROLE_PROP.into(), PropValue::Text(LISTED_WALL_ROLE.into())),
                (ORDER_PROP.into(), PropValue::I64(order as i64)),
            ]),
        );
    }
}

/// Replace Input Form's active-wall Binding. This Binding is role-tagged and
/// therefore cannot be mistaken for UIView containment.
pub fn wire_active_wall_binding(graph: &mut Graph, input_pod: NodeId, wall: NodeId) {
    graph.edges.retain(|_, edge| {
        !(edge.kind == EdgeKind::Binding
            && edge.sources == [input_pod]
            && edge.props.get(ROLE_PROP) == Some(&PropValue::Text(ACTIVE_WALL_ROLE.into())))
    });
    insert_edge(
        graph,
        EdgeKind::Binding,
        vec![input_pod],
        vec![wall],
        None,
        BTreeMap::from([(ROLE_PROP.into(), PropValue::Text(ACTIVE_WALL_ROLE.into()))]),
    );
}

/// Find or create the shared ACI 318 Function node.
pub fn ensure_aci_318_engine(graph: &mut Graph) -> NodeId {
    if let Some(node) = graph
        .nodes
        .values()
        .find(|node| node.space_class == SpaceClass::Function && node.label == "ACI 318 Engine")
    {
        return node.id;
    }
    graph.insert_node(Node {
        id: NodeId(0),
        space_class: SpaceClass::Function,
        label: "ACI 318 Engine".into(),
        world_pos: [0.0, 0.0],
        props: BTreeMap::new(),
    })
}

/// Wire the executable analysis path and bind/stream the resulting node to the
/// Results pod. The Input Form reaches `RunAnalysis` through its active-wall
/// Binding, so this flow starts from a UIView.
pub fn wire_run_analysis(
    graph: &mut Graph,
    wall: NodeId,
    engine: NodeId,
    results: NodeId,
    results_pod: NodeId,
) {
    clear_results_pod(graph, results_pod);
    graph.edges.retain(|_, edge| {
        !((edge.kind == EdgeKind::Signal
            && edge.targets == [engine]
            && edge.label.as_deref() == Some("RunAnalysis"))
            || (edge.kind == EdgeKind::Signal
                && edge.sources == [engine]
                && edge.label.as_deref() == Some("AnalysisComplete")))
    });

    insert_edge(
        graph,
        EdgeKind::Signal,
        vec![wall],
        vec![engine],
        Some("RunAnalysis"),
        BTreeMap::new(),
    );
    insert_edge(
        graph,
        EdgeKind::Signal,
        vec![engine],
        vec![results],
        Some("AnalysisComplete"),
        BTreeMap::new(),
    );
    insert_edge(
        graph,
        EdgeKind::Binding,
        vec![results_pod],
        vec![results],
        None,
        BTreeMap::from([(ROLE_PROP.into(), PropValue::Text(RESULTS_ROLE.into()))]),
    );
    insert_edge(
        graph,
        EdgeKind::Stream,
        vec![results],
        vec![results_pod],
        None,
        BTreeMap::new(),
    );
}

/// Remove the live result presented by a Results table UIView.
pub fn clear_results_pod(graph: &mut Graph, results_pod: NodeId) {
    let old_results: Vec<_> = graph
        .edges
        .values()
        .filter(|edge| {
            edge.kind == EdgeKind::Binding
                && edge.sources == [results_pod]
                && edge.props.get(ROLE_PROP) == Some(&PropValue::Text(RESULTS_ROLE.into()))
        })
        .flat_map(|edge| edge.targets.iter().copied())
        .collect();
    graph.edges.retain(|_, edge| {
        !(edge.kind == EdgeKind::Binding
            && edge.sources == [results_pod]
            && edge.props.get(ROLE_PROP) == Some(&PropValue::Text(RESULTS_ROLE.into())))
            && !(edge.kind == EdgeKind::Stream
                && edge.targets == [results_pod]
                && edge
                    .sources
                    .iter()
                    .any(|source| old_results.contains(source)))
    });
}

/// Return the Results node currently bound to the Results table UIView.
pub fn results_for_pod<'a>(graph: &'a Graph, results_pod: NodeId) -> Option<&'a Node> {
    graph
        .edges
        .values()
        .find(|edge| {
            edge.kind == EdgeKind::Binding
                && edge.sources == [results_pod]
                && edge.props.get(ROLE_PROP) == Some(&PropValue::Text(RESULTS_ROLE.into()))
        })
        .and_then(|edge| edge.targets.first())
        .and_then(|id| graph.nodes.get(id))
}

fn insert_edge(
    graph: &mut Graph,
    kind: EdgeKind,
    sources: Vec<NodeId>,
    targets: Vec<NodeId>,
    label: Option<&str>,
    props: BTreeMap<String, PropValue>,
) {
    graph.insert_edge(HyperEdge {
        id: EdgeId(0),
        kind,
        sources,
        targets,
        curvature: 0.25,
        label: label.map(str::to_owned),
        props,
    });
}

/// Wire the stable UIView-to-wall paths after a workspace is constructed.
pub fn wire_workspace(graph: &mut Graph, workspace: &StructuralWorkspace) {
    if let Some(wall_list) = pod_node_id(workspace, WALL_LIST) {
        wire_wall_list_streams(graph, wall_list);
    }
    if let (Some(input), Some(wall)) = (pod_node_id(workspace, INPUT_FORM), workspace.active_wall) {
        wire_active_wall_binding(graph, input, wall);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::workspace::graph_containers::{binding_children, insert_uiview};

    #[test]
    fn wall_list_stream_is_multi_target_hyperedge() {
        let mut graph = Graph::new();
        let pod = insert_uiview(&mut graph, "Wall List");
        let w1 = graph.insert_node(Node {
            id: NodeId(0),
            space_class: SpaceClass::Entity,
            label: "Wall A".into(),
            world_pos: [0.0, 0.0],
            props: BTreeMap::new(),
        });
        let w2 = graph.insert_node(Node {
            id: NodeId(0),
            space_class: SpaceClass::Entity,
            label: "Wall B".into(),
            world_pos: [1.0, 0.0],
            props: BTreeMap::new(),
        });

        wire_wall_list_streams(&mut graph, pod);

        let streams: Vec<_> = graph
            .edges
            .values()
            .filter(|e| e.kind == EdgeKind::Stream && e.sources == [pod])
            .collect();
        assert_eq!(streams.len(), 1, "expected a single Wall List Stream");
        assert!(
            streams[0].targets.len() > 1,
            "expected multi-target Stream, got {:?}",
            streams[0].targets
        );
        assert_eq!(streams[0].targets, vec![w1, w2]);

        let kids = binding_children(&graph, pod);
        assert!(kids.contains(&w1) && kids.contains(&w2));
    }
}
