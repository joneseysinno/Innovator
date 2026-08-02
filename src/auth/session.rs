use super::capability::CapabilitySet;
use super::role::Role;

/// Authenticated (or guest) user session.
pub struct Session {
    pub display_name: String,
    pub role: Role,
    pub capabilities: CapabilitySet,
}

impl Session {
    pub fn new(display_name: impl Into<String>, role: Role) -> Self {
        let capabilities = CapabilitySet::from_role(role);
        Self {
            display_name: display_name.into(),
            role,
            capabilities,
        }
    }

    /// Placeholder for single-user mode (no auth yet).
    pub fn guest() -> Self {
        Self::new("Guest", Role::StructuralEngineer)
    }
}
