use hypernode::NodeId;

/// Workspace-level Signal hyperedge kinds fired by chrome and page IO.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WorkspaceSignal {
    NewWall,
    Save,
    RunAnalysis,
    Export,
    WallSelected(NodeId),
    /// Fired after the analysis engine writes ResultsNode.
    AnalysisComplete,
}

impl WorkspaceSignal {
    pub fn label(self) -> &'static str {
        match self {
            Self::NewWall => "New Wall",
            Self::Save => "Save",
            Self::RunAnalysis => "Run",
            Self::Export => "Export",
            Self::WallSelected(_) => "Wall Selected",
            Self::AnalysisComplete => "Analysis Complete",
        }
    }
}
