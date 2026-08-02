use crate::workspace::facade::WorkspaceFacade;

/// A live workspace instance — owns a boxed domain workspace.
pub struct WorkspaceInstance(pub Box<dyn WorkspaceFacade>);

impl WorkspaceInstance {
    pub fn new<W: WorkspaceFacade + 'static>(ws: W) -> Self {
        Self(Box::new(ws))
    }

    pub fn from_boxed(ws: Box<dyn WorkspaceFacade>) -> Self {
        Self(ws)
    }
}

impl std::ops::Deref for WorkspaceInstance {
    type Target = dyn WorkspaceFacade;
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl std::ops::DerefMut for WorkspaceInstance {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.0
    }
}
