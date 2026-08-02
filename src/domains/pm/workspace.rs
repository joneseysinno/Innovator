//! Project Management workspace (stub landing + light header).

use super::build_content;
use super::KIND_ID;
use crate::workspace::facade::WorkspaceFacade;
use crate::workspace::header::WorkspaceHeader;
use crate::workspace::tab::WorkspaceTab;
use hyper_ui::particles::Particle;
use hyper_ui::ParticleId;
use std::any::Any;

pub struct PmWorkspace {
    pub tab: WorkspaceTab,
    pub header: Option<WorkspaceHeader>,
}

impl PmWorkspace {
    pub fn status_id(&self) -> Option<hyper_ui::ParticleId> {
        self.header.as_ref().map(|h| h.status_id)
    }
}

impl WorkspaceFacade for PmWorkspace {
    fn tab(&self) -> &WorkspaceTab {
        &self.tab
    }

    fn kind_id(&self) -> &'static str {
        KIND_ID
    }

    fn header(&self) -> Option<&WorkspaceHeader> {
        self.header.as_ref()
    }

    fn status_id(&self) -> Option<ParticleId> {
        self.header.as_ref().map(|h| h.status_id)
    }

    fn build_content(&mut self) -> Particle {
        build_content::build_content()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
