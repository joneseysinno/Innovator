use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub(crate) struct Globals {
    pub(crate) screen_size: [f32; 2],
    pub(crate) time: f32,
    pub(crate) _pad: f32,
}
