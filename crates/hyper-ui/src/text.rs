//! glyphon text wrapper with content-keyed Buffer cache.

mod collect_text;
mod key;
mod pending;
mod renderer;

pub use collect_text::collect_text;
pub use key::TextKey;
pub use renderer::TextRenderer;

pub(crate) use pending::PendingText;
