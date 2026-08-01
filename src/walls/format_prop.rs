use hypernode::PropValue;

/// Display string for a wall property value.
pub fn format_prop(value: &PropValue) -> String {
    match value {
        PropValue::F64(v) => format!("{v}"),
        PropValue::I64(v) => format!("{v}"),
        PropValue::U8(v) => format!("{v}"),
        PropValue::Bool(v) => format!("{v}"),
        PropValue::Text(v) => v.clone(),
    }
}
