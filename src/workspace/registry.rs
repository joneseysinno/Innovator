//! Registry of available workspace descriptors.

use crate::auth::capability::CapabilitySet;
use crate::workspace::descriptor::WorkspaceDescriptor;
use crate::workspace::facade::WorkspaceFacade;
use crate::workspace::workspace_id::WorkspaceId;
use infinite_db::InfiniteDb;

pub struct WorkspaceRegistry {
    descriptors: Vec<Box<dyn WorkspaceDescriptor>>,
}

impl WorkspaceRegistry {
    pub fn new() -> Self {
        Self {
            descriptors: Vec::new(),
        }
    }

    pub fn register(&mut self, d: Box<dyn WorkspaceDescriptor>) {
        self.descriptors.push(d);
    }

    /// Descriptors the current user may open.
    pub fn available_for<'a>(&'a self, caps: &CapabilitySet) -> Vec<&'a dyn WorkspaceDescriptor> {
        self.descriptors
            .iter()
            .filter(|d| {
                d.required_capabilities().is_empty()
                    || d.required_capabilities().iter().all(|c| caps.has(*c))
            })
            .map(|d| d.as_ref())
            .collect()
    }

    pub fn find(&self, kind_id: &str) -> Option<&dyn WorkspaceDescriptor> {
        self.descriptors
            .iter()
            .find(|d| d.kind_id() == kind_id)
            .map(|d| d.as_ref())
    }

    /// Spawn via kind_id — disjoint-borrow friendly when called as
    /// `shell.registry.spawn(..., &mut shell.db)`.
    pub fn spawn(
        &self,
        kind_id: &str,
        id: WorkspaceId,
        db: &mut InfiniteDb,
    ) -> Option<Box<dyn WorkspaceFacade>> {
        self.find(kind_id).map(|d| d.spawn(id, db))
    }
}

impl Default for WorkspaceRegistry {
    fn default() -> Self {
        Self::new()
    }
}
