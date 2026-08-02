//! Registration contract for domain workspaces.

use crate::auth::capability::Capability;
use crate::workspace::facade::WorkspaceFacade;
use crate::workspace::workspace_id::WorkspaceId;
use infinite_db::InfiniteDb;

/// Contract every domain workspace must satisfy for discovery / spawning.
/// The shell calls these methods — domains never call back into the shell.
pub trait WorkspaceDescriptor: Send + Sync {
    /// Stable ASCII key — stored in DB, config files, URLs.
    /// Never rename after shipping.  Example: `"structural_analysis"`
    fn kind_id(&self) -> &'static str;

    /// Label shown in the launcher / tab strip.
    fn label(&self) -> &'static str;

    /// Glyph shown in the launcher grid.
    fn icon(&self) -> &'static str;

    /// Capabilities required to open this workspace.
    /// Empty slice = available to all users.
    fn required_capabilities(&self) -> &[Capability];

    /// Spawn a live instance.
    fn spawn(&self, id: WorkspaceId, db: &mut InfiniteDb) -> Box<dyn WorkspaceFacade>;
}
