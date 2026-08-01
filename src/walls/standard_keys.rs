/// Built-in wall property keys (not shown in Custom section).
pub fn is_standard_key(key: &str) -> bool {
    matches!(
        key,
        "wall_name"
            | "wall_type"
            | "height"
            | "length"
            | "thickness"
            | "clear_cover"
            | "fc"
            | "fy"
            | "es"
            | "lambda"
            | "vert_bar_size"
            | "vert_spacing"
            | "horiz_bar_size"
            | "horiz_spacing"
            | "pu"
            | "vu"
            | "mu"
    )
}

pub fn is_geometry_or_rebar_key(key: &str) -> bool {
    matches!(
        key,
        "height"
            | "length"
            | "thickness"
            | "clear_cover"
            | "vert_bar_size"
            | "vert_spacing"
            | "horiz_bar_size"
            | "horiz_spacing"
    )
}
