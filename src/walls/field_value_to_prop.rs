use hyper_ui::FieldValue;
use hypernode::PropValue;

/// Convert a committed UI field value into a HyperNode property.
pub fn field_value_to_prop(value: &FieldValue, prefer_u8: bool) -> PropValue {
    match value {
        FieldValue::F64(v) => {
            if prefer_u8 {
                PropValue::U8(v.round().clamp(0.0, 255.0) as u8)
            } else {
                PropValue::F64(*v)
            }
        }
        FieldValue::Text(s) => {
            if prefer_u8 {
                if let Ok(n) = s.parse::<u8>() {
                    return PropValue::U8(n);
                }
            }
            if let Ok(n) = s.parse::<f64>() {
                return PropValue::F64(n);
            }
            match s.to_ascii_lowercase().as_str() {
                "true" | "yes" | "1" => PropValue::Bool(true),
                "false" | "no" | "0" => PropValue::Bool(false),
                _ => PropValue::Text(s.clone()),
            }
        }
        FieldValue::Bool(b) => PropValue::Bool(*b),
    }
}
