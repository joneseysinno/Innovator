use crate::geom::Vec2;

use super::super::{NodeInstance, SceneNode};
use super::SceneRenderer;

impl SceneRenderer {
    pub fn upload_node_instances(&mut self, nodes: &[SceneNode]) {
        self.nodes.clear();
        for n in nodes {
            let pos = self
                .camera
                .world_to_screen(Vec2::new(n.world_pos[0] as f32, n.world_pos[1] as f32));
            let size = [
                n.size_world[0] * self.camera.zoom,
                n.size_world[1] * self.camera.zoom,
            ];
            let mut color = n.color;
            if n.selected {
                color = [0.2, 0.55, 0.95, 1.0];
            }
            self.nodes.push(NodeInstance {
                position: [pos.x - size[0] * 0.5, pos.y - size[1] * 0.5],
                size,
                color,
                border_color: n.border_color,
                border_radius: n.border_radius * self.camera.zoom,
                border_width: n.border_width,
                _pad: [0.0; 2],
            });
        }
    }
}
