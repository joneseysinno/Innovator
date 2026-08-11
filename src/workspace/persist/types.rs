//! Persisted DTOs — only intent/authored fields. No resolved, no rect.

use serde::{Deserialize, Serialize};

pub const PERSIST_VERSION: u32 = 1;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedSession {
    pub version: u32,
    pub next_workspace_id: u64,
    pub workspaces: Vec<PersistedWorkspace>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedWorkspace {
    pub open_id: String,
    pub state: PersistedContainer,
    pub focused_page: Option<u32>,
    pub page_tree: Option<PersistedPageTree>,
    pub page_overrides: PersistedOverrides,
    /// Structural IO assignment: page → [(pod, IoKind as string)].
    pub page_ios: Option<Vec<(u32, Vec<(u32, String)>)>>,
    pub next_page_id: Option<u32>,
    /// Placeholder stub labels: ((page, pod), [labels]).
    pub stub_ios: Option<Vec<((u32, u32), Vec<String>)>>,
}

/// `{id, label, icon, intent, extent}` — never resolved/rect.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedContainer {
    pub id: u64,
    pub label: String,
    pub icon: String,
    pub intent: PersistedVisibility,
    pub extent: PersistedExtent,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PersistedVisibility {
    Shown,
    Collapsed,
    Hidden,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct PersistedExtent {
    pub min: f32,
    pub ideal: f32,
    pub weight: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedOverrides {
    pub entries: Vec<PersistedOverrideEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedOverrideEntry {
    pub id: u64,
    pub class: PersistedSizeClass,
    pub fraction: f32,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PersistedSizeClass {
    Compact,
    Medium,
    Expanded,
    Large,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PersistedPageTree {
    Leaf(PersistedPageNode),
    Split {
        direction: PersistedSeamDirection,
        first: Box<PersistedPageTree>,
        second: Box<PersistedPageTree>,
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum PersistedSeamDirection {
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedPageNode {
    pub id: u32,
    pub state: PersistedContainer,
    pub pods: PersistedPodList,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedPodList {
    pub pods: Vec<PersistedPod>,
    pub gap: f32,
    pub overrides: PersistedOverrides,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersistedPod {
    pub id: u32,
    pub state: PersistedContainer,
    pub title: String,
    pub min_height: f32,
    pub height: f32,
    /// Optional page-rail glyph; absent in older saves.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub nav_icon: Option<String>,
}
