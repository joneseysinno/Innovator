use super::PodDividerRenderer;

impl PodDividerRenderer {
    pub fn new() -> Self {
        Self {
            dividers: Vec::new(),
            area_heights: Vec::new(),
            drag: None,
            last_click: None,
        }
    }

    pub fn clear(&mut self) {
        self.dividers.clear();
        self.area_heights.clear();
        self.drag = None;
        self.last_click = None;
    }

    pub fn is_dragging(&self) -> bool {
        self.drag.is_some()
    }
}

impl Default for PodDividerRenderer {
    fn default() -> Self {
        Self::new()
    }
}
