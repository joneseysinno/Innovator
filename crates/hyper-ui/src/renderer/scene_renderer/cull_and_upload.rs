use super::super::SpatialSource;
use super::SceneRenderer;

impl SceneRenderer {
    pub fn cull_and_upload(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        source: &dyn SpatialSource,
    ) {
        let world_rect = self.camera.visible_world_rect();
        let nodes = source.query_nodes_in_rect(world_rect);
        self.upload_node_instances(&nodes);

        let edge_cmds = source.query_edges_for_visible(world_rect);
        self.upload_edge_commands(&edge_cmds);

        let screen = [
            self.camera.screen_px.x as f32,
            self.camera.screen_px.y as f32,
        ];
        self.nodes.upload(device, queue, screen);
        self.edges.upload(device, queue, screen);
    }
}
