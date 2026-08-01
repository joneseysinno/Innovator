//! Project Management workspace (stub landing + light header).

pub mod build_content;
pub mod new;

use crate::workspace::header::WorkspaceHeader;
use crate::workspace::tab::WorkspaceTab;

pub struct PmWorkspace {
    pub tab: WorkspaceTab,
    pub header: Option<WorkspaceHeader>,
}

impl PmWorkspace {
    pub fn status_id(&self) -> Option<hyper_ui::ParticleId> {
        self.header.as_ref().map(|h| h.status_id)
    }
}
