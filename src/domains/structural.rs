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
