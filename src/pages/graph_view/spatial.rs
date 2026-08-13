//! Convert scoped graph + page-local positions → Layer A spatial.

use crate::domains::graph_view::state::GraphViewState;
use hyper_ui::{EdgeKindGpu, InMemoryWorldSpatial, SceneNode, Vec2, WorldEdge};
use hypernode::{EdgeKind, HyperEdge, Node, SpaceClass};

const NODE_SIZE: [f32; 2] = [1.1, 1.1];
const JUNCTION_SIZE: [f32; 2] = [0.35, 0.35];

pub fn build_spatial(
    state: &GraphViewState,
    nodes: &[&Node],
    edges: &[&HyperEdge],
    conductivity: &dyn Fn(u64, u64) -> f64,
) -> InMemoryWorldSpatial {
    let mut scene_nodes = Vec::with_capacity(nodes.len() + edges.len());
    let mut scene_edges = Vec::new();

    for node in nodes {
        let Some(pos) = state.positions.get(&node.id) else {
            continue;
        };
        let selected = state.selected == Some(node.id);
        let pinned = state.pinned.contains(&node.id);
        let (color, border) = space_class_colors(node.space_class, selected, pinned);
        scene_nodes.push(SceneNode {
            world_pos: [pos.x as f64, pos.y as f64],
            size_world: NODE_SIZE,
            color,
            border_color: border,
            border_radius: 0.45,
            border_width: if selected { 3.0 } else { 1.5 },
            selected,
        });
    }

    for edge in edges {
        let is_hyper = edge.sources.len() != 1 || edge.targets.len() != 1;
        let (base_color, width, kind) = edge_style(edge.kind);

        if !is_hyper {
            let s = edge.sources[0];
            let t = edge.targets[0];
            let Some(&ps) = state.positions.get(&s) else {
                continue;
            };
            let Some(&pt) = state.positions.get(&t) else {
                continue;
            };
            let cond = conductivity(s.0, t.0).clamp(0.05, 4.0) as f32;
            let incident = state.selected.is_some_and(|id| id == s || id == t);
            let (color, w) = emphasize(base_color, width * (0.6 + 0.5 * cond), incident);
            scene_edges.push(WorldEdge {
                source: [ps.x as f64, ps.y as f64],
                target: [pt.x as f64, pt.y as f64],
                curvature: edge.curvature.max(0.15),
                color,
                width: w,
                kind,
            });
        } else {
            let Some(&pj) = state.junctions.get(&edge.id) else {
                continue;
            };
            scene_nodes.push(SceneNode {
                world_pos: [pj.x as f64, pj.y as f64],
                size_world: JUNCTION_SIZE,
                color: [0.55, 0.55, 0.60, 0.85],
                border_color: [0.75, 0.75, 0.80, 1.0],
                border_radius: 0.5,
                border_width: 1.0,
                selected: false,
            });
            for &s in &edge.sources {
                let Some(&ps) = state.positions.get(&s) else {
                    continue;
                };
                push_segment(
                    &mut scene_edges,
                    ps,
                    pj,
                    base_color,
                    width,
                    kind,
                    edge.curvature,
                    state,
                    s,
                    conductivity(s.0, edge.id.0),
                );
            }
            for &t in &edge.targets {
                let Some(&pt) = state.positions.get(&t) else {
                    continue;
                };
                push_segment(
                    &mut scene_edges,
                    pj,
                    pt,
                    base_color,
                    width,
                    kind,
                    edge.curvature,
                    state,
                    t,
                    conductivity(edge.id.0, t.0),
                );
            }
        }
    }

    InMemoryWorldSpatial {
        nodes: scene_nodes,
        edges: scene_edges,
    }
}

fn push_segment(
    out: &mut Vec<WorldEdge>,
    from: Vec2,
    to: Vec2,
    base_color: [f32; 4],
    width: f32,
    kind: EdgeKindGpu,
    curvature: f32,
    state: &GraphViewState,
    endpoint: hypernode::NodeId,
    cond: f64,
) {
    let incident = state.selected == Some(endpoint);
    let cond = cond.clamp(0.05, 4.0) as f32;
    let (color, w) = emphasize(base_color, width * (0.6 + 0.5 * cond), incident);
    out.push(WorldEdge {
        source: [from.x as f64, from.y as f64],
        target: [to.x as f64, to.y as f64],
        curvature: curvature.max(0.1),
        color,
        width: w,
        kind,
    });
}

fn emphasize(color: [f32; 4], width: f32, incident: bool) -> ([f32; 4], f32) {
    if incident {
        (
            [color[0], color[1], color[2], (color[3] * 1.2).min(1.0)],
            width * 1.6,
        )
    } else {
        (color, width)
    }
}

fn space_class_colors(
    class: SpaceClass,
    selected: bool,
    pinned: bool,
) -> ([f32; 4], [f32; 4]) {
    let (r, g, b) = match class {
        SpaceClass::UIView => (0.35, 0.65, 0.95),
        SpaceClass::Entity => (0.90, 0.55, 0.25),
        SpaceClass::Function => (0.45, 0.85, 0.50),
        SpaceClass::Carrier => (0.75, 0.45, 0.90),
    };
    let a = if selected { 1.0 } else { 0.92 };
    let border = if pinned {
        [1.0, 0.85, 0.30, 1.0]
    } else if selected {
        [1.0, 1.0, 1.0, 1.0]
    } else {
        [r * 1.15, g * 1.15, b * 1.15, 1.0]
    };
    ([r, g, b, a], border)
}

fn edge_style(kind: EdgeKind) -> ([f32; 4], f32, EdgeKindGpu) {
    match kind {
        EdgeKind::Binding => ([0.45, 0.48, 0.55, 0.55], 1.2, EdgeKindGpu::Binding),
        EdgeKind::Signal => ([0.55, 0.80, 0.95, 0.90], 2.0, EdgeKindGpu::Signal),
        EdgeKind::Stream => ([0.40, 0.90, 0.70, 0.95], 2.2, EdgeKindGpu::Stream),
        EdgeKind::Wave => ([0.95, 0.70, 0.35, 0.90], 2.0, EdgeKindGpu::Wave),
    }
}
