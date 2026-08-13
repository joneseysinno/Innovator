use super::select_workspace::select_workspace;
use super::AppShell;
use crate::workspace::graph_containers::resolve_home_nav_target;

/// Open an existing seeded workspace — visibility write reached through a
/// Home→workspace `Signal` edge walk. Never spawns.
pub fn open_workspace(shell: &mut AppShell, open_id: &'static str) {
    let Some(home) = shell.workspaces.iter().find(|w| w.open_id() == "home") else {
        return;
    };
    let home_node = home.node_id;
    let Some(target_node) = resolve_home_nav_target(&shell.graph, home_node, open_id) else {
        return;
    };
    let Some(id) = shell
        .workspaces
        .iter()
        .find(|w| w.node_id == target_node)
        .map(|w| w.id())
    else {
        return;
    };
    select_workspace(shell, id);
}
