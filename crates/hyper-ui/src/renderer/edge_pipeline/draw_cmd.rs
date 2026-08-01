mod from_endpoints;
mod to_instance;

use super::EdgeKindGpu;

/// CPU-side draw command before GPU upload.
#[derive(Debug, Clone)]
pub struct EdgeDrawCmd {
    pub p0: [f32; 2],
    pub p1: [f32; 2],
    pub p2: [f32; 2],
    pub p3: [f32; 2],
    pub color: [f32; 4],
    pub width: f32,
    pub arrow: bool,
    pub edge_kind: EdgeKindGpu,
}
