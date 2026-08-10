use crate::workspace::header::HEADER_HEIGHT;
use crate::workspace::tab_strip::TAB_STRIP_HEIGHT;
use hyper_ui::Rect;

/// Split the layout window into app tab strip, workspace host, and page region.
///
/// - `tabs` — app chrome only
/// - `workspace` — fills everything under the tab strip (header + pages)
/// - `header` — workspace chrome inside the host (when present)
/// - `pages` — page region inside the host (workspace minus header)
pub fn layout_areas(window: Rect, has_header: bool) -> LayoutAreas {
    let tab_h = TAB_STRIP_HEIGHT.min(window.size.y);
    let tabs = Rect::from_xywh(window.origin.x, window.origin.y, window.size.x, tab_h);

    let ws_y = window.origin.y + tab_h;
    let ws_h = (window.size.y - tab_h).max(0.0);
    let workspace = Rect::from_xywh(window.origin.x, ws_y, window.size.x, ws_h);

    let mut y = ws_y;
    let mut remaining = ws_h;

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
    LayoutAreas {
        tabs,
        workspace,
        header,
        pages,
    }
}

#[derive(Debug, Clone, Copy)]
pub struct LayoutAreas {
    pub tabs: Rect,
    /// Window under the app tab strip — the active workspace must fill this.
    pub workspace: Rect,
    pub header: Option<Rect>,
    /// Page region inside the workspace host.
    pub pages: Rect,
}
