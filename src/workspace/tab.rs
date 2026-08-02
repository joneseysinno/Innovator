use crate::workspace::workspace_id::WorkspaceId;

/// Entry in the app-level workspace tab strip.
#[derive(Debug, Clone)]
pub struct WorkspaceTab {
    pub id: WorkspaceId,
    pub kind_id: &'static str,
    pub title: String,
    pub icon: &'static str,
}

impl WorkspaceTab {
    pub fn new(
        id: WorkspaceId,
        kind_id: &'static str,
        title: impl Into<String>,
        icon: &'static str,
    ) -> Self {
        Self {
            id,
            kind_id,
            title: title.into(),
            icon,
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }
}
