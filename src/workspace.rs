//! App shell infrastructure — tabs, signals, layout chrome.

pub mod app_shell;
pub mod app_signal;
pub mod from_seed;
pub mod graph_containers;
pub mod handle_result;
pub mod header;
pub mod page;
pub mod seed;
pub mod persist;
pub mod signal;
pub mod tab;
pub mod tab_strip;
pub mod workspace;
pub mod workspace_id;

pub use app_shell::AppShell;
pub use app_signal::AppSignal;
pub use handle_result::HandleResult;
pub use header::{WorkspaceHeader, HEADER_HEIGHT};
pub use page::Page;
pub use signal::WorkspaceSignal;
pub use tab::WorkspaceTab;
pub use tab_strip::{TabStripIO, TAB_STRIP_HEIGHT};
pub use workspace::Workspace;
pub use workspace_id::WorkspaceId;
