use super::AppShell;
use crate::results::ensure_results_space;
use crate::walls::ensure_walls_space;
use crate::workspace::home::HomeWorkspace;
use crate::workspace::instance::WorkspaceInstance;
use crate::workspace::screen_class::ScreenClass;
use crate::workspace::tab_strip::build_tab_strip;
use crate::workspace::workspace_id::WorkspaceId;
use hyper_ui::Rect;
use infinite_db::InfiniteDb;
use std::collections::HashMap;

impl AppShell {
    pub fn new(mut db: InfiniteDb) -> Self {
        ensure_walls_space(&mut db);
        ensure_results_space(&mut db);

        let (tx, signal_rx) = flume::unbounded();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_secs(2));
            let _ = tx.send("engine online".into());
        });

        let first = HomeWorkspace::new(WorkspaceId(1));
        let active_id = first.tab.id;
        let workspaces = vec![WorkspaceInstance::Home(first)];
        let tabs: Vec<_> = workspaces.iter().map(|w| w.tab().clone()).collect();
        let tab_strip = build_tab_strip(&tabs, active_id);

        Self {
            window: None,
            renderer: None,
            db,
            screen_class: ScreenClass::Desktop,
            workspaces,
            active_id,
            next_workspace_id: 2,
            tab_strip,
            signal_rx,
            window_area: Rect::from_xywh(0.0, 0.0, 1280.0, 800.0),
            // Home has no header — pages start below tab strip (~28px).
            pages_area: Rect::from_xywh(0.0, 28.0, 1280.0, 772.0),
            has_header: false,
            pending_context_menu: None,
            context_menu_triggers: HashMap::new(),
        }
    }
}
