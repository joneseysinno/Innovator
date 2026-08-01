use super::super::{Globals, NodeInstance};
use super::NodePipeline;

impl NodePipeline {
    pub fn upload(&mut self, device: &wgpu::Device, queue: &wgpu::Queue, screen: [f32; 2]) {
        queue.write_buffer(
            &self.globals_buf,
            0,
            bytemuck::bytes_of(&Globals {
                screen_size: screen,
                _pad: [0.0; 2],
            }),
        );

        if self.instances.is_empty() {
            return;
        }

        if self.instances.len() > self.instance_capacity {
            self.instance_capacity = self.instances.len().next_power_of_two();
            self.instance_buf = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("node_instances"),
                size: (self.instance_capacity * std::mem::size_of::<NodeInstance>()) as u64,
                usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }

        queue.write_buffer(
            &self.instance_buf,
            0,
            bytemuck::cast_slice(&self.instances),
        );
    }
}
