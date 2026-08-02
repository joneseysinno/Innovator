use super::workspace::HomeWorkspace;
use crate::domains::home::HomeDescriptor;
use crate::workspace::tab::WorkspaceTab;
use crate::workspace::workspace_id::WorkspaceId;
use std::collections::HashMap;

impl HomeWorkspace {
    pub fn new(id: WorkspaceId) -> Self {
        Self {
            tab: WorkspaceTab::new(
                id,
                HomeDescriptor::KIND_ID,
                HomeDescriptor::LABEL,
                HomeDescriptor::ICON,
            ),
            actions: HashMap::new(),
        }
    }
}
