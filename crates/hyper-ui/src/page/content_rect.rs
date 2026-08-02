use crate::geom::Rect;

use super::{IconRailSide, PageNode};

impl PageNode {
    /// Page rect minus optional header and icon rail.
    pub fn content_rect(&self, page_rect: Rect) -> Rect {
        let mut origin = page_rect.origin;
        let mut size = page_rect.size;

        if let Some(header) = &self.header {
            let h = header.height.min(size.y);
            origin.y += h;
            size.y = (size.y - h).max(0.0);
        }

        if let Some(rail) = &self.icon_rail {
            let w = rail.width.min(size.x);
            match rail.side {
                IconRailSide::Left => {
                    origin.x += w;
                    size.x = (size.x - w).max(0.0);
                }
                IconRailSide::Right => {
                    size.x = (size.x - w).max(0.0);
                }
            }
        }

        Rect::new(origin, size)
    }

    /// Header strip at the top of the page, if configured.
    pub fn header_rect(&self, page_rect: Rect) -> Option<Rect> {
        let header = self.header.as_ref()?;
        let h = header.height.min(page_rect.size.y);
        Some(Rect::from_xywh(
            page_rect.origin.x,
            page_rect.origin.y,
            page_rect.size.x,
            h,
        ))
    }

    /// Icon rail strip beside the content area, if configured.
    pub fn icon_rail_rect(&self, page_rect: Rect) -> Option<Rect> {
        let rail = self.icon_rail.as_ref()?;
        let mut origin = page_rect.origin;
        let mut size = page_rect.size;

        if let Some(header) = &self.header {
            let h = header.height.min(size.y);
            origin.y += h;
            size.y = (size.y - h).max(0.0);
        }

        let w = rail.width.min(size.x);
        match rail.side {
            IconRailSide::Left => Some(Rect::from_xywh(origin.x, origin.y, w, size.y)),
            IconRailSide::Right => Some(Rect::from_xywh(
                origin.x + size.x - w,
                origin.y,
                w,
                size.y,
            )),
        }
    }
}
