//! Home dashboard workspace — entry point to other workspaces.

use crate::workspace::app_signal::AppSignal;
use hyper_ui::ParticleId;
use std::collections::HashMap;

pub struct HomeWorkspace {
    /// Dashboard action triggers (OpenWorkspace).
    pub actions: HashMap<ParticleId, AppSignal>,
}
