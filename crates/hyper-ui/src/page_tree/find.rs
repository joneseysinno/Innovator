use super::{PageId, PageNode, PageTree};

impl PageTree {
    pub fn find(&self, id: PageId) -> Option<&PageNode> {
        match self {
            Self::Leaf(page) if page.id == id => Some(page),
            Self::Leaf(_) => None,
            Self::Split { first, second, .. } => first.find(id).or_else(|| second.find(id)),
        }
    }

    pub fn find_mut(&mut self, id: PageId) -> Option<&mut PageNode> {
        match self {
            Self::Leaf(page) if page.id == id => Some(page),
            Self::Leaf(_) => None,
            Self::Split { first, second, .. } => first
                .find_mut(id)
                .or_else(|| second.find_mut(id)),
        }
    }

    /// Leaves in pre-order (first, then second).
    pub fn leaves(&self) -> Vec<&PageNode> {
        let mut out = Vec::new();
        self.collect_leaves(&mut out);
        out
    }

    fn collect_leaves<'a>(&'a self, out: &mut Vec<&'a PageNode>) {
        match self {
            Self::Leaf(page) => out.push(page),
            Self::Split { first, second, .. } => {
                first.collect_leaves(out);
                second.collect_leaves(out);
            }
        }
    }
}
