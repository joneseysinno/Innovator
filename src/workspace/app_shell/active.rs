use super::AppShell;
use crate::workspace::instance::WorkspaceInstance;
use crate::workspace::workspace_id::WorkspaceId;

impl AppShell {
    pub fn active(&self) -> Option<&WorkspaceInstance> {
        self.workspaces.iter().find(|w| w.id() == self.active_id)
    }

    pub fn active_mut(&mut self) -> Option<&mut WorkspaceInstance> {
        let id = self.active_id;
        self.workspaces.iter_mut().find(|w| w.id() == id)
    }

    pub fn set_active(&mut self, id: WorkspaceId) -> bool {
        if self.workspaces.iter().any(|w| w.id() == id) {
            self.active_id = id;
            true
        } else {
            false
        }
    }
}
