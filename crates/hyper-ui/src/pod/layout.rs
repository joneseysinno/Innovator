use crate::container::{FocusPath, Visibility};
use crate::geom::{Rect, Vec2};
use crate::layout::{
    resolve, Axis, InputClass, Overflow, ResolveReport, SizeClass, Viewport, POD_LADDER,
};

use super::{PodId, PodList};

impl PodList {
    /// Resolve pod stack against `area` with [`Overflow::Scroll`].
    ///
    /// Writes `resolved`/`rect` onto each pod's [`crate::container::ContainerState`].
    /// Returns absolute `(PodId, Rect)` pairs (including gaps between pods).
    pub fn layout(&mut self, area: Rect) -> (Vec<(PodId, Rect)>, ResolveReport) {
        let viewport = Viewport {
            size: Vec2::new(area.size.x, area.size.y),
            scale_factor: 1.0,
            size_class: SizeClass::from_width(area.size.x.max(1.0)),
            input_class: InputClass::Pointer,
        };
        self.layout_with(area, &viewport, &FocusPath::default())
    }

    pub fn layout_with(
        &mut self,
        area: Rect,
        viewport: &Viewport,
        focus: &FocusPath,
    ) -> (Vec<(PodId, Rect)>, ResolveReport) {
        if self.pods.is_empty() {
            return (
                Vec::new(),
                ResolveReport {
                    demotions: Vec::new(),
                    promotions: Vec::new(),
                    scroll_extent: 0.0,
                    underflowed: false,
                },
            );
        }

        self.apply_resolved_collapse(viewport.size_class);

        for pod in &mut self.pods {
            pod.sync_state_from_fields();
        }

        let mut states: Vec<_> = self.pods.iter().map(|p| p.state.clone()).collect();
        // Gaps sit between pods outside resolve — budget the axis net of gaps
        // so surplus fill consumes the content area exactly when content fits.
        let gap_total = self.gap * self.pods.len().saturating_sub(1) as f32;
        let axis_for_pods = (area.size.y - gap_total).max(0.0);
        let mut report = resolve(
            &mut states,
            axis_for_pods,
            area.size.x,
            Axis::Vertical,
            Overflow::Scroll,
            POD_LADDER,
            0,
            focus,
            &self.overrides,
            viewport,
        );

        for (pod, state) in self.pods.iter_mut().zip(states.iter()) {
            pod.state = state.clone();
        }

        let n = self.pods.len();
        let mut out = Vec::with_capacity(n);
        let mut y = area.origin.y;
        for (i, (pod, state)) in self.pods.iter().zip(states.iter()).enumerate() {
            let h = match state.resolved() {
                Visibility::Hidden => 0.0,
                Visibility::Collapsed => POD_LADDER.collapsed_extent,
                Visibility::Shown => state.rect().size.y,
            };
            out.push((
                pod.id,
                Rect::from_xywh(area.origin.x, y, area.size.x, h),
            ));
            y += h;
            if i + 1 < n {
                y += self.gap;
            }
        }

        let content = out
            .last()
            .map(|(_, r)| r.origin.y + r.size.y - area.origin.y)
            .unwrap_or(0.0);
        report.scroll_extent = (content - area.size.y).max(0.0);
        self.scroll_offset = self.scroll_offset.clamp(0.0, report.scroll_extent);

        (out, report)
    }

    /// Immutable convenience for callers that only need rects.
    pub fn layout_rects(&self, area: Rect) -> Vec<(PodId, Rect)> {
        let mut clone = self.clone();
        clone.layout(area).0
    }

    /// Content-space y of a pod's title bar relative to `area`.
    pub fn content_y_of(rects: &[(PodId, Rect)], id: PodId, area: Rect) -> Option<f32> {
        rects
            .iter()
            .find(|(pid, _)| *pid == id)
            .map(|(_, r)| r.origin.y - area.origin.y)
    }

    /// After a collapse toggle, keep the pod's title at the same screen Y.
    pub fn anchor_scroll_on_toggle(&mut self, id: PodId, area: Rect, screen_y_before: f32) {
        let (rects, report) = self.layout(area);
        let Some(content_y) = Self::content_y_of(&rects, id, area) else {
            return;
        };
        self.scroll_offset = (content_y - screen_y_before).clamp(0.0, report.scroll_extent);
    }
}
