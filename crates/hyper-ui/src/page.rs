//! Page-level binary split tree — spatial containers above pods.

mod content_rect;
mod find;
mod header;
mod icon_rail;
mod id;
mod leaf_rects;
mod leaves_mut;
mod merge;
mod node;
mod seam_id;
mod set_ratio;
mod side;
mod split;
mod tree;

pub use header::{PageHeaderConfig, PageHeaderSlots};
pub use icon_rail::{IconRailConfig, IconRailSide};
pub use id::PageId;
pub use node::PageNode;
pub use seam_id::PageSeamId;
pub use side::PageSide;
pub use tree::PageTree;
