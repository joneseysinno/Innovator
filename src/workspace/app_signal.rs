use crate::workspace::workspace_id::WorkspaceId;

/// App-level chrome signals (tab strip + Home dashboard), not workspace header actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppSignal {
    SelectWorkspace(WorkspaceId),
    /// `+` on the tab strip — always adds Structural Analysis.
    AddWorkspace,
    /// Focus an existing seeded workspace — visibility write + focus update.
    OpenWorkspace(&'static str),
}
