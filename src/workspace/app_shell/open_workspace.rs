use super::rebuild_active::rebuild_active;
use super::select_workspace::select_workspace;
use super::AppShell;
use crate::workspace::instance::WorkspaceInstance;
use crate::workspace::workspace_id::WorkspaceId;

/// Focus an existing tab of `kind_id`, or create and select a new one.
pub fn open_workspace(shell: &mut AppShell, kind_id: &'static str) {
    if let Some(id) = shell
        .workspaces
        .iter()
        .find(|w| w.kind_id() == kind_id)
        .map(|w| w.id())
    {
        select_workspace(shell, id);
        return;
    }

    let id = WorkspaceId(shell.next_workspace_id);
    shell.next_workspace_id += 1;

    let Some(boxed) = shell.registry.spawn(kind_id, id, &mut shell.db) else {
        return;
    };
    let instance = WorkspaceInstance::from_boxed(boxed);

    shell.workspaces.push(instance);
    shell.active_id = id;

    let mut renderer = match shell.renderer.take() {
        Some(r) => r,
        None => return,
    };
    rebuild_active(shell, &mut renderer);
    shell.renderer = Some(renderer);
}
