use super::{Rect, Vec2};

/// Center `inner` inside `outer`, letterboxed. If `inner` is larger than `outer`
/// on an axis, scale uniformly to fit.
pub fn letterbox_rect(outer: Rect, inner: Vec2) -> Rect {
    let ox = outer.size.x.max(1.0);
    let oy = outer.size.y.max(1.0);
    let scale = (ox / inner.x.max(1.0)).min(oy / inner.y.max(1.0)).min(1.0);
    let w = inner.x * scale;
    let h = inner.y * scale;
    let x = outer.origin.x + (ox - w) * 0.5;
    let y = outer.origin.y + (oy - h) * 0.5;
    Rect::from_xywh(x, y, w, h)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phone_letterbox_centered() {
        let outer = Rect::from_xywh(0.0, 0.0, 1440.0, 900.0);
        let r = letterbox_rect(outer, Vec2::new(390.0, 844.0));
        assert!((r.size.x - 390.0).abs() < 0.1);
        assert!((r.size.y - 844.0).abs() < 0.1);
        assert!((r.origin.x - (1440.0 - 390.0) * 0.5).abs() < 0.1);
    }
}
