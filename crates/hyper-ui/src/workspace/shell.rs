use crate::geom::Rect;
use crate::page::{PageId, PageTree};

/// Generic workspace container — no application content knowledge.
#[derive(Debug, Clone)]
pub struct WorkspaceShell {
    pub pages: PageTree,
    pub area: Rect,
    pub next_page_id: u32,
}

impl WorkspaceShell {
    pub fn new(area: Rect) -> Self {
        Self {
            pages: PageTree::Leaf(crate::page::PageNode::empty(PageId(0))),
            area,
            next_page_id: 1,
        }
    }

    pub fn with_pages(area: Rect, pages: PageTree, next_page_id: u32) -> Self {
        Self {
            pages,
            area,
            next_page_id,
        }
    }

    pub fn next_id(&mut self) -> PageId {
        let id = PageId(self.next_page_id);
        self.next_page_id = self.next_page_id.saturating_add(1);
        id
    }
}
