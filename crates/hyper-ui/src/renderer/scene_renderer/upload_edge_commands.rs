use crate::geom::Vec2;

use super::super::EdgeDrawCmd;
use super::SceneRenderer;

impl SceneRenderer {
    pub fn upload_edge_commands(&mut self, edges: &[EdgeDrawCmd]) {
        self.edges.clear();
        for e in edges {
            let cmd = if self.edges_are_world {
                let s = self.camera.world_to_screen(Vec2::new(e.p0[0], e.p0[1]));
                let t = self.camera.world_to_screen(Vec2::new(e.p3[0], e.p3[1]));
                EdgeDrawCmd::from_endpoints(
                    [s.x, s.y],
                    [t.x, t.y],
                    0.8,
                    e.color,
                    e.width.max(1.5),
                    e.edge_kind,
                )
            } else {
                e.clone()
            };
            self.edges.push(&cmd);
        }
    }
}
