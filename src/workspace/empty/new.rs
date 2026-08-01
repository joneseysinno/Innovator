use super::EmptyWorkspace;
use crate::workspace::kind::WorkspaceKind;
use crate::workspace::tab::WorkspaceTab;
use crate::workspace::workspace_id::WorkspaceId;

impl EmptyWorkspace {
    pub fn new(id: WorkspaceId) -> Self {
        Self {
            tab: WorkspaceTab::new(id, WorkspaceKind::Empty),
        }
    }
}
