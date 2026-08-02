use super::{PageNode, PageTree};

impl PageTree {
    /// Mutable leaves in pre-order (first, then second).
    pub fn leaves_mut(&mut self) -> Vec<&mut PageNode> {
        let mut out = Vec::new();
        self.collect_leaves_mut(&mut out);
        out
    }

    fn collect_leaves_mut<'a>(&'a mut self, out: &mut Vec<&'a mut PageNode>) {
        match self {
            Self::Leaf(page) => out.push(page),
            Self::Split { first, second, .. } => {
                first.collect_leaves_mut(out);
                second.collect_leaves_mut(out);
            }
        }
    }
}
