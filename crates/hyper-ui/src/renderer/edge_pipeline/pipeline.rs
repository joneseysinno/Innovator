mod clear;
mod draw;
mod new;
mod push;
mod upload;

use super::EdgeInstance;

pub struct EdgePipeline {
    pipeline: wgpu::RenderPipeline,
    globals_buf: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    instance_buf: wgpu::Buffer,
    instance_capacity: usize,
    instances: Vec<EdgeInstance>,
    start_time: std::time::Instant,
}
