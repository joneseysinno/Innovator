use hypernode::PropValue;
use std::collections::BTreeMap;

/// Default property set for a special concrete wall.
pub fn default_wall_props(name: impl Into<String>) -> BTreeMap<String, PropValue> {
    let name = name.into();
    BTreeMap::from([
        ("wall_name".into(), PropValue::Text(name)),
        (
            "wall_type".into(),
            PropValue::Text("special_concrete".into()),
        ),
        ("height".into(), PropValue::F64(12.0)),
        ("length".into(), PropValue::F64(20.0)),
        ("thickness".into(), PropValue::F64(8.0)),
        ("clear_cover".into(), PropValue::F64(0.75)),
        ("fc".into(), PropValue::F64(4000.0)),
        ("fy".into(), PropValue::F64(60000.0)),
        ("es".into(), PropValue::F64(29000.0)),
        ("lambda".into(), PropValue::F64(1.0)),
        ("vert_bar_size".into(), PropValue::U8(5)),
        ("vert_spacing".into(), PropValue::F64(12.0)),
        ("horiz_bar_size".into(), PropValue::U8(4)),
        ("horiz_spacing".into(), PropValue::F64(12.0)),
        ("pu".into(), PropValue::F64(0.0)),
        ("vu".into(), PropValue::F64(0.0)),
        ("mu".into(), PropValue::F64(0.0)),
    ])
}
