use super::HomeWorkspace;
use crate::workspace::kind::WorkspaceKind;
use crate::workspace::tab::WorkspaceTab;
use crate::workspace::workspace_id::WorkspaceId;
use std::collections::HashMap;

impl HomeWorkspace {
    pub fn new(id: WorkspaceId) -> Self {
        Self {
            tab: WorkspaceTab::new(id, WorkspaceKind::Home),
            actions: HashMap::new(),
        }
    }
}
