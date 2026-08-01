use crate::particles::Particle;
use crate::ui::collect_tab_order;

use super::UiRenderer;

impl UiRenderer {
    pub fn set_tree(&mut self, root: Particle) {
        let tab = collect_tab_order(&root);
        self.tree = crate::particles::ParticleTree::new(root);
        self.input.set_tab_order(tab);
    }
}
