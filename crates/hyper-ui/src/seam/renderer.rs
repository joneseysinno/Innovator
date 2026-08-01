mod cursor_icon;
mod default;
mod draw_commands;
mod handle_event;
mod new;
mod rebuild_from_pods;

use crate::geom::Vec2;

use super::SeamDrawCmd;

pub struct SeamRenderer {
    pub seams: Vec<SeamDrawCmd>,
    drag: Option<(usize, Vec2)>,
    last_click: Option<(usize, std::time::Instant)>,
}
