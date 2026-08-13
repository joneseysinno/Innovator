use super::AppShell;
use crate::auth::session::Session;
use crate::devtools::PreviewPreset;
use crate::results::ensure_results_space;
use crate::walls::{ensure_walls_space, load_walls};
use crate::workspace::graph_containers::{
    bind_workspaces_to_root, insert_root, wire_home_nav_signals,
};
use crate::workspace::persist::{self, LAYOUT_PATH};
use crate::workspace::seed;
use crate::workspace::tab_strip::build_tab_strip;
use crate::workspace::workspace::Workspace;
use hyper_ui::container::Visibility;
use hyper_ui::{FocusPath, InputClass, PageNode, Rect, ResolveReport, SizeClass};
use hypernode::Graph;
use infinite_db::InfiniteDb;
use std::collections::HashMap;

impl AppShell {
    pub fn new(mut db: InfiniteDb, session: Session) -> Self {
        ensure_walls_space(&mut db);
        ensure_results_space(&mut db);

        let (tx, signal_rx) = flume::unbounded();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_secs(2));
            let _ = tx.send("engine online".into());
        });

        let mut graph = Graph::new();
        load_walls(&mut db, &mut graph);

        // Root before any workspace so every Binding forest hangs from depth 0.
        let root_id = insert_root(&mut graph);

        // Restore from layout.json when present; otherwise first-run seeds.
        let (workspaces, next_workspace_id) = match persist::load_layout(LAYOUT_PATH) {
            Some(session) => persist::restore_workspaces(&session, &mut db, &mut graph),
            None => {
                let ws = Workspace::from_seeds(&mut db, &mut graph);
                let next = ws.len() as u64 + 1;
                (ws, next)
            }
        };
        let ws_nodes: Vec<_> = workspaces.iter().map(|w| w.node_id).collect();
        bind_workspaces_to_root(&mut graph, root_id, &ws_nodes);

        if let Some(home) = workspaces.iter().find(|w| w.open_id == "home") {
            // LAUNCHABLE + Graph (F11) — every OpenWorkspace target needs a Signal.
            let nav_seeds: Vec<&seed::WorkspaceSeed> = seed::LAUNCHABLE
                .iter()
                .chain(std::iter::once(&seed::DEVTOOLS_GRAPH))
                .collect();
            let targets: Vec<_> = nav_seeds
                .iter()
                .filter_map(|s| {
                    workspaces
                        .iter()
                        .find(|w| w.open_id == s.open_id)
                        .map(|w| (w.node_id, s.open_id))
                })
                .collect();
            wire_home_nav_signals(&mut graph, home.node_id, &targets);
        }

        let active = workspaces
            .iter()
            .find(|w| w.state.intent == Visibility::Shown)
            .or_else(|| workspaces.first())
            .expect("at least one workspace");
        let active_id = active.id();
        let mut focus_chain = vec![active.state.id];
        if let Some(s) = active.structural() {
            focus_chain.push(PageNode::container_id(s.focused_page));
        } else if let Some(p) = active.placeholder() {
            focus_chain.push(PageNode::container_id(p.focused_page));
        } else if let Some(h) = active.home_ws() {
            focus_chain.push(PageNode::container_id(h.focused_page));
        }
        let focus = FocusPath::new(focus_chain);
        let tabs: Vec<_> = workspaces.iter().map(|w| w.tab()).collect();
        let tab_strip = build_tab_strip(&tabs, active_id);

        Self {
            window: None,
            renderer: None,
            db,
            session,
            graph,
            root_id,
            workspaces,
            focus,
            next_workspace_id,
            tab_strip,
            signal_rx,
            window_area: Rect::from_xywh(0.0, 0.0, 1280.0, 800.0),
            pages_area: Rect::from_xywh(0.0, 32.0, 1280.0, 768.0),
            has_header: false,
            pending_context_menu: None,
            pending_template_menu: None,
            context_menu_triggers: HashMap::new(),
            scale_factor: 1.0,
            physical_width: 1280,
            physical_height: 800,
            size_class: SizeClass::Expanded,
            input_class: InputClass::Pointer,
            preview: PreviewPreset::Native,
            overlay_open: false,
            last_report: ResolveReport {
                demotions: Vec::new(),
                promotions: Vec::new(),
                scroll_extent: 0.0,
                underflowed: false,
            },
        }
    }

    /// Persist container tree + overrides (never resolved/rect).
    pub fn persist_layout(&self) {
        let _ = persist::save_layout(LAYOUT_PATH, self);
    }
}
