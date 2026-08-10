use crate::container::FocusPath;
use crate::geom::{Rect, Vec2};
use crate::layout::{InputClass, SizeClass, Viewport};

use super::{Pod, PodId, PodList};

impl PodList {
    /// Redistribute preferred size between the pod `above` and the next pod.
    /// Writes size-class-scoped [`crate::layout::Overrides`].
    pub fn apply_divider_drag(
        &mut self,
        above: PodId,
        delta: f32,
        area_height: f32,
        size_class: SizeClass,
    ) {
        let Some(idx) = self.pods.iter().position(|p| p.id == above) else {
            return;
        };
        if idx + 1 >= self.pods.len() {
            return;
        }
        if self.pods[idx].collapsed || self.pods[idx + 1].collapsed {
            return;
        }

        let area = Rect::from_xywh(0.0, 0.0, 400.0, area_height);
        let viewport = Viewport {
            size: Vec2::new(400.0, area_height),
            scale_factor: 1.0,
            size_class,
            input_class: InputClass::Pointer,
        };
        let (rects, _) = self.layout_with(area, &viewport, &FocusPath::default());
        let Some((_, above_rect)) = rects.iter().find(|(id, _)| *id == above) else {
            return;
        };
        let below_id = self.pods[idx + 1].id;
        let Some((_, below_rect)) = rects.iter().find(|(id, _)| *id == below_id) else {
            return;
        };

        let pair_px = above_rect.size.y + below_rect.size.y;
        let new_above = (above_rect.size.y + delta).clamp(
            self.pods[idx].min_height,
            (pair_px - self.pods[idx + 1].min_height).max(self.pods[idx].min_height),
        );
        let new_below = (pair_px - new_above).max(self.pods[idx + 1].min_height);

        let denom = area_height.max(1.0);
        self.overrides
            .set(Pod::container_id(above), size_class, new_above / denom);
        self.overrides
            .set(Pod::container_id(below_id), size_class, new_below / denom);

        self.pods[idx].height = new_above.max(0.01);
        self.pods[idx + 1].height = new_below.max(0.01);
        self.pods[idx].state.extent.ideal = new_above;
        self.pods[idx + 1].state.extent.ideal = new_below;
        self.pods[idx].state.extent.weight = new_above.max(0.01);
        self.pods[idx + 1].state.extent.weight = new_below.max(0.01);
    }

    /// Clear size-class overrides and equalize authored weights.
    pub fn equalize(&mut self, size_class: SizeClass) {
        for pod in &mut self.pods {
            if !pod.collapsed {
                pod.height = 1.0;
                pod.state.extent.weight = 1.0;
                pod.state.extent.ideal = 480.0_f32.max(pod.min_height);
                self.overrides.remove(pod.state.id, size_class);
            }
        }
    }
}
