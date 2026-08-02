mod draw;
mod handle_event;
mod new;
mod rebuild;

use crate::geom::Rect;

use super::PodId;

/// Thin horizontal strip between vertically stacked pods.
#[derive(Debug, Clone)]
pub struct PodDivider {
    pub above: PodId,
    pub below: PodId,
    pub rect: Rect,
    pub hovered: bool,
    pub dragging: bool,
}

/// Renderer / hit-test state for pod dividers within one or more pages.
pub struct PodDividerRenderer {
    pub dividers: Vec<PodDivider>,
    /// Content-area height for the page that owns each divider (parallel to `dividers`).
    pub area_heights: Vec<f32>,
    drag: Option<(usize, f32)>,
    last_click: Option<(usize, std::time::Instant)>,
}
