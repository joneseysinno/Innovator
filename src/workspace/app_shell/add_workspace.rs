use super::rebuild_active::rebuild_active;
use super::AppShell;
use crate::workspace::analysis::AnalysisWorkspace;
use crate::workspace::instance::WorkspaceInstance;
use crate::workspace::kind::WorkspaceKind;
use crate::workspace::workspace_id::WorkspaceId;

/// `+` on the tab strip — adds another Analysis workspace and selects it.
pub fn add_workspace(shell: &mut AppShell) {
    let id = WorkspaceId(shell.next_workspace_id);
    shell.next_workspace_id += 1;
    let n = shell
        .workspaces
        .iter()
        .filter(|w| matches!(w, WorkspaceInstance::Analysis(_)))
        .count()
        + 1;
    let title = format!("{} {n}", WorkspaceKind::Analysis.default_title());
    let ws = AnalysisWorkspace::new(id, &mut shell.db).with_title(title);
    shell.workspaces.push(WorkspaceInstance::Analysis(ws));
    shell.active_id = id;

    let mut renderer = match shell.renderer.take() {
        Some(r) => r,
        None => return,
    };
    rebuild_active(shell, &mut renderer);
    shell.renderer = Some(renderer);
}
