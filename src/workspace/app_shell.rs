//! App-level window host: tab strip + active workspace content.

pub mod active;
pub mod add_workspace;
pub mod application_handler;
pub mod build_tree;
pub mod handle_analysis_action;
pub mod handle_app_signal;
pub mod handle_builder_field;
pub mod handle_page_signal;
pub mod handle_value_changed;
pub mod handle_workspace_signal;
pub mod layout_areas;
pub mod new;
pub mod open_workspace;
pub mod page_context_menu;
pub mod page_template_menu;
pub mod rebuild_active;
pub mod rebuild_seams;
pub mod resumed;
pub mod select_workspace;
pub mod sync_chrome_layouts;
pub mod update_focus;
pub mod viewport;
pub mod window_event;

use crate::auth::session::Session;
use crate::devtools::PreviewPreset;
use crate::domains::structural::PageSignal;
use crate::workspace::app_shell::page_context_menu::PageContextMenu;
use crate::workspace::app_shell::page_template_menu::PageTemplateMenu;
use crate::workspace::tab_strip::TabStripIO;
use crate::workspace::workspace::Workspace;
use hyper_ui::{
    FocusPath, HyperRenderer, InputClass, ParticleId, Rect, ResolveReport, SizeClass,
};
use hypernode::Graph;
use infinite_db::InfiniteDb;
use std::collections::HashMap;
use std::sync::Arc;
use winit::window::Window;

/// Window host — Blender-style workspace tabs + active workspace body.
pub struct AppShell {
    pub(crate) window: Option<Arc<Window>>,
    pub(crate) renderer: Option<HyperRenderer>,
    pub db: InfiniteDb,
    pub session: Session,
    /// Composed, permission-scoped graph view (today: everything loaded).
    /// Access only via [`Self::composed_view`] / [`Self::composed_view_mut`].
    graph: Graph,
    pub workspaces: Vec<Workspace>,
    /// Workspace → page → pod focus chain. Pointer-down only; never hover.
    pub focus: FocusPath,
    pub(crate) next_workspace_id: u64,
    pub(crate) tab_strip: TabStripIO,
    pub(crate) signal_rx: flume::Receiver<String>,
    /// Full window in logical pixels (origin 0,0).
    pub(crate) window_area: Rect,
    /// Content layout rect (letterbox when previewing) in logical pixels.
    pub(crate) pages_area: Rect,
    pub(crate) has_header: bool,
    pub pending_context_menu: Option<PageContextMenu>,
    pub pending_template_menu: Option<PageTemplateMenu>,
    pub context_menu_triggers: HashMap<ParticleId, PageSignal>,
    /// DPI scale of the current monitor.
    pub scale_factor: f32,
    pub physical_width: u32,
    pub physical_height: u32,
    /// Hysteretic size class from the layout viewport width.
    pub size_class: SizeClass,
    pub input_class: InputClass,
    pub preview: PreviewPreset,
    pub overlay_open: bool,
    pub last_report: ResolveReport,
}

impl AppShell {
    /// Current composed view of the graph.
    ///
    /// Today returns everything loaded. Later: Grant-filtered union of
    /// reachable scopes — same signature, additive filtering underneath.
    pub fn composed_view(&self) -> &Graph {
        &self.graph
    }

    /// Mutable composed view — see [`Self::composed_view`].
    pub fn composed_view_mut(&mut self) -> &mut Graph {
        &mut self.graph
    }
}
