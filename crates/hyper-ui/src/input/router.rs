mod blur_current;
mod focus_next;
mod handle_key;
mod route;

use crate::geom::Vec2;
use crate::layout::InputClass;
use crate::particles::ParticleId;

#[derive(Debug, Default)]
pub struct InputRouter {
    pub focused: Option<ParticleId>,
    pub hovered: Option<ParticleId>,
    pub pressed: Option<ParticleId>,
    pub cursor: Vec2,
    /// Tab-order of focusable field/trigger ids, rebuilt by the app/demo.
    pub tab_order: Vec<ParticleId>,
    /// Pointer vs touch — enables drag-to-scroll on viewports for Touch/Hybrid.
    pub input_class: InputClass,
    modifiers_shift: bool,
    /// Active viewport drag-scroll: (viewport id, last cursor axis position).
    scroll_drag: Option<(ParticleId, f32)>,
}

impl InputRouter {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_tab_order(&mut self, order: Vec<ParticleId>) {
        self.tab_order = order;
    }

    pub fn set_input_class(&mut self, class: InputClass) {
        self.input_class = class;
    }
}
