mod blur_current;
mod focus_next;
mod handle_key;
mod route;

use crate::geom::Vec2;
use crate::layout::InputClass;
use crate::particles::ParticleId;

#[derive(Debug)]
pub struct InputRouter {
    pub focused: Option<ParticleId>,
    pub hovered: Option<ParticleId>,
    pub pressed: Option<ParticleId>,
    pub cursor: Vec2,
    /// Tab-order of focusable field/trigger ids, rebuilt by the app/demo.
    pub tab_order: Vec<ParticleId>,
    /// Pointer vs touch — enables drag-to-scroll on viewports for Touch/Hybrid.
    pub input_class: InputClass,
    /// Window scale factor — converts winit physical pointer coords → logical.
    pub scale_factor: f32,
    modifiers_shift: bool,
    /// Active viewport drag-scroll: (viewport id, last cursor axis position).
    scroll_drag: Option<(ParticleId, f32)>,
}

impl Default for InputRouter {
    fn default() -> Self {
        Self {
            focused: None,
            hovered: None,
            pressed: None,
            cursor: Vec2::ZERO,
            tab_order: Vec::new(),
            input_class: InputClass::Pointer,
            scale_factor: 1.0,
            modifiers_shift: false,
            scroll_drag: None,
        }
    }
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

    pub fn set_scale_factor(&mut self, scale: f32) {
        self.scale_factor = scale.max(0.01);
    }

    /// Convert a physical winit position into logical UI coordinates.
    pub(crate) fn to_logical(&self, x: f64, y: f64) -> Vec2 {
        let s = self.scale_factor.max(0.01);
        Vec2::new(x as f32 / s, y as f32 / s)
    }
}
