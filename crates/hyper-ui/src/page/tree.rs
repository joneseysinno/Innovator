use crate::seam::SeamDirection;

use super::PageNode;

/// Binary split tree for page-level workspace layout.
///
/// Topology only — sizing comes from [`crate::layout::resolve`], not stored ratios.
#[derive(Debug, Clone)]
pub enum PageTree {
    Leaf(PageNode),
    Split {
        direction: SeamDirection,
        first: Box<PageTree>,
        second: Box<PageTree>,
    },
}
