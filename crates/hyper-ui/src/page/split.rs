use crate::seam::SeamDirection;

use super::{PageId, PageNode, PageSide, PageTree};

impl PageTree {
    /// Split the leaf with `page_id` into the original page + a new empty sibling.
    ///
    /// Returns the new page's id, or `None` if `page_id` was not found.
    pub fn split_page(
        &mut self,
        page_id: PageId,
        direction: SeamDirection,
        new_id: PageId,
    ) -> Option<PageId> {
        match self {
            Self::Leaf(page) if page.id == page_id => {
                let original = std::mem::replace(page, PageNode::empty(PageId(0)));
                *self = PageTree::Split {
                    direction,
                    first: Box::new(PageTree::Leaf(original)),
                    second: Box::new(PageTree::Leaf(PageNode::empty(new_id))),
                };
                Some(new_id)
            }
            Self::Leaf(_) => None,
            Self::Split { first, second, .. } => first
                .split_page(page_id, direction, new_id)
                .or_else(|| second.split_page(page_id, direction, new_id)),
        }
    }

    /// Split the leaf on `side` of the Split identified by pre-order `seam_index`.
    pub fn split_at_seam(
        &mut self,
        seam_index: u32,
        side: PageSide,
        direction: SeamDirection,
        new_id: PageId,
    ) -> Option<PageId> {
        let mut idx = 0u32;
        self.split_at_seam_inner(seam_index, side, direction, new_id, &mut idx)
    }

    fn split_at_seam_inner(
        &mut self,
        target: u32,
        side: PageSide,
        direction: SeamDirection,
        new_id: PageId,
        idx: &mut u32,
    ) -> Option<PageId> {
        match self {
            Self::Leaf(_) => None,
            Self::Split { first, second, .. } => {
                if *idx == target {
                    let child = match side {
                        PageSide::First => first.as_mut(),
                        PageSide::Second => second.as_mut(),
                    };
                    return match child {
                        PageTree::Leaf(page) => {
                            let page_id = page.id;
                            child.split_page(page_id, direction, new_id)
                        }
                        PageTree::Split { .. } => {
                            // Split the first leaf under this child.
                            let page_id = child.leaves().first().map(|p| p.id)?;
                            child.split_page(page_id, direction, new_id)
                        }
                    };
                }
                *idx += 1;
                first
                    .split_at_seam_inner(target, side, direction, new_id, idx)
                    .or_else(|| second.split_at_seam_inner(target, side, direction, new_id, idx))
            }
        }
    }
}
