use crate::auth::role::Role;
use std::collections::HashSet;

/// Fine-grained permission checked against workspace descriptors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Capability {
    RunStructuralAnalysis,
    ViewCostEstimates,
    EditSchedule,
    ExportReports,
    ManageUsers,
    ViewOwnerDashboard,
}

/// Set of capabilities held by a session.
#[derive(Debug, Clone, Default)]
pub struct CapabilitySet(pub HashSet<Capability>);

impl CapabilitySet {
    pub fn has(&self, cap: Capability) -> bool {
        self.0.contains(&cap)
    }

    pub fn from_role(role: Role) -> Self {
        use Capability::*;
        let caps: &[Capability] = match role {
            Role::StructuralEngineer => &[RunStructuralAnalysis, ExportReports],
            Role::ProjectManager => &[EditSchedule, ExportReports],
            Role::Estimator => &[ViewCostEstimates, ExportReports],
            Role::BimCoordinator => &[ExportReports],
            Role::FieldInspector => &[ExportReports],
            Role::Owner => &[ViewOwnerDashboard, ExportReports],
            Role::Admin => &[
                RunStructuralAnalysis,
                ViewCostEstimates,
                EditSchedule,
                ExportReports,
                ManageUsers,
                ViewOwnerDashboard,
            ],
        };
        Self(caps.iter().copied().collect())
    }
}
