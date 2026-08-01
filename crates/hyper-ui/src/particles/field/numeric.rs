use super::FieldValue;

/// Trait for values that can drive a typed field (kept for API compatibility).
pub trait NumericValue: Send + Sync {
    fn to_field_value(&self) -> FieldValue;
    fn parse(raw: &str) -> Result<FieldValue, ()>;
}

impl NumericValue for f64 {
    fn to_field_value(&self) -> FieldValue {
        FieldValue::F64(*self)
    }
    fn parse(raw: &str) -> Result<FieldValue, ()> {
        raw.trim().parse::<f64>().map(FieldValue::F64).map_err(|_| ())
    }
}
