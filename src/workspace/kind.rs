/// Kind of workspace hosted in the app shell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WorkspaceKind {
    /// Structural Analysis
    Analysis,
    /// Project Management
    PM,
    ///Home dashboard
    Home,
    /// Stub workspace with no header (demo tab).
    Empty,
}

impl WorkspaceKind {
    pub fn default_title(self) -> &'static str {
        match self {
            Self::Analysis => "Structural Analysis",
            Self::PM => "Project Management",
            Self::Home => "Home",
            Self::Empty => "Empty",
        }
    }
}
