/// Build a stable custom property key from a user label.
pub fn slug_key(label: &str) -> String {
    let mut out = String::from("custom:");
    let mut last_us = false;
    for ch in label.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            last_us = false;
        } else if !last_us && !out.ends_with(':') {
            out.push('_');
            last_us = true;
        }
    }
    while out.ends_with('_') {
        out.pop();
    }
    if out == "custom:" || out == "custom" {
        out.push_str("field");
    }
    out
}
