use crate::particles::{Particle, ParticleId};
use crate::renderer::node_pipeline::NodeInstance;
use crate::text::{self, TextRenderer};
use crate::ui::collect_rects;

use super::UiRenderer;

impl UiRenderer {
    pub fn rebuild_draw_lists(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        text: &mut TextRenderer,
        screen: [f32; 2],
        focused: Option<ParticleId>,
    ) {
        self.rects.clear();
        self.focus_ring.clear();
        text.clear_pending();

        if let Some(root) = self.tree.root.as_ref() {
            collect_rects(root, &mut self.rects, focused);
            text::collect_text(root, text, focused);
        }

        // Seams
        for cmd in self.seams.draw_commands() {
            let (origin, size) = cmd.line_rect();
            let color = if cmd.dragging {
                [0.35, 0.65, 0.95, 1.0]
            } else if cmd.hovered {
                [0.55, 0.58, 0.65, 1.0]
            } else {
                [0.30, 0.32, 0.36, 1.0]
            };
            self.rects.push(NodeInstance {
                position: [origin.x, origin.y],
                size: [size.x, size.y],
                color,
                border_color: [0.0; 4],
                border_radius: 0.0,
                border_width: 0.0,
                _pad: [0.0; 2],
            });
        }

        if let Some(id) = focused {
            if let Some(p) = self.tree.find(id) {
                if matches!(p, Particle::Field(f) if !f.read_only) {
                    let l = p.layout();
                    self.focus_ring.push(NodeInstance {
                        position: [l.origin.x - 1.0, l.origin.y - 1.0],
                        size: [l.size.x + 2.0, l.size.y + 2.0],
                        color: [0.0, 0.0, 0.0, 0.0],
                        border_color: [0.30, 0.60, 0.98, 1.0],
                        border_radius: 5.0,
                        border_width: 2.0,
                        _pad: [0.0; 2],
                    });
                }
            }
        }

        self.rects.upload(device, queue, screen);
        self.focus_ring.upload(device, queue, screen);
        text.prepare(device, queue);
    }
}
