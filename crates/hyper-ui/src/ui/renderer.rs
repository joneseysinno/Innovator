mod draw_focus_ring;
mod draw_rects;
mod layout;
mod new;
mod rebuild_draw_lists;
mod set_tree;

use crate::input::InputRouter;
use crate::particles::ParticleTree;
use crate::renderer::node_pipeline::NodePipeline;
use crate::seam::{PodTree, SeamRenderer};

pub struct UiRenderer {
    pub rects: NodePipeline,
    pub focus_ring: NodePipeline,
    pub tree: ParticleTree,
    pub input: InputRouter,
    pub seams: SeamRenderer,
    pub pods: PodTree,
}
