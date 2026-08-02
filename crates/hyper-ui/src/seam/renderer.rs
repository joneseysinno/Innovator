mod clear;
mod cursor_icon;
mod default;
mod draw_commands;
pub(crate) mod handle_event;
mod new;
mod rebuild_from_page_tree;

use crate::geom::Vec2;

use super::SeamDrawCmd;

pub struct SeamRenderer {
    pub seams: Vec<SeamDrawCmd>,
    drag: Option<(usize, Vec2)>,
    last_click: Option<(usize, std::time::Instant)>,
}

impl SeamRenderer {
    pub fn is_dragging(&self) -> bool {
        self.drag.is_some()
    }
}
