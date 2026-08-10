use crate::container::{FocusPath, Visibility};
use crate::geom::{Rect, Vec2};
use crate::layout::{
    resolve, Axis, InputClass, Overflow, Overrides, ResolveReport, SizeClass, Viewport, PAGE_LADDER,
};

use super::{PageId, PageNode, PageTree};

impl PageTree {
    /// Resolve all leaves against `area` with [`Overflow::Cascade`] / [`PAGE_LADDER`].
    ///
    /// Writes `resolved`/`rect` onto each leaf's [`crate::container::ContainerState`].
    /// Returns rects for **Shown** leaves only (Hidden omitted from the spatial row).
    pub fn layout(
        &mut self,
        area: Rect,
        focus: &FocusPath,
        overrides: &Overrides,
        viewport: &Viewport,
    ) -> (Vec<(PageId, Rect)>, ResolveReport) {
        let mut leaves = self.leaves_mut();
        if leaves.is_empty() {
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

        let mut states: Vec<_> = leaves.iter().map(|p| p.state.clone()).collect();
        let report = resolve(
            &mut states,
            area.size.x,
            area.size.y,
            Axis::Horizontal,
            Overflow::Cascade,
            PAGE_LADDER,
            1,
            focus,
            overrides,
            viewport,
        );

        for (leaf, state) in leaves.iter_mut().zip(states.iter()) {
            leaf.state = state.clone();
        }

        let mut out = Vec::new();
        let mut x = area.origin.x;
        for (leaf, state) in leaves.iter().zip(states.iter()) {
            if state.resolved() != Visibility::Shown {
                continue;
            }
            let w = state.rect().size.x;
            out.push((
                leaf.id,
                Rect::from_xywh(x, area.origin.y, w, area.size.y),
            ));
            x += w;
        }
        (out, report)
    }

    /// Immutable layout: clones, resolves, returns Shown rects (does not write state).
    pub fn layout_rects(
        &self,
        area: Rect,
        focus: &FocusPath,
        overrides: &Overrides,
    ) -> Vec<(PageId, Rect)> {
        let mut clone = self.clone();
        let viewport = Viewport {
            size: Vec2::new(area.size.x, area.size.y),
            scale_factor: 1.0,
            size_class: SizeClass::from_width(area.size.x.max(1.0)),
            input_class: InputClass::Pointer,
        };
        clone.layout(area, focus, overrides, &viewport).0
    }

    /// Shown leaf rects from the last [`Self::layout`] write, offset into `area`.
    ///
    /// If layout has not run, falls back to equal-width packing of all leaves.
    pub fn leaf_rects(&self, area: Rect) -> Vec<(PageId, Rect)> {
        let leaves = self.leaves();
        if leaves.is_empty() {
            return Vec::new();
        }

        let shown: Vec<_> = leaves
            .iter()
            .filter(|p| p.state.resolved() == Visibility::Shown)
            .collect();

        if shown.is_empty() {
            // Floor should prevent this; equal-pack as emergency.
            let w = area.size.x / leaves.len() as f32;
            return leaves
                .iter()
                .enumerate()
                .map(|(i, p)| {
                    (
                        p.id,
                        Rect::from_xywh(
                            area.origin.x + i as f32 * w,
                            area.origin.y,
                            w,
                            area.size.y,
                        ),
                    )
                })
                .collect();
        }

        // Prefer resolved rect widths when they sum sensibly.
        let widths: Vec<f32> = shown
            .iter()
            .map(|p| {
                let w = p.state.rect().size.x;
                if w > 1.0 {
                    w
                } else {
                    p.state.extent.min.max(1.0)
                }
            })
            .collect();
        let sum: f32 = widths.iter().sum();
        let scale = if sum > 0.1 {
            area.size.x / sum
        } else {
            1.0
        };

        let mut out = Vec::with_capacity(shown.len());
        let mut x = area.origin.x;
        for (p, w) in shown.iter().zip(widths.iter()) {
            let width = w * scale;
            out.push((
                p.id,
                Rect::from_xywh(x, area.origin.y, width, area.size.y),
            ));
            x += width;
        }
        out
    }

    /// All leaves including Hidden (for rails / focus).
    pub fn all_leaf_ids(&self) -> Vec<PageId> {
        self.leaves().iter().map(|p| p.id).collect()
    }

    pub fn hidden_leaves(&self) -> Vec<&PageNode> {
        self.leaves()
            .into_iter()
            .filter(|p| p.state.resolved() == Visibility::Hidden)
            .collect()
    }

    pub fn shown_leaves(&self) -> Vec<&PageNode> {
        self.leaves()
            .into_iter()
            .filter(|p| p.state.resolved() == Visibility::Shown)
            .collect()
    }
}
