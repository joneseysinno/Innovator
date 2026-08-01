//! First composite particle — composition, not inheritance.
//!
//! `engineer_input` returns a `Particle::Surface` subtree:
//! surface → stack(row) → [label | field | unit]

mod input;
mod into_particle;
mod readonly;

pub use input::{engineer_input, EngineerInput};
pub use readonly::engineer_input_readonly;
