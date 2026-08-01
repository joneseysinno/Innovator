use crate::workspace::workspace_id::WorkspaceId;

/// App-level chrome signals (tab strip), not workspace header actions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AppSignal {
    SelectWorkspace(WorkspaceId),
    AddWorkspace,
}
