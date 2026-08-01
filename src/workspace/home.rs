//! Home dashboard workspace — entry point to other workspaces.

pub mod build_content;
pub mod new;

use crate::workspace::app_signal::AppSignal;
use crate::workspace::tab::WorkspaceTab;
use hyper_ui::ParticleId;
use std::collections::HashMap;

pub struct HomeWorkspace {
    pub tab: WorkspaceTab,
    /// Dashboard action triggers (OpenWorkspace).
    pub actions: HashMap<ParticleId, AppSignal>,
}
