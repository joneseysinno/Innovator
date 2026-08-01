use crate::workspace::kind::WorkspaceKind;
use crate::workspace::workspace_id::WorkspaceId;

/// Entry in the app-level workspace tab strip.
#[derive(Debug, Clone)]
pub struct WorkspaceTab {
    pub id: WorkspaceId,
    pub title: String,
    pub kind: WorkspaceKind,
}

impl WorkspaceTab {
    pub fn new(id: WorkspaceId, kind: WorkspaceKind) -> Self {
        Self {
            id,
            title: kind.default_title().into(),
            kind,
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }
}
