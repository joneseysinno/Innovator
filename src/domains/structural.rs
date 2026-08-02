//! Structural analysis domain workspace.

pub mod action;
pub mod build_icon_rail;
pub mod build_page_header;
pub mod build_pages;
pub mod field_builder_draft;
pub mod io_kind;
pub mod kind;
pub mod new;
pub mod page_signal;
pub mod templates;
pub mod workspace;

pub use action::AnalysisAction;
pub use field_builder_draft::{BuilderFieldSlot, CustomFieldKind, FieldBuilderDraft};
pub use io_kind::IoKind;
pub use kind::AnalysisKind;
pub use page_signal::PageSignal;
pub use workspace::StructuralWorkspace;

use crate::auth::capability::Capability;
use crate::workspace::descriptor::WorkspaceDescriptor;
use crate::workspace::facade::WorkspaceFacade;
use crate::workspace::workspace_id::WorkspaceId;
use infinite_db::InfiniteDb;

/// Stable kind id — stored in config / URLs. Never rename after shipping.
pub const KIND_ID: &str = "structural_analysis";

pub struct StructuralDescriptor;

impl StructuralDescriptor {
    pub const KIND_ID: &'static str = KIND_ID;
    pub const LABEL: &'static str = "Structural Analysis";
    pub const ICON: &'static str = "⬡";
}

impl WorkspaceDescriptor for StructuralDescriptor {
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
        &[Capability::RunStructuralAnalysis]
    }

    fn spawn(&self, id: WorkspaceId, db: &mut InfiniteDb) -> Box<dyn WorkspaceFacade> {
        Box::new(StructuralWorkspace::new(id, db))
    }
}
