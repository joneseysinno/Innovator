mod clear;
mod cursor_icon;
mod default;
mod draw_commands;
pub(crate) mod handle_event;
mod new;
mod rebuild_from_page_tree;
mod rebuild_from_pods;

use crate::geom::Vec2;
use crate::page_tree::PageId;

use super::SeamDrawCmd;

pub struct SeamRenderer {
    pub seams: Vec<SeamDrawCmd>,
    /// Parallel to `seams` when rebuilding concatenated pod seams across pages.
    pub pod_owners: Vec<(PageId, usize)>,
    drag: Option<(usize, Vec2)>,
    last_click: Option<(usize, std::time::Instant)>,
}

impl SeamRenderer {
    pub fn is_dragging(&self) -> bool {
        self.drag.is_some()
    }
}
