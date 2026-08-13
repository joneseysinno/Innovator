//! Spring-embedder force layout for graph-view positions.

use crate::domains::graph_view::state::GraphViewState;
use hyper_ui::Vec2;
use hypernode::{EdgeId, EdgeKind, HyperEdge, NodeId};
use std::collections::HashMap;

const REPULSION: f32 = 80.0;
const SPRING_LEN_BINDING: f32 = 2.5;
const SPRING_LEN_DEFAULT: f32 = 5.0;
const SPRING_K_BINDING: f32 = 0.08;
const SPRING_K_DEFAULT: f32 = 0.03;
const JUNCTION_MASS: f32 = 0.35;
const MIN_DIST: f32 = 0.35;
const ALPHA_DECAY: f32 = 0.985;
const ALPHA_FLOOR: f32 = 0.005;

/// Seed positions on a circle when the view first opens.
pub fn seed_circle(state: &mut GraphViewState, node_ids: &[NodeId], edge_ids: &[EdgeId]) {
    let n = node_ids.len().max(1) as f32;
    let radius = (n.sqrt() * 3.0).max(4.0);
    for (i, id) in node_ids.iter().enumerate() {
        if state.positions.contains_key(id) {
            continue;
        }
        let t = (i as f32) / n * std::f32::consts::TAU;
        state
            .positions
            .insert(*id, Vec2::new(radius * t.cos(), radius * t.sin()));
    }
    for (i, id) in edge_ids.iter().enumerate() {
        if state.junctions.contains_key(id) {
            continue;
        }
        let t = (i as f32 + 0.5) / (edge_ids.len().max(1) as f32) * std::f32::consts::TAU;
        state
            .junctions
            .insert(*id, Vec2::new(radius * 0.4 * t.cos(), radius * 0.4 * t.sin()));
    }
    state.seeded = true;
    state.alpha = 1.0;
}

/// Drop positions for nodes/junctions no longer in scope.
pub fn prune_positions(state: &mut GraphViewState, nodes: &[NodeId], hyper_edges: &[EdgeId]) {
    let node_set: std::collections::HashSet<_> = nodes.iter().copied().collect();
    let edge_set: std::collections::HashSet<_> = hyper_edges.iter().copied().collect();
    state.positions.retain(|id, _| node_set.contains(id));
    state.junctions.retain(|id, _| edge_set.contains(id));
    state.pinned.retain(|id| node_set.contains(id));
    if state.selected.is_some_and(|id| !node_set.contains(&id)) {
        state.selected = None;
    }
}

/// One cooling frame of the force simulation.
pub fn step(state: &mut GraphViewState, edges: &[&HyperEdge]) {
    if state.alpha < ALPHA_FLOOR && state.pinned.is_empty() {
        return;
    }

    let ids: Vec<NodeId> = state.positions.keys().copied().collect();
    let mut forces: HashMap<NodeId, Vec2> = ids.iter().map(|id| (*id, Vec2::ZERO)).collect();
    let mut j_forces: HashMap<EdgeId, Vec2> =
        state.junctions.keys().map(|id| (*id, Vec2::ZERO)).collect();

    // All-pairs repulsion between real nodes.
    for i in 0..ids.len() {
        for j in (i + 1)..ids.len() {
            let a = ids[i];
            let b = ids[j];
            let pa = state.positions[&a];
            let pb = state.positions[&b];
            let (fx, fy) = repulsion(pa, pb);
            let fa = forces.get_mut(&a).unwrap();
            *fa = *fa + Vec2::new(fx, fy);
            let fb = forces.get_mut(&b).unwrap();
            *fb = *fb + Vec2::new(-fx, -fy);
        }
    }

    for edge in edges {
        let is_hyper = edge.sources.len() != 1 || edge.targets.len() != 1;
        let (k, rest) = spring_params(edge.kind);
        if !is_hyper {
            let s = edge.sources[0];
            let t = edge.targets[0];
            let Some(&ps) = state.positions.get(&s) else {
                continue;
            };
            let Some(&pt) = state.positions.get(&t) else {
                continue;
            };
            let (fx, fy) = spring(ps, pt, rest, k);
            if let Some(f) = forces.get_mut(&s) {
                *f = *f + Vec2::new(fx, fy);
            }
            if let Some(f) = forces.get_mut(&t) {
                *f = *f + Vec2::new(-fx, -fy);
            }
        } else if let Some(&pj) = state.junctions.get(&edge.id) {
            for &s in &edge.sources {
                let Some(&ps) = state.positions.get(&s) else {
                    continue;
                };
                let (fx, fy) = spring(ps, pj, rest, k);
                if let Some(f) = forces.get_mut(&s) {
                    *f = *f + Vec2::new(fx, fy);
                }
                if let Some(f) = j_forces.get_mut(&edge.id) {
                    *f = *f + Vec2::new(-fx, -fy);
                }
            }
            for &t in &edge.targets {
                let Some(&pt) = state.positions.get(&t) else {
                    continue;
                };
                let (fx, fy) = spring(pj, pt, rest, k);
                if let Some(f) = j_forces.get_mut(&edge.id) {
                    *f = *f + Vec2::new(fx, fy);
                }
                if let Some(f) = forces.get_mut(&t) {
                    *f = *f + Vec2::new(-fx, -fy);
                }
            }
        }
    }

    let alpha = state.alpha;
    for id in ids {
        if state.pinned.contains(&id) {
            continue;
        }
        let f = forces[&id];
        let p = state.positions.get_mut(&id).unwrap();
        p.x += f.x * alpha;
        p.y += f.y * alpha;
    }
    for (id, f) in j_forces {
        let p = state.junctions.get_mut(&id).unwrap();
        p.x += f.x * alpha * JUNCTION_MASS;
        p.y += f.y * alpha * JUNCTION_MASS;
    }

    state.alpha = (state.alpha * ALPHA_DECAY).max(ALPHA_FLOOR);
}

