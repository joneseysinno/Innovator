//! Home dashboard domain workspace.

pub mod build_content;
pub mod new;
pub mod workspace;

pub use workspace::HomeWorkspace;

use crate::auth::capability::Capability;
use crate::workspace::descriptor::WorkspaceDescriptor;
use crate::workspace::facade::WorkspaceFacade;
use crate::workspace::workspace_id::WorkspaceId;
use infinite_db::InfiniteDb;

/// Stable kind id — stored in config / URLs. Never rename after shipping.
pub const KIND_ID: &str = "home";

pub struct HomeDescriptor;

impl HomeDescriptor {
    pub const KIND_ID: &'static str = KIND_ID;
    pub const LABEL: &'static str = "Home";
    pub const ICON: &'static str = "⌂";
}

impl WorkspaceDescriptor for HomeDescriptor {
    fn kind_id(&self) -> &'static str {
        Self::KIND_ID
    }

    fn label(&self) -> &'static str {
        Self::LABEL
    }

    fn icon(&self) -> &'static str {
        Self::ICON
    }

    fn required_capabilities(&self) -> &[Capability] {
        &[]
    }

    fn spawn(&self, id: WorkspaceId, _db: &mut InfiniteDb) -> Box<dyn WorkspaceFacade> {
        Box::new(HomeWorkspace::new(id))
    }
}
