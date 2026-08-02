//! Stub empty workspace — no header.

use super::build_content;
use super::KIND_ID;
use crate::workspace::facade::WorkspaceFacade;
use crate::workspace::tab::WorkspaceTab;
use hyper_ui::particles::Particle;
use std::any::Any;

pub struct EmptyWorkspace {
    pub tab: WorkspaceTab,
}

impl WorkspaceFacade for EmptyWorkspace {
    fn tab(&self) -> &WorkspaceTab {
        &self.tab
    }

    fn kind_id(&self) -> &'static str {
        KIND_ID
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
