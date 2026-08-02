use super::{PodId, PodList};

impl PodList {
    /// Redistribute preferred height between the pod `above` and the next pod.
    pub fn apply_divider_drag(&mut self, above: PodId, delta: f32, area_height: f32) {
        let Some(idx) = self.pods.iter().position(|p| p.id == above) else {
            return;
        };
        if idx + 1 >= self.pods.len() {
            return;
        }
        if self.pods[idx].collapsed || self.pods[idx + 1].collapsed {
            return;
        }

        let n = self.pods.len();
        let gap_total = self.gap * (n.saturating_sub(1) as f32);
        let collapsed_h: f32 = self
            .pods
            .iter()
            .filter(|p| p.collapsed)
            .map(|_| super::pod::COLLAPSED_HEIGHT)
            .sum();
        let available = (area_height - gap_total - collapsed_h).max(1.0);

        let flex_sum: f32 = self
            .pods
            .iter()
            .filter(|p| !p.collapsed)
            .map(|p| p.height.max(0.01))
            .sum::<f32>()
            .max(0.01);

        let above_flex = self.pods[idx].height.max(0.01);
        let below_flex = self.pods[idx + 1].height.max(0.01);
        let pair_flex = above_flex + below_flex;
        let pair_px = available * (pair_flex / flex_sum);

        let new_above = (pair_px * (above_flex / pair_flex) + delta).clamp(
            self.pods[idx].min_height,
            (pair_px - self.pods[idx + 1].min_height).max(self.pods[idx].min_height),
        );
        let new_below = (pair_px - new_above).max(self.pods[idx + 1].min_height);

        self.pods[idx].height = new_above.max(0.01);
        self.pods[idx + 1].height = new_below.max(0.01);
    }

    /// Equalize flex weights of all expanded pods.
    pub fn equalize(&mut self) {
        for pod in &mut self.pods {
            if !pod.collapsed {
                pod.height = 1.0;
            }
        }
    }
}
