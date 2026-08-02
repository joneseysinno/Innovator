//! Registers every AEC domain workspace with the shell registry.

pub mod empty;
pub mod home;
pub mod pm;
pub mod structural;

use crate::workspace::registry::WorkspaceRegistry;

pub fn register_all(registry: &mut WorkspaceRegistry) {
    registry.register(Box::new(structural::StructuralDescriptor));
    registry.register(Box::new(pm::PmDescriptor));
    registry.register(Box::new(home::HomeDescriptor));
    registry.register(Box::new(empty::EmptyDescriptor));
}
