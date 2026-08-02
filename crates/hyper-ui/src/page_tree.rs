//! Page-level binary split tree — spatial containers above pods.

mod content_rect;
mod find;
mod icon_rail;
mod leaf_rects;
mod merge;
mod page_header;
mod page_id;
mod page_node;
mod page_seam_id;
mod page_side;
mod set_ratio;
mod split;
mod tree;

pub use icon_rail::{IconRailConfig, IconRailSide};
pub use page_header::{PageHeaderConfig, PageHeaderSlots};
pub use page_id::PageId;
pub use page_node::PageNode;
pub use page_seam_id::PageSeamId;
pub use page_side::PageSide;
pub use tree::PageTree;
