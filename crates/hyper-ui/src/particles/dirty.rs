use crate::particles::ParticleId;

/// Generational dirty flags for partial re-render.
#[derive(Debug, Default)]
pub struct DirtyFlags {
    pub layout: Vec<ParticleId>,
    pub paint: Vec<ParticleId>,
    pub text: Vec<ParticleId>,
    pub layout_all: bool,
    pub paint_all: bool,
    pub text_all: bool,
}

impl DirtyFlags {
    pub fn clear(&mut self) {
        self.layout.clear();
        self.paint.clear();
        self.text.clear();
        self.layout_all = false;
        self.paint_all = false;
        self.text_all = false;
    }

    pub fn needs_layout(&self) -> bool {
        self.layout_all || !self.layout.is_empty()
    }

    pub fn needs_paint(&self) -> bool {
        self.paint_all || !self.paint.is_empty() || self.needs_layout()
    }
}
