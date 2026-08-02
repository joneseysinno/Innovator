mod draw_focus_ring;
mod draw_rects;
mod layout;
mod new;
mod rebuild_draw_lists;
mod set_tree;

use crate::input::InputRouter;
use crate::particles::ParticleTree;
use crate::pod::PodDividerRenderer;
use crate::renderer::node_pipeline::NodePipeline;
use crate::seam::SeamRenderer;

pub struct UiRenderer {
    pub rects: NodePipeline,
    pub focus_ring: NodePipeline,
    pub tree: ParticleTree,
    pub input: InputRouter,
    /// Page boundaries — support split/merge.
    pub page_seams: SeamRenderer,
    /// Pod dividers — filled by the host each frame before draw.
    pub pod_dividers: PodDividerRenderer,
}
