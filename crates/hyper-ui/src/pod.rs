//! Flat ordered pod list — collapsible content slots within a page.

mod apply_divider_drag;
mod collapse;
mod component_id;
mod divider;
mod icon_rail;
mod id;
mod layout;
mod list;
mod pod;
mod shell;

#[cfg(test)]
mod layout_tests;

pub use component_id::ComponentId;
pub use collapse::{default_collapse, resolved_collapse};
pub use divider::{PodDivider, PodDividerRenderer};
pub use icon_rail::{build_pod_icon_rail, default_icon_rail_config, effective_icon_rail};
pub use id::PodId;
pub use list::PodList;
pub use pod::{Pod, COLLAPSED_HEIGHT};
pub use shell::{
    pod_nav_icons, pod_shell, wrap_pod_column, PodShell, POD_FRAME_BORDER, POD_FRAME_BORDER_WIDTH,
    POD_FRAME_FILL, POD_STACK_GAP, POD_TITLE_FILL,
};
