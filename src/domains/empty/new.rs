use super::workspace::EmptyWorkspace;
use crate::domains::empty::EmptyDescriptor;
use crate::workspace::tab::WorkspaceTab;
use crate::workspace::workspace_id::WorkspaceId;

impl EmptyWorkspace {
    pub fn new(id: WorkspaceId) -> Self {
        Self {
            tab: WorkspaceTab::new(
                id,
                EmptyDescriptor::KIND_ID,
                EmptyDescriptor::LABEL,
                EmptyDescriptor::ICON,
            ),
        }
    }
}