fn spring_params(kind: EdgeKind) -> (f32, f32) {
    match kind {
        EdgeKind::Binding => (SPRING_K_BINDING, SPRING_LEN_BINDING),
        _ => (SPRING_K_DEFAULT, SPRING_LEN_DEFAULT),
    }
}

fn repulsion(a: Vec2, b: Vec2) -> (f32, f32) {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    let dist_sq = (dx * dx + dy * dy).max(MIN_DIST * MIN_DIST);
    let dist = dist_sq.sqrt();
    let force = REPULSION / dist_sq;
    (force * dx / dist, force * dy / dist)
}

fn spring(a: Vec2, b: Vec2, rest: f32, k: f32) -> (f32, f32) {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    let dist = (dx * dx + dy * dy).sqrt().max(1e-4);
    let displace = dist - rest;
    let force = k * displace;
    (force * dx / dist, force * dy / dist)
}

/// Total positional delta after a step (for stability tests).
pub fn total_delta(before: &HashMap<NodeId, Vec2>, after: &HashMap<NodeId, Vec2>) -> f32 {
    before
        .iter()
        .map(|(id, p)| {
            after
                .get(id)
                .map(|q| {
                    let dx = p.x - q.x;
                    let dy = p.y - q.y;
                    (dx * dx + dy * dy).sqrt()
                })
                .unwrap_or(0.0)
        })
        .sum()
}

/// Minimum pairwise distance among non-junction nodes.
pub fn min_node_distance(positions: &HashMap<NodeId, Vec2>) -> f32 {
    let ids: Vec<_> = positions.keys().copied().collect();
    let mut min = f32::MAX;
    for i in 0..ids.len() {
        for j in (i + 1)..ids.len() {
            let a = positions[&ids[i]];
            let b = positions[&ids[j]];
            let d = ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt();
            if d < min {
                min = d;
            }
        }
    }
    if min == f32::MAX {
        f32::MAX
    } else {
        min
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use hypernode::{EdgeId, HyperEdge};

    #[test]
    fn force_layout_cools_and_separates() {
        let mut state = GraphViewState::default();
        let nodes: Vec<NodeId> = (1..=8).map(NodeId).collect();
        let edges_owned: Vec<HyperEdge> = (0..7)
            .map(|i| HyperEdge {
                id: EdgeId(i),
                kind: if i % 2 == 0 {
                    EdgeKind::Binding
                } else {
                    EdgeKind::Signal
                },
                sources: vec![nodes[i as usize]],
                targets: vec![nodes[i as usize + 1]],
                curvature: 0.0,
                label: None,
                props: Default::default(),
            })
            .collect();
        let edge_ids: Vec<_> = edges_owned.iter().map(|e| e.id).collect();
        seed_circle(&mut state, &nodes, &edge_ids);

        let mut prev_delta = f32::MAX;
        for frame in 0..400 {
            let before = state.positions.clone();
            let refs: Vec<&HyperEdge> = edges_owned.iter().collect();
            step(&mut state, &refs);
            let delta = total_delta(&before, &state.positions);
            if frame > 80 && frame % 40 == 0 {
                assert!(
                    delta <= prev_delta * 1.25,
                    "delta should trend down: frame={frame} delta={delta} prev={prev_delta}"
                );
                prev_delta = delta;
            }
        }
        assert!(
            state.alpha <= 0.05,
            "alpha should cool: {}",
            state.alpha
        );
        let min_d = min_node_distance(&state.positions);
        assert!(
            min_d >= MIN_DIST * 0.5,
            "nodes should not collapse: min_d={min_d}"
        );
    }
}
