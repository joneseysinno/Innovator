//! App-level Blender-style workspace tab strip.

pub mod build;
pub mod height;

pub use build::{build_tab_strip, TabStripIO};
pub use height::TAB_STRIP_HEIGHT;
