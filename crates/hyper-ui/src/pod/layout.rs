use crate::geom::Rect;

use super::pod::COLLAPSED_HEIGHT;
use super::{PodId, PodList};

impl PodList {
    /// Compute `(PodId, Rect)` for every pod given the available content rect.
    /// Collapsed pods receive only their title-bar height.
    pub fn layout(&self, area: Rect) -> Vec<(PodId, Rect)> {
        if self.pods.is_empty() {
            return Vec::new();
        }

        let n = self.pods.len();
        let gap_total = self.gap * (n.saturating_sub(1) as f32);
        let collapsed_h: f32 = self
            .pods
            .iter()
            .filter(|p| p.collapsed)
            .map(|_| COLLAPSED_HEIGHT)
            .sum();
        let flex_sum: f32 = self
            .pods
            .iter()
            .filter(|p| !p.collapsed)
            .map(|p| p.height.max(0.01))
            .sum();
        let expanded = self.pods.iter().filter(|p| !p.collapsed).count();
        let available = (area.size.y - gap_total - collapsed_h).max(0.0);

        let mut heights = Vec::with_capacity(n);
        if expanded == 0 {
            for pod in &self.pods {
                heights.push(if pod.collapsed {
                    COLLAPSED_HEIGHT
                } else {
                    0.0
                });
            }
        } else {
            let mut used = 0.0_f32;
            for pod in &self.pods {
                if pod.collapsed {
                    heights.push(COLLAPSED_HEIGHT);
                } else {
                    let h = (available * (pod.height.max(0.01) / flex_sum)).max(pod.min_height);
                    heights.push(h);
                    used += h;
                }
            }
            if used > available && available > 0.0 {
                let scale = available / used;
                for (i, pod) in self.pods.iter().enumerate() {
                    if !pod.collapsed {
                        heights[i] *= scale;
                    }
                }
            } else if used < available {
                let leftover = available - used;
                for (i, pod) in self.pods.iter().enumerate() {
                    if !pod.collapsed {
                        heights[i] += leftover * (pod.height.max(0.01) / flex_sum);
                    }
                }
            }
        }

        let mut out = Vec::with_capacity(n);
        let mut y = area.origin.y;
        for (i, pod) in self.pods.iter().enumerate() {
            let h = heights[i].max(0.0);
            out.push((
                pod.id,
                Rect::from_xywh(area.origin.x, y, area.size.x, h),
            ));
            y += h;
            if i + 1 < n {
                y += self.gap;
            }
        }
        out
    }
}
