//! winit → UiEvent routing with hit-testing, focus, and hover.

mod event;
mod hit_kind;
mod router;

pub use event::UiEvent;
pub use router::InputRouter;
