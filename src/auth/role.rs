/// User role — drives default capability sets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Role {
    StructuralEngineer,
    ProjectManager,
    Estimator,
    BimCoordinator,
    FieldInspector,
    Owner,
    Admin,
}
