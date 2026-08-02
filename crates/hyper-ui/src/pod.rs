//! Flat ordered pod list — collapsible content slots within a page.

mod apply_divider_drag;
mod collapse;
mod divider;
mod id;
mod layout;
mod list;
mod pod;

pub use divider::{PodDivider, PodDividerRenderer};
pub use id::PodId;
pub use list::PodList;
pub use pod::{Pod, COLLAPSED_HEIGHT};
