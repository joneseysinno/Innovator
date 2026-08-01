//! Builds and queries a small HyperNode graph — no Innovator types.

use hypernode::{
    hilbert_encode_2d, world_to_hilbert_cell, EdgeKind, Graph, HyperEdge, HyperNode, Node,
    NodeId, PropValue, SpaceClass,
};
use std::collections::BTreeMap;

fn main() {
    let mut graph = Graph::new();

    let wall = graph.insert_node(Node {
        id: NodeId(0),
        space_class: SpaceClass::Entity,
        label: "Wall A".into(),
        world_pos: [10.0, 20.0],
        props: BTreeMap::from([
            ("height".into(), PropValue::F64(12.0)),
            ("thickness".into(), PropValue::F64(8.0)),
        ]),
    });

    let results = graph.insert_node(Node {
        id: NodeId(0),
        space_class: SpaceClass::Entity,
        label: "Results A".into(),
        world_pos: [12.0, 20.0],
        props: BTreeMap::new(),
    });

    let engine = graph.insert_node(Node {
        id: NodeId(0),
        space_class: SpaceClass::Function,
        label: "ACI 318 Engine".into(),
        world_pos: [11.0, 22.0],
        props: BTreeMap::new(),
    });

    graph.insert_edge(HyperEdge {
        id: hypernode::EdgeId(0),
        kind: EdgeKind::Signal,
        sources: vec![wall],
        targets: vec![engine],
        curvature: 0.35,
        label: Some("RunAnalysis".into()),
    });

    graph.insert_edge(HyperEdge {
        id: hypernode::EdgeId(0),
        kind: EdgeKind::Signal,
        sources: vec![engine],
        targets: vec![results],
        curvature: 0.35,
        label: Some("AnalysisComplete".into()),
    });

    let hit = graph.nodes_in_bbox([9.0, 19.0], [13.0, 23.0]);
    println!("nodes in bbox: {}", hit.len());
    for n in &hit {
        let cx = world_to_hilbert_cell(n.world_pos()[0], 0.0, 1.0, 8);
        let cy = world_to_hilbert_cell(n.world_pos()[1], 0.0, 1.0, 8);
        let h = hilbert_encode_2d(cx, cy, 8);
        println!(
            "  {} ({:?}) hilbert={h} height={:?}",
            n.label(),
            n.space_class(),
            n.get_prop("height")
        );
    }

    let edges = graph.edges_for_nodes(&[wall, engine, results]);
    println!("edges touching selection: {}", edges.len());
    for e in edges {
        println!("  {:?} {:?} → {:?}", e.label, e.sources, e.targets);
    }
}
