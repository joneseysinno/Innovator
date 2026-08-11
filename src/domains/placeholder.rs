//! Generic placeholder workspace — PageTree + stub IO (no domain state).

pub mod build_content;
pub mod from_seed;
pub mod layout;
pub mod stub_io;

use hyper_ui::{Overrides, PageId, PageTree, PodId};
use std::collections::HashMap;

/// Labels for stub IO stacked inside a pod: `(page, pod) → [io labels]`.
pub type StubIoMap = HashMap<(PageId, PodId), Vec<String>>;

pub struct PlaceholderWorkspace {
    pub open_id: &'static str,
    pub page_tree: PageTree,
    pub stub_ios: StubIoMap,
    pub page_overrides: Overrides,
    pub focused_page: PageId,
    pub page_viewport_ids: HashMap<PageId, hyper_ui::ParticleId>,
    pub page_show_triggers: HashMap<hyper_ui::ParticleId, PageId>,
    pub pod_collapse_triggers: HashMap<hyper_ui::ParticleId, PodId>,
    pub icon_rail_triggers: HashMap<hyper_ui::ParticleId, (PageId, PodId)>,
}
