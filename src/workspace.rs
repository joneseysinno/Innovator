//! App shell infrastructure — tabs, signals, layout chrome.

pub mod app_shell;
pub mod app_signal;
pub mod descriptor;
pub mod facade;
pub mod header;
pub mod instance;
pub mod page;
pub mod registry;
pub mod screen_class;
pub mod signal;
pub mod size_class;
pub mod tab;
pub mod tab_strip;
pub mod workspace_id;

pub use app_shell::AppShell;
pub use app_signal::AppSignal;
pub use descriptor::WorkspaceDescriptor;
pub use facade::{HandleResult, WorkspaceFacade};
pub use header::{WorkspaceHeader, HEADER_HEIGHT};
pub use page::Page;
pub use registry::WorkspaceRegistry;
pub use screen_class::ScreenClass;
pub use signal::WorkspaceSignal;
pub use tab::WorkspaceTab;
pub use tab_strip::{TabStripIO, TAB_STRIP_HEIGHT};
pub use workspace_id::WorkspaceId;
