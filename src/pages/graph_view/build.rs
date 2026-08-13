//! Build Graph View page particles (canvas + inspector + filter chips).

use super::force::{prune_positions, seed_circle, step as force_step};
use super::scope::resolve_scope;
use super::spatial::build_spatial;
use crate::domains::graph_view::state::GraphScope;
use crate::domains::graph_view::workspace::{GraphFilterAction, GraphViewWorkspace};
use hyper_ui::particles::{
    Particle, SinkParticle, SourceParticle, StackParticle, SurfaceParticle, TriggerParticle,
    ViewParticle,
};
use hyper_ui::Vec2;
use hypernode::{EdgeKind, Graph, NodeId, SpaceClass};
use physarum::PhysarumNetwork;

/// Refresh layout + spatial from the live graph (call before building pods).
pub fn sync_graph_view(
    ws: &mut GraphViewWorkspace,
    graph: &Graph,
    active_workspace: Option<NodeId>,
) {
    ws.particle_sinks.clear();
    ws.filter_triggers.clear();

    let scoped = resolve_scope(graph, &ws.state, active_workspace);
    let node_ids: Vec<NodeId> = scoped.nodes.iter().map(|n| n.id).collect();
    let hyper_edge_ids: Vec<_> = scoped
        .edges
        .iter()
        .filter(|e| e.sources.len() != 1 || e.targets.len() != 1)
        .map(|e| e.id)
        .collect();

    prune_positions(&mut ws.state, &node_ids, &hyper_edge_ids);
    if !ws.state.seeded || ws.state.positions.len() < node_ids.len() {
        seed_circle(&mut ws.state, &node_ids, &hyper_edge_ids);
    }

    for _ in 0..4 {
        force_step(&mut ws.state, &scoped.edges);
    }

    sync_physarum(ws, &scoped.edges);
    let physarum = ws.physarum.clone();
    let cond = move |a: u64, b: u64| {
        physarum
            .as_ref()
            .map(|n| n.conductivity(a, b))
            .unwrap_or(1.0)
    };
    ws.spatial = build_spatial(&ws.state, &scoped.nodes, &scoped.edges, &cond);
}

/// Graph pod: filter chips + canvas sink.
pub fn build_canvas_pod(ws: &mut GraphViewWorkspace) -> Particle {
    let filters = build_filter_bar(ws);
    let canvas = build_canvas(ws);
    Particle::Stack(StackParticle::column(vec![filters, canvas]).with_gap(0.0))
}

/// Inspector pod: selected node details.
pub fn build_inspector_pod(ws: &GraphViewWorkspace, graph: &Graph) -> Particle {
    build_inspector(ws, graph)
}

/// Convenience: full-page column (tests / single-pod).
pub fn build_graph_view(
    ws: &mut GraphViewWorkspace,
    graph: &Graph,
    active_workspace: Option<NodeId>,
) -> Particle {
    sync_graph_view(ws, graph, active_workspace);
    Particle::Stack(
        StackParticle::column(vec![
            build_canvas_pod(ws),
            build_inspector_pod(ws, graph),
        ])
        .with_gap(0.0),
    )
}

fn sync_physarum(ws: &mut GraphViewWorkspace, edges: &[&hypernode::HyperEdge]) {
    let net = ws.physarum.get_or_insert_with(PhysarumNetwork::new);
    for edge in edges {
        for s in &edge.sources {
            for t in &edge.targets {
                let len = ws
                    .state
                    .positions
                    .get(s)
                    .zip(ws.state.positions.get(t))
                    .map(|(a, b)| {
                        let dx = a.x - b.x;
                        let dy = a.y - b.y;
                        ((dx * dx + dy * dy).sqrt() as f64).max(0.5)
                    })
                    .unwrap_or(1.0);
                net.add_edge(s.0, t.0, len);
            }
        }
    }
    if let Some(sel) = ws.state.selected {
        let sinks: Vec<(u64, f64)> = edges
            .iter()
            .flat_map(|e| e.targets.iter().map(|t| (t.0, 0.15)))
            .collect();
        net.inject(&[(sel.0, 1.0)], &sinks);
    } else if let Some((&id, _)) = ws.state.positions.iter().next() {
        net.inject(&[(id.0, 0.5)], &[]);
    }
    net.step(0.15);
}

