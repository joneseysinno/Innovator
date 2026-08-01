//! Stub empty workspace — no header.

pub mod build_content;
pub mod new;

use crate::workspace::tab::WorkspaceTab;

pub struct EmptyWorkspace {
    pub tab: WorkspaceTab,
}
