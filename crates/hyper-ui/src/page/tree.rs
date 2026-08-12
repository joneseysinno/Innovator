use super::PageNode;

/// Ordered page containers for workspace layout.
///
/// Sizing is resolved by [`crate::layout::resolve`]. The order mirrors the
/// workspace's Binding children in the composed view graph.
#[derive(Debug, Clone, Default)]
pub struct PageTree {
    pub pages: Vec<PageNode>,
}
