use super::AppShell;
use crate::results::ensure_results_space;
use crate::walls::ensure_walls_space;
use crate::workspace::analysis::AnalysisWorkspace;
use crate::workspace::instance::WorkspaceInstance;
use crate::workspace::screen_class::ScreenClass;
use crate::workspace::tab_strip::build_tab_strip;
use crate::workspace::workspace_id::WorkspaceId;
use hyper_ui::Rect;
use infinite_db::InfiniteDb;

impl AppShell {
    pub fn new(mut db: InfiniteDb) -> Self {
        ensure_walls_space(&mut db);
        ensure_results_space(&mut db);

        let (tx, signal_rx) = flume::unbounded();
        std::thread::spawn(move || {
            std::thread::sleep(std::time::Duration::from_secs(2));
            let _ = tx.send("engine online".into());
        });

        let first = AnalysisWorkspace::new(WorkspaceId(1), &mut db);
        let active_id = first.tab.id;
        let workspaces = vec![WorkspaceInstance::Analysis(first)];
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
            pages_area: Rect::from_xywh(0.0, 72.0, 1280.0, 728.0),
            has_header: true,
        }
    }
}
