use crate::seam::SeamDirection;

use super::PageNode;

/// Binary split tree for page-level workspace layout.
#[derive(Debug, Clone)]
pub enum PageTree {
    Leaf(PageNode),
    Split {
        direction: SeamDirection,
        ratio: f32,
        first: Box<PageTree>,
        second: Box<PageTree>,
    },
}
