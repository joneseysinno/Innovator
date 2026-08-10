/// Stable identity for a container at any level.
/// Persisted. Survives restart, sync, and re-arrangement.
///
/// One id space across workspace, page, and pod. Unique app-wide.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ContainerId(pub u64);
