use super::select_workspace::select_workspace;
use super::AppShell;

/// Open an existing seeded workspace — visibility write + focus. Never spawns.
pub fn open_workspace(shell: &mut AppShell, open_id: &'static str) {
    let Some(id) = shell
        .workspaces
        .iter()
        .find(|w| w.open_id() == open_id)
        .map(|w| w.id())
    else {
        return;
    };
    select_workspace(shell, id);
}
