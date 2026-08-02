use super::{PageId, PageSeamId, PageSide, PageTree};

impl PageTree {
    /// Remove the Split at `seam_id` and promote `keep`. Returns the retired PageIds.
    pub fn merge(&mut self, seam_id: PageSeamId, keep: PageSide) -> Option<Vec<PageId>> {
        let mut idx = 0u32;
        self.merge_inner(seam_id.0, keep, &mut idx)
    }

    fn merge_inner(
        &mut self,
        target: u32,
        keep: PageSide,
        idx: &mut u32,
    ) -> Option<Vec<PageId>> {
        match self {
            Self::Leaf(_) => None,
            Self::Split { first, second, .. } => {
                if *idx == target {
                    let retired = match keep {
                        PageSide::First => second.collect_page_ids(),
                        PageSide::Second => first.collect_page_ids(),
                    };
                    let survivor = match keep {
                        PageSide::First => std::mem::replace(
                            first.as_mut(),
                            PageTree::Leaf(super::PageNode::empty(PageId(0))),
                        ),
                        PageSide::Second => std::mem::replace(
                            second.as_mut(),
                            PageTree::Leaf(super::PageNode::empty(PageId(0))),
                        ),
                    };
                    *self = survivor;
                    return Some(retired);
                }
                *idx += 1;
                first
                    .merge_inner(target, keep, idx)
                    .or_else(|| second.merge_inner(target, keep, idx))
            }
        }
    }

    fn collect_page_ids(&self) -> Vec<PageId> {
        match self {
            Self::Leaf(page) => vec![page.id],
            Self::Split { first, second, .. } => {
                let mut ids = first.collect_page_ids();
                ids.extend(second.collect_page_ids());
                ids
            }
        }
    }
}
