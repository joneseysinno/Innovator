/// Numeric / textual values a field can hold.
#[derive(Debug, Clone, PartialEq)]
pub enum FieldValue {
    F64(f64),
    Text(String),
    Bool(bool),
}

impl FieldValue {
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            Self::F64(v) => Some(*v),
            Self::Text(s) => s.parse().ok(),
            Self::Bool(b) => Some(if *b { 1.0 } else { 0.0 }),
        }
    }

    pub fn display(&self) -> String {
        match self {
            Self::F64(v) => format_number(*v),
            Self::Text(s) => s.clone(),
            Self::Bool(b) => b.to_string(),
        }
    }
}

fn format_number(v: f64) -> String {
    if v.fract() == 0.0 {
        format!("{v:.0}")
    } else {
        format!("{v}")
    }
}
