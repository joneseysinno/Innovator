use super::super::FrameCtx;
use super::HyperRenderer;

impl HyperRenderer {
    pub fn end_frame(&mut self, mut ctx: FrameCtx) {
        {
            let mut pass = ctx.encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("hyper-ui pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &ctx.view,
                    depth_slice: None,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.clear_color),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
                multiview_mask: None,
            });

            self.scene.draw(&mut pass);
            self.ui.draw_rects(&mut pass);
            // text rendered after rects
            let _ = self.text.render_into(&mut pass);
            self.ui.draw_focus_ring(&mut pass);
        }

        self.queue.submit(Some(ctx.encoder.finish()));
        self.queue.present(ctx.surface_texture);
        self.text.trim();
    }
}
