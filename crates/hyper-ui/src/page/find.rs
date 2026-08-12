use super::{PageId, PageNode, PageTree};

impl PageTree {
    pub fn find(&self, id: PageId) -> Option<&PageNode> {
        self.pages.iter().find(|page| page.id == id)
    }

    pub fn find_mut(&mut self, id: PageId) -> Option<&mut PageNode> {
        self.pages.iter_mut().find(|page| page.id == id)
    }

    /// Pages in Binding order.
    pub fn leaves(&self) -> Vec<&PageNode> {
        self.pages.iter().collect()
    }
}
