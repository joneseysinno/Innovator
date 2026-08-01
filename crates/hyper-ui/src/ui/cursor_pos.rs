use crate::geom::Vec2;
use crate::input::InputRouter;

pub fn cursor_pos(router: &InputRouter) -> Vec2 {
    router.cursor
}
