use super::rebuild_active::rebuild_active;
use super::AppShell;
use crate::workspace::workspace_id::WorkspaceId;

pub fn select_workspace(shell: &mut AppShell, id: WorkspaceId) {
    if !shell.set_active(id) {
        return;
    }
    let mut renderer = match shell.renderer.take() {
        Some(r) => r,
        None => return,
    };
    rebuild_active(shell, &mut renderer);
    shell.renderer = Some(renderer);
}
