use hypernode::NodeId;

/// Stable identity for a component within a pod — fourth container tier.
///
/// Wraps [`NodeId`] so components are first-class graph citizens
/// (`SpaceClass::UIView`). Populated by `write_pod_components` in innovator.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ComponentId(pub NodeId);
