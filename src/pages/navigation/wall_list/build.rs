use super::build_row::build_row;
use super::WallListIO;
use crate::workspace::signal::WorkspaceSignal;
use hyper_ui::particles::{
    Particle, SourceParticle, StackParticle, SurfaceParticle, TriggerParticle, ViewParticle,
};
use hypernode::{Graph, NodeId};
use std::collections::HashMap;

/// Build the WallListIO pod from the current wall graph.
pub fn build_wall_list(graph: &Graph, active_wall: Option<NodeId>) -> WallListIO {
    let title = SourceParticle::new("Walls").with_weight(500);

    let mut sinks = HashMap::new();
    let mut rows = Vec::new();
    for node in graph.nodes.values() {
        let (row, sink_id) = build_row(node, active_wall);
        sinks.insert(sink_id, node.id);
        rows.push(row);
    }

    let new_wall = TriggerParticle::new("+ New Wall");
    let mut triggers = HashMap::new();
    triggers.insert(new_wall.id, WorkspaceSignal::NewWall);

    let mut children = vec![Particle::Source(title)];
    children.extend(rows);
    children.push(Particle::Trigger(new_wall));

    let body = StackParticle::column(children).with_gap(6.0);

    let surface = SurfaceParticle::new([0.12, 0.13, 0.16, 1.0])
        .with_padding(10.0)
        .with_radius(0.0)
        .with_border([0.26, 0.28, 0.32, 1.0], 1.0)
        .with_child(Particle::Stack(body));

    let mut view = ViewParticle::new("wall_list");
    view.child = Some(Box::new(Particle::Surface(surface)));

    WallListIO {
        particle: Particle::View(view),
        sinks,
        triggers,
    }
}
