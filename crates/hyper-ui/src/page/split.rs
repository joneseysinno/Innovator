use crate::seam::SeamDirection;

use super::{PageId, PageNode, PageSide, PageTree};

impl PageTree {
    /// Insert an empty sibling immediately after `page_id`.
    ///
    /// Returns the new page's id, or `None` if `page_id` was not found.
    pub fn split_page(
        &mut self,
        page_id: PageId,
        _direction: SeamDirection,
        new_id: PageId,
    ) -> Option<PageId> {
        let index = self.pages.iter().position(|page| page.id == page_id)?;
        self.pages.insert(index + 1, PageNode::empty(new_id));
        Some(new_id)
    }

    /// Split the page on `side` of the seam between adjacent shown pages.
    pub fn split_at_seam(
        &mut self,
        seam_index: u32,
        side: PageSide,
        direction: SeamDirection,
        new_id: PageId,
    ) -> Option<PageId> {
        let shown: Vec<_> = self
            .pages
            .iter()
            .filter(|page| page.state.resolved() == crate::container::Visibility::Shown)
            .map(|page| page.id)
            .collect();
        let first = *shown.get(seam_index as usize)?;
        let page_id = match side {
            PageSide::First => first,
            PageSide::Second => *shown.get(seam_index as usize + 1)?,
        };
        self.split_page(page_id, direction, new_id)
    }
}
