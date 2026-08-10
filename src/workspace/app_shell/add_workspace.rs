use super::rebuild_active::rebuild_active;
use super::AppShell;
use crate::workspace::seed;
use crate::workspace::workspace::Workspace;
use crate::workspace::workspace_id::WorkspaceId;
use hyper_ui::container::Visibility;

/// `+` on the tab strip — adds another Structural Analysis workspace and selects it.
pub fn add_workspace(shell: &mut AppShell) {
    let id = WorkspaceId(shell.next_workspace_id);
    shell.next_workspace_id += 1;
    let n = shell
        .workspaces
        .iter()
        .filter(|w| w.open_id() == seed::STRUCTURAL.open_id)
        .count()
        + 1;
    let title = format!("{} {n}", seed::STRUCTURAL.label);
    let mut ws = Workspace::new_structural_titled(id, title, &mut shell.db);
    ws.state.intent = Visibility::Hidden;
    shell.workspaces.push(ws);
    shell.set_active(id);
    shell.persist_layout();

    let mut renderer = match shell.renderer.take() {
        Some(r) => r,
        None => return,
    };
    rebuild_active(shell, &mut renderer);
    shell.renderer = Some(renderer);
}
