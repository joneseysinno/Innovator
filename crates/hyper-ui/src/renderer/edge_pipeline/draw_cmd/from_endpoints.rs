use super::super::EdgeKindGpu;
use super::EdgeDrawCmd;

impl EdgeDrawCmd {
    /// Build a cubic from source→target with lateral curvature.
    pub fn from_endpoints(
        source: [f32; 2],
        target: [f32; 2],
        curvature: f32,
        color: [f32; 4],
        width: f32,
        kind: EdgeKindGpu,
    ) -> Self {
        let mx = (source[0] + target[0]) * 0.5;
        let my = (source[1] + target[1]) * 0.5;
        let dx = target[0] - source[0];
        let dy = target[1] - source[1];
        let len = (dx * dx + dy * dy).sqrt().max(1.0);
        let nx = -dy / len * curvature * len * 0.35;
        let ny = dx / len * curvature * len * 0.35;
        Self {
            p0: source,
            p1: [mx + nx, my + ny],
            p2: [mx + nx, my + ny],
            p3: target,
            color,
            width,
            arrow: true,
            edge_kind: kind,
        }
    }
}
