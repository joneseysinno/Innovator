use crate::workspace::header::HEADER_HEIGHT;
use crate::workspace::tab_strip::TAB_STRIP_HEIGHT;
use hyper_ui::Rect;

/// Split window into tab strip, optional header, and pages regions.
pub fn layout_areas(window: Rect, has_header: bool) -> (Rect, Option<Rect>, Rect) {
    let tab_h = TAB_STRIP_HEIGHT.min(window.size.y);
    let tabs = Rect::from_xywh(window.origin.x, window.origin.y, window.size.x, tab_h);

    let mut y = window.origin.y + tab_h;
    let mut remaining = (window.size.y - tab_h).max(0.0);

    let header = if has_header && remaining > 0.0 {
        let h = HEADER_HEIGHT.min(remaining);
        let rect = Rect::from_xywh(window.origin.x, y, window.size.x, h);
        y += h;
        remaining = (remaining - h).max(0.0);
        Some(rect)
    } else {
        None
    };

    let pages = Rect::from_xywh(window.origin.x, y, window.size.x, remaining);
    (tabs, header, pages)
}
