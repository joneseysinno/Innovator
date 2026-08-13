//! Home dashboard workspace — entry point to other workspaces.

use hyper_ui::{Overrides, PageId, PageTree, ParticleId, PodId};
use hypernode::NodeId;
use std::collections::HashMap;

pub struct HomeWorkspace {
    pub page_tree: PageTree,
    pub page_overrides: Overrides,
    pub focused_page: PageId,
    pub page_viewport_ids: HashMap<PageId, ParticleId>,
    pub page_show_triggers: HashMap<ParticleId, PageId>,
    pub pod_collapse_triggers: HashMap<ParticleId, PodId>,
    pub icon_rail_triggers: HashMap<ParticleId, (PageId, PodId)>,
    /// Dashboard launcher triggers → workspace open_id (resolved via Signal walk).
    pub launcher_triggers: HashMap<ParticleId, &'static str>,
    /// Graph UIView identity for this workspace container.
    pub node_id: NodeId,
}
