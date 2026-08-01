use hyper_ui::{engineer_input as hyper_engineer_input, EngineerInput};

/// App-facing engineer input — wraps the hyper-ui composition helper.
pub fn engineer_input(label: &str, value: f64, unit: &str) -> EngineerInput {
    hyper_engineer_input(label, value, unit)
}
