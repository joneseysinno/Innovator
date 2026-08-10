use crate::container::ContainerId;
use crate::geom::Vec2;
use crate::layout::{Axis, LayoutBox};
use crate::particles::{Particle, ParticleId};

/// Scroll viewport — clips its child and offsets it along one axis.
///
/// `offset` is transient: not persisted, not synced. Reset on tree rebuild
/// unless the host re-applies an anchored offset after rebuild.
#[derive(Debug, Clone)]
pub struct ViewportParticle {
    pub id: ParticleId,
    pub layout: LayoutBox,
    pub child: Option<Box<Particle>>,
    /// Scroll offset along [`Self::axis`] (content px scrolled out of view).
    pub offset: f32,
    /// Measured child extent along the scroll axis.
    pub content_extent: f32,
    pub axis: Axis,
    /// Content-space positions for [`Self::scroll_to_container`].
    /// Host rebuilds these when the child tree is laid out (Phase 6+).
    pub anchors: Vec<(ContainerId, f32)>,
}

impl ViewportParticle {
    pub fn new() -> Self {
        Self {
            id: ParticleId::fresh(),
            layout: LayoutBox::default(),
            child: None,
            offset: 0.0,
            content_extent: 0.0,
            axis: Axis::Vertical,
            anchors: Vec::new(),
        }
    }

    pub fn with_child(mut self, child: Particle) -> Self {
        self.child = Some(Box::new(child));
        self
    }

    pub fn with_axis(mut self, axis: Axis) -> Self {
        self.axis = axis;
        self
    }

    /// Viewport extent along the scroll axis.
    pub fn viewport_extent(&self) -> f32 {
        match self.axis {
            Axis::Horizontal => self.layout.size.x,
            Axis::Vertical => self.layout.size.y,
        }
    }

    /// Maximum legal offset: `max(0, content_extent - viewport_extent)`.
    pub fn max_offset(&self) -> f32 {
        (self.content_extent - self.viewport_extent()).max(0.0)
    }

    pub fn clamp_offset(&mut self) {
        let max = self.max_offset();
        self.offset = self.offset.clamp(0.0, max);
    }

    /// Scroll by `delta` (positive → reveals content further along the axis).
    pub fn scroll_by(&mut self, delta: f32) {
        self.offset += delta;
        self.clamp_offset();
    }

    /// Scroll so `content_offset` aligns with the start of the viewport.
    pub fn scroll_to(&mut self, content_offset: f32) {
        self.offset = content_offset;
        self.clamp_offset();
    }

    /// Scroll so the anchored container sits at the start of the viewport.
    pub fn scroll_to_container(&mut self, id: ContainerId) -> bool {
        let Some((_, content_offset)) = self.anchors.iter().find(|(c, _)| *c == id) else {
            return false;
        };
        self.scroll_to(*content_offset);
        true
    }

    pub fn set_anchors(&mut self, anchors: Vec<(ContainerId, f32)>) {
        self.anchors = anchors;
    }

    /// Greedy measure — fills the assigned slot like [`super::ViewParticle`].
    pub fn measure(&self, available: Vec2) -> Vec2 {
        Vec2::new(available.x.max(0.0), available.y.max(0.0))
    }
}

impl Default for ViewportParticle {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::container::ContainerId;
    use crate::geom::{Rect, Vec2};
    use crate::layout::arrange_particle;
    use crate::particles::Particle;

    fn ten_pod_viewport(viewport_h: f32, pod_h: f32) -> ViewportParticle {
        let anchors: Vec<(ContainerId, f32)> = (0..10)
            .map(|i| (ContainerId(i), i as f32 * pod_h))
            .collect();
        let mut vp = ViewportParticle::new();
        vp.layout = LayoutBox {
            origin: Vec2::ZERO,
            size: Vec2::new(300.0, viewport_h),
        };
        vp.content_extent = 10.0 * pod_h;
        vp.set_anchors(anchors);
        vp
    }

    #[test]
    fn clamp_at_both_ends() {
        let mut vp = ten_pod_viewport(390.0, 120.0);
        assert_eq!(vp.max_offset(), 10.0 * 120.0 - 390.0);

        vp.offset = -50.0;
        vp.clamp_offset();
        assert_eq!(vp.offset, 0.0);

        vp.offset = 99999.0;
        vp.clamp_offset();
        assert_eq!(vp.offset, vp.max_offset());
    }

    #[test]
    fn scroll_to_lands_each_pod_at_top() {
        let pod_h = 120.0;
        let mut vp = ten_pod_viewport(390.0, pod_h);
        let ids: Vec<ContainerId> = (0..10).map(ContainerId).collect();
        for (i, id) in ids.iter().enumerate() {
            assert!(vp.scroll_to_container(*id));
            let expected = (i as f32 * pod_h).min(vp.max_offset());
            assert!(
                (vp.offset - expected).abs() < 0.01,
                "pod {i}: offset {} vs {expected}",
                vp.offset
            );
        }
    }

    #[test]
    fn scroll_by_clamps() {
        let mut vp = ten_pod_viewport(390.0, 120.0);
        vp.scroll_by(-100.0);
        assert_eq!(vp.offset, 0.0);
        vp.scroll_by(vp.max_offset() + 50.0);
        assert_eq!(vp.offset, vp.max_offset());
    }

    #[test]
    fn arrange_offsets_child_vertically() {
        use crate::particles::{SourceParticle, StackParticle};

        let children: Vec<Particle> = (0..20)
            .map(|i| Particle::Source(SourceParticle::new(format!("line {i}"))))
            .collect();
        let stack = Particle::Stack(StackParticle::column(children).with_gap(4.0));
        let mut vp = ViewportParticle::new().with_child(stack);
        vp.offset = 40.0;

        let mut particle = Particle::Viewport(vp);
        arrange_particle(&mut particle, Rect::from_xywh(10.0, 20.0, 200.0, 300.0));

        let Particle::Viewport(vp) = &particle else {
            panic!("expected viewport");
        };
        assert_eq!(vp.layout.origin, Vec2::new(10.0, 20.0));
        assert_eq!(vp.layout.size, Vec2::new(200.0, 300.0));
        assert!(vp.content_extent > 300.0, "content_extent={}", vp.content_extent);
        assert!((vp.offset - 40.0).abs() < 0.01, "offset clamped to {}", vp.offset);
        let child = vp.child.as_ref().unwrap();
        assert!(
            (child.layout().origin.y - (20.0 - 40.0)).abs() < 0.01,
            "child y = {}",
            child.layout().origin.y
        );
    }
}