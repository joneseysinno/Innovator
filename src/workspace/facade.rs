//! Trait object contract for live workspace instances.

use crate::workspace::app_signal::AppSignal;
use crate::workspace::header::WorkspaceHeader;
use crate::workspace::signal::WorkspaceSignal;
use crate::workspace::tab::WorkspaceTab;
use crate::workspace::workspace_id::WorkspaceId;
use hyper_ui::particles::Particle;
use hyper_ui::{PageTree, ParticleId};
use std::any::Any;

/// Contract every domain workspace must satisfy.
/// The shell calls these — domains never call back into the shell.
pub trait WorkspaceFacade: Any {
    // ── Identity ──────────────────────────────────────────────────────────

    fn tab(&self) -> &WorkspaceTab;

    fn id(&self) -> WorkspaceId {
        self.tab().id
    }

    fn kind_id(&self) -> &'static str;

    // ── Chrome ────────────────────────────────────────────────────────────

    /// Optional action header rendered between the tab strip and page body.
    fn header(&self) -> Option<&WorkspaceHeader> {
        None
    }

    /// Status ParticleId for live signal text, if any.
    fn status_id(&self) -> Option<ParticleId> {
        None
    }

    // ── Layout ────────────────────────────────────────────────────────────

    /// Page-split binary tree, if this workspace uses one.
    fn page_tree(&self) -> Option<&PageTree> {
        None
    }

    fn page_tree_mut(&mut self) -> Option<&mut PageTree> {
        None
    }

    // ── Content ───────────────────────────────────────────────────────────

    /// Build the workspace body particle tree. Called on every rebuild.
    fn build_content(&mut self) -> Particle;

    // ── Event dispatch ────────────────────────────────────────────────────

    /// Handle a workspace-level signal. Return true if a rebuild is needed.
    fn handle_workspace_signal(
        &mut self,
        signal: WorkspaceSignal,
        db: &mut infinite_db::InfiniteDb,
        signal_tx: &flume::Sender<String>,
    ) -> HandleResult {
        let _ = (signal, db, signal_tx);
        HandleResult::Ignored
    }

    /// Handle an app-level signal. Return true if a rebuild is needed.
    fn handle_app_signal(&mut self, signal: AppSignal) -> HandleResult {
        let _ = signal;
        HandleResult::Ignored
    }

    // ── Type erasure escape hatch ─────────────────────────────────────────

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Result returned from event handlers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandleResult {
    /// Event was consumed; caller must rebuild the particle tree.
    Rebuild,
    /// Event was consumed; no rebuild needed (e.g. status text update only).
    Consumed,
    /// Event was not handled by this workspace.
    Ignored,
}
