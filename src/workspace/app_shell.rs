//! App-level window host: tab strip + active workspace content.

pub mod active;
pub mod add_workspace;
pub mod application_handler;
pub mod build_tree;
pub mod handle_analysis_action;
pub mod handle_app_signal;
pub mod handle_builder_field;
pub mod handle_value_changed;
pub mod handle_workspace_signal;
pub mod layout_areas;
pub mod new;
pub mod open_workspace;
pub mod rebuild_active;
pub mod rebuild_seams;
pub mod resumed;
pub mod select_workspace;
pub mod sync_chrome_layouts;
pub mod sync_page_layouts;
pub mod window_event;

use crate::workspace::instance::WorkspaceInstance;
use crate::workspace::screen_class::ScreenClass;
use crate::workspace::tab_strip::TabStripIO;
use crate::workspace::workspace_id::WorkspaceId;
use hyper_ui::{HyperRenderer, Rect};
use infinite_db::InfiniteDb;
use std::sync::Arc;
use winit::window::Window;

/// Window host — Blender-style workspace tabs + active workspace body.
pub struct AppShell {
    pub(crate) window: Option<Arc<Window>>,
    pub(crate) renderer: Option<HyperRenderer>,
    pub db: InfiniteDb,
    pub screen_class: ScreenClass,
    pub workspaces: Vec<WorkspaceInstance>,
    pub active_id: WorkspaceId,
    pub(crate) next_workspace_id: u64,
    pub(crate) tab_strip: TabStripIO,
    pub(crate) signal_rx: flume::Receiver<String>,
    pub(crate) window_area: Rect,
    pub(crate) pages_area: Rect,
    pub(crate) has_header: bool,
}
