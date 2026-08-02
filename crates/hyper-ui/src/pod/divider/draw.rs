use crate::renderer::node_pipeline::{NodeInstance, NodePipeline};

use super::{PodDivider, PodDividerRenderer};

impl PodDividerRenderer {
    pub fn draw_into(&self, rects: &mut NodePipeline) {
        for d in &self.dividers {
            push_divider(rects, d);
        }
    }
}

fn push_divider(rects: &mut NodePipeline, d: &PodDivider) {
    let color = if d.dragging {
        [0.35, 0.65, 0.95, 1.0]
    } else if d.hovered {
        [0.55, 0.58, 0.65, 1.0]
    } else {
        [0.30, 0.32, 0.36, 1.0]
    };
    // Visual line is 1px centered in the hit rect.
    let y = d.rect.origin.y + (d.rect.size.y - 1.0) * 0.5;
    rects.push(NodeInstance {
        position: [d.rect.origin.x, y],
        size: [d.rect.size.x, 1.0],
        color,
        border_color: [0.0; 4],
        border_radius: 0.0,
        border_width: 0.0,
        _pad: [0.0; 2],
    });
}
