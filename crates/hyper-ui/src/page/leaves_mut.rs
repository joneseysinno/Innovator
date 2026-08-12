use super::{PageNode, PageTree};

impl PageTree {
    /// Mutable pages in Binding order.
    pub fn leaves_mut(&mut self) -> Vec<&mut PageNode> {
        self.pages.iter_mut().collect()
    }
}
