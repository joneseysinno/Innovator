use serde::{Deserialize, Serialize};

/// One capacity check produced by the analysis engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckResult {
    pub name: String,
    pub demand: f64,
    pub capacity: f64,
    pub ratio: f64,
    pub pass: bool,
    /// Unit label for demand/capacity display.
    pub unit: String,
    /// Custom/info rows — not structural pass/fail.
    #[serde(default)]
    pub informational: bool,
}

impl CheckResult {
    pub fn structural(
        name: impl Into<String>,
        demand: f64,
        capacity: f64,
        unit: impl Into<String>,
    ) -> Self {
        let capacity = capacity.max(1e-9);
        let ratio = demand / capacity;
        Self {
            name: name.into(),
            demand,
            capacity,
            ratio,
            pass: ratio <= 1.0,
            unit: unit.into(),
            informational: false,
        }
    }

    pub fn info(name: impl Into<String>, value: f64, unit: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            demand: value,
            capacity: 0.0,
            ratio: 0.0,
            pass: true,
            unit: unit.into(),
            informational: true,
        }
    }
}
