use super::PmWorkspace;
use crate::workspace::header::build_pm::build_pm_header;
use crate::workspace::kind::WorkspaceKind;
use crate::workspace::tab::WorkspaceTab;
use crate::workspace::workspace_id::WorkspaceId;

impl PmWorkspace {
    pub fn new(id: WorkspaceId) -> Self {
        Self {
            tab: WorkspaceTab::new(id, WorkspaceKind::PM),
            header: Some(build_pm_header()),
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.tab.title = title.into();
        self
    }
}
