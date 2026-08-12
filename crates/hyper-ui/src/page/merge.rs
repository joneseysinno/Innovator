use super::{PageId, PageSeamId, PageSide, PageTree};

impl PageTree {
    /// Remove one of the pages adjacent to `seam_id`. Returns its retired id.
    pub fn merge(&mut self, seam_id: PageSeamId, keep: PageSide) -> Option<Vec<PageId>> {
        let shown: Vec<_> = self
            .pages
            .iter()
            .filter(|page| page.state.resolved() == crate::container::Visibility::Shown)
            .map(|page| page.id)
            .collect();
        let seam = seam_id.0 as usize;
        let retired_id = match keep {
            PageSide::First => *shown.get(seam + 1)?,
            PageSide::Second => *shown.get(seam)?,
        };
        let index = self.pages.iter().position(|page| page.id == retired_id)?;
        self.pages.remove(index);
        Some(vec![retired_id])
    }
}
