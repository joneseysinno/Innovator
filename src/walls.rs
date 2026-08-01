//! Special concrete wall domain helpers (create / persist / load).

pub mod default_props;
pub mod field_value_to_prop;
pub mod format_prop;
pub mod load_walls;
pub mod new_wall;
pub mod persist_wall;
pub mod prop_f64;
pub mod slug_key;
pub mod space;
pub mod standard_keys;

pub use default_props::default_wall_props;
pub use field_value_to_prop::field_value_to_prop;
pub use format_prop::format_prop;
pub use load_walls::load_walls;
pub use new_wall::new_wall;
pub use persist_wall::persist_wall;
pub use prop_f64::prop_f64;
pub use slug_key::slug_key;
pub use space::{ensure_walls_space, WALLS_SPACE};
pub use standard_keys::{is_geometry_or_rebar_key, is_standard_key};
