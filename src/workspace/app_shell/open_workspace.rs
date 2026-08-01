use super::rebuild_active::rebuild_active;
use super::select_workspace::select_workspace;
use super::AppShell;
use crate::workspace::analysis::AnalysisWorkspace;
use crate::workspace::empty::EmptyWorkspace;
use crate::workspace::home::HomeWorkspace;
use crate::workspace::instance::WorkspaceInstance;
use crate::workspace::kind::WorkspaceKind;
use crate::workspace::pm::PmWorkspace;
use crate::workspace::workspace_id::WorkspaceId;

/// Focus an existing tab of `kind`, or create and select a new one.
pub fn open_workspace(shell: &mut AppShell, kind: WorkspaceKind) {
    if let Some(id) = shell
        .workspaces
        .iter()
        .find(|w| w.kind() == kind)
        .map(|w| w.id())
    {
        select_workspace(shell, id);
        return;
    }

    let id = WorkspaceId(shell.next_workspace_id);
    shell.next_workspace_id += 1;

    let instance = match kind {
        WorkspaceKind::Analysis => {
            WorkspaceInstance::Analysis(AnalysisWorkspace::new(id, &mut shell.db))
        }
        WorkspaceKind::PM => WorkspaceInstance::Pm(PmWorkspace::new(id)),
        WorkspaceKind::Home => WorkspaceInstance::Home(HomeWorkspace::new(id)),
        WorkspaceKind::Empty => WorkspaceInstance::Empty(EmptyWorkspace::new(id)),
    };

    shell.workspaces.push(instance);
    shell.active_id = id;

    let mut renderer = match shell.renderer.take() {
        Some(r) => r,
        None => return,
    };
    rebuild_active(shell, &mut renderer);
    shell.renderer = Some(renderer);
}
