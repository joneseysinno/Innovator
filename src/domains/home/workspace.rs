//! Home dashboard workspace — entry point to other workspaces.

use super::build_content;
use super::KIND_ID;
use crate::workspace::app_signal::AppSignal;
use crate::workspace::facade::WorkspaceFacade;
use crate::workspace::tab::WorkspaceTab;
use hyper_ui::particles::Particle;
use hyper_ui::ParticleId;
use std::any::Any;
use std::collections::HashMap;

pub struct HomeWorkspace {
    pub tab: WorkspaceTab,
    /// Dashboard action triggers (OpenWorkspace).
    pub actions: HashMap<ParticleId, AppSignal>,
}

impl WorkspaceFacade for HomeWorkspace {
    fn tab(&self) -> &WorkspaceTab {
        &self.tab
    }

    fn kind_id(&self) -> &'static str {
        KIND_ID
    }

    fn build_content(&mut self) -> Particle {
        build_content::build_content(self)
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
