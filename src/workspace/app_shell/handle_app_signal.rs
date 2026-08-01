use super::add_workspace::add_workspace;
use super::select_workspace::select_workspace;
use super::AppShell;
use crate::workspace::app_signal::AppSignal;

pub fn handle_app_signal(shell: &mut AppShell, signal: AppSignal) {
    match signal {
        AppSignal::SelectWorkspace(id) => select_workspace(shell, id),
        AppSignal::AddWorkspace => add_workspace(shell),
    }
}
