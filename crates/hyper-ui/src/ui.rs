//! Layer B UI renderer — particle tree → wgpu draw calls.

mod apply_signal_text;
mod collect_rects;
mod collect_tab_order;
mod cursor_pos;
mod renderer;

pub use apply_signal_text::apply_signal_text;
pub use cursor_pos::cursor_pos;
pub use renderer::UiRenderer;

pub(crate) use collect_rects::collect_rects;
pub(crate) use collect_tab_order::collect_tab_order;
