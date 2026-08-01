use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct EdgeInstance {
    pub p0: [f32; 2],
    pub p1: [f32; 2],
    pub p2: [f32; 2],
    pub p3: [f32; 2],
    pub color: [f32; 4],
    pub width: f32,
    pub edge_kind: u32,
    pub arrow: u32,
    pub _pad: u32,
}
