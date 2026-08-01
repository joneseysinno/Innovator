/// Per-frame context handed to the application between begin/end.
pub struct FrameCtx {
    pub encoder: wgpu::CommandEncoder,
    pub view: wgpu::TextureView,
    pub surface_texture: wgpu::SurfaceTexture,
    pub width: u32,
    pub height: u32,
}
