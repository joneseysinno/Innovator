mod bind_group_layout;
mod clear;
mod draw;
mod instance_count;
mod new;
mod push;
mod upload;

use super::NodeInstance;

pub struct NodePipeline {
    pipeline: wgpu::RenderPipeline,
    bind_group_layout: wgpu::BindGroupLayout,
    globals_buf: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    instance_buf: wgpu::Buffer,
    instance_capacity: usize,
    instances: Vec<NodeInstance>,
}
