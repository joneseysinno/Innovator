use super::EdgeKindGpu;

/// World-space edge description (transformed during cull).
#[derive(Debug, Clone)]
pub struct WorldEdge {
    pub source: [f64; 2],
    pub target: [f64; 2],
    pub curvature: f32,
    pub color: [f32; 4],
    pub width: f32,
    pub kind: EdgeKindGpu,
}
