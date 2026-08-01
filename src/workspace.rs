//! App shell (workspace tabs) + per-workspace content.

pub mod analysis;
pub mod analysis_action;
pub mod app_shell;
pub mod app_signal;
pub mod empty;
pub mod field_builder_draft;
pub mod header;
pub mod instance;
pub mod kind;
pub mod page;
pub mod screen_class;
pub mod signal;
pub mod size_class;
pub mod tab;
pub mod tab_strip;
pub mod workspace_id;

pub use analysis::AnalysisWorkspace;
pub use app_shell::AppShell;
pub use app_signal::AppSignal;
pub use header::{WorkspaceHeader, HEADER_HEIGHT};
pub use kind::WorkspaceKind;
pub use page::Page;
pub use screen_class::ScreenClass;
pub use signal::WorkspaceSignal;
pub use tab::WorkspaceTab;
pub use tab_strip::{TabStripIO, TAB_STRIP_HEIGHT};
pub use workspace_id::WorkspaceId;
