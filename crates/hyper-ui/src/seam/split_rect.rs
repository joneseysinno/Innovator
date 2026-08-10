use crate::geom::Rect;

use super::SeamDirection;

#[allow(dead_code)]
pub(crate) fn split_rect(area: Rect, direction: SeamDirection, ratio: f32) -> (Rect, Rect) {
    match direction {
        SeamDirection::Vertical => {
            let w1 = area.size.x * ratio;
            (
                Rect::from_xywh(area.origin.x, area.origin.y, w1, area.size.y),
                Rect::from_xywh(
                    area.origin.x + w1,
                    area.origin.y,
                    area.size.x - w1,
                    area.size.y,
                ),
            )
        }
        SeamDirection::Horizontal => {
            let h1 = area.size.y * ratio;
            (
                Rect::from_xywh(area.origin.x, area.origin.y, area.size.x, h1),
                Rect::from_xywh(
                    area.origin.x,
                    area.origin.y + h1,
                    area.size.x,
                    area.size.y - h1,
                ),
            )
        }
    }
}
