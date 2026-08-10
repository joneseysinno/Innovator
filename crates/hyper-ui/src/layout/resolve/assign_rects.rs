use crate::container::{ContainerState, Visibility};
use crate::geom::{Rect, Vec2};

/// Arrangement axis for sequential rect assignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Axis {
    Horizontal,
    Vertical,
}

/// Step 8 — pack allocated sizes into rects along `axis`.
pub(super) fn assign(
    children: &mut [ContainerState],
    sizes: &[f32],
    cross_extent: f32,
    axis: Axis,
    origin: Vec2,
) {
    let mut cursor = 0.0_f32;
    for (i, child) in children.iter_mut().enumerate() {
        let extent = sizes[i];
        if child.resolved() == Visibility::Hidden || extent <= 0.0 {
            child.set_rect(Rect::default());
            continue;
        }
        let rect = match axis {
            Axis::Horizontal => Rect::from_xywh(origin.x + cursor, origin.y, extent, cross_extent),
            Axis::Vertical => Rect::from_xywh(origin.x, origin.y + cursor, cross_extent, extent),
        };
        child.set_rect(rect);
        cursor += extent;
    }
}