fn build_canvas(ws: &mut GraphViewWorkspace) -> Particle {
    let chrome = StackParticle::column(vec![
        Particle::Source(SourceParticle::new("Graph").with_weight(500)),
        Particle::Source(SourceParticle::secondary(
            "drag pan · scroll zoom · click select · drag node pins",
        )),
    ])
    .with_gap(4.0);
    let surface = SurfaceParticle::new([0.10, 0.11, 0.14, 1.0])
        .with_padding(8.0)
        .with_radius(0.0)
        .with_border([0.26, 0.28, 0.32, 1.0], 1.0)
        .with_child(Particle::Stack(chrome));
    let sink = SinkParticle::new().with_child(Particle::Surface(surface));
    ws.graph_view_sink = Some(sink.id);
    let mut view = ViewParticle::new("graph_canvas");
    view.child = Some(Box::new(Particle::Sink(sink)));
    Particle::View(view)
}

fn build_inspector(ws: &GraphViewWorkspace, graph: &Graph) -> Particle {
    let body = match ws.state.selected.and_then(|id| graph.nodes.get(&id)) {
        Some(node) => {
            let pinned = if ws.state.pinned.contains(&node.id) {
                "pinned"
            } else {
                "free"
            };
            StackParticle::column(vec![
                Particle::Source(
                    SourceParticle::new(format!("Node {}", node.id.0)).with_weight(500),
                ),
                Particle::Source(SourceParticle::secondary(format!("label: {}", node.label))),
                Particle::Source(SourceParticle::secondary(format!(
                    "class: {:?}",
                    node.space_class
                ))),
                Particle::Source(SourceParticle::muted(format!("layout: {pinned}"))),
                Particle::Source(SourceParticle::muted(format!(
                    "props: {}",
                    node.props.len()
                ))),
            ])
            .with_gap(4.0)
        }
        None => StackParticle::column(vec![
            Particle::Source(SourceParticle::new("Inspector").with_weight(500)),
            Particle::Source(SourceParticle::secondary("Select a node")),
        ])
        .with_gap(4.0),
    };
    Particle::Surface(
        SurfaceParticle::new([0.12, 0.13, 0.16, 1.0])
            .with_padding(8.0)
            .with_radius(0.0)
            .with_border([0.26, 0.28, 0.32, 1.0], 1.0)
            .with_child(Particle::Stack(body)),
    )
}

fn build_filter_bar(ws: &mut GraphViewWorkspace) -> Particle {
    let mut chips = Vec::new();

    for (label, scope) in [
        ("Composed", GraphScope::Composed),
        ("Workspace", GraphScope::ActiveWorkspace),
        ("Reachable", GraphScope::Reachable),
    ] {
        let active = ws.state.scope == scope;
        let t = if active {
            TriggerParticle::primary(label)
        } else {
            TriggerParticle::new(label)
        };
        ws.filter_triggers
            .insert(t.id, GraphFilterAction::Scope(scope));
        chips.push(Particle::Trigger(t));
    }

    chips.push(Particle::Source(SourceParticle::muted("·")));

    for (label, class) in [
        ("UIView", SpaceClass::UIView),
        ("Entity", SpaceClass::Entity),
        ("Fn", SpaceClass::Function),
        ("Carrier", SpaceClass::Carrier),
    ] {
        let on = !ws.state.space_classes.is_empty() && ws.state.space_classes.contains(&class);
        let t = if on {
            TriggerParticle::primary(label)
        } else {
            TriggerParticle::new(label)
        };
        ws.filter_triggers
            .insert(t.id, GraphFilterAction::ToggleSpace(class));
        chips.push(Particle::Trigger(t));
    }

    chips.push(Particle::Source(SourceParticle::muted("·")));

    for (label, kind) in [
        ("Bind", EdgeKind::Binding),
        ("Signal", EdgeKind::Signal),
        ("Stream", EdgeKind::Stream),
        ("Wave", EdgeKind::Wave),
    ] {
        let on = !ws.state.edge_kinds.is_empty() && ws.state.edge_kinds.contains(&kind);
        let t = if on {
            TriggerParticle::primary(label)
        } else {
            TriggerParticle::new(label)
        };
        ws.filter_triggers
            .insert(t.id, GraphFilterAction::ToggleEdge(kind));
        chips.push(Particle::Trigger(t));
    }

    Particle::Surface(
        SurfaceParticle::new([0.11, 0.12, 0.15, 1.0])
            .with_padding(6.0)
            .with_radius(0.0)
            .with_child(Particle::Stack(StackParticle::row(chips).with_gap(6.0))),
    )
}

/// Hit-test a world-space point against node positions.
pub fn hit_test(ws: &GraphViewWorkspace, world: Vec2) -> Option<NodeId> {
    let mut best: Option<(NodeId, f32)> = None;
    let hit_r = 0.7_f32;
    for (&id, pos) in &ws.state.positions {
        let dx = pos.x - world.x;
        let dy = pos.y - world.y;
        let d = (dx * dx + dy * dy).sqrt();
        if d <= hit_r && best.is_none_or(|(_, bd)| d < bd) {
            best = Some((id, d));
        }
    }
    best.map(|(id, _)| id)
}
