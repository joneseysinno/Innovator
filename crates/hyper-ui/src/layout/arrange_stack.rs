use crate::geom::{Rect, Vec2};
use crate::particles::{Particle, StackAlign, StackDirection, StackParticle};

use super::{arrange_particle, measure_particle};

pub(super) fn arrange_stack(stack: &mut StackParticle, rect: Rect) {
    if stack.children.is_empty() {
        return;
    }
    let gaps = stack.gap * (stack.children.len().saturating_sub(1) as f32);

    match stack.direction {
        StackDirection::Row => {
            let child_avail_w = if rect.size.x.is_finite() && rect.size.x > 0.0 {
                rect.size.x
            } else {
                400.0
            };
            let sizes: Vec<Vec2> = stack
                .children
                .iter()
                .map(|c| measure_particle(c, Vec2::new(child_avail_w, rect.size.y)))
                .collect();
            let total_fixed: f32 = sizes.iter().map(|s| s.x).sum();
            let flex_extra = (rect.size.x - gaps - total_fixed).max(0.0);
            // Distribute leftover to fields (flex) — simple equal share among children with flex.
            let flex_count = stack
                .children
                .iter()
                .filter(|c| matches!(c, Particle::Field(f) if f.flex > 0.0))
                .count()
                .max(1);
            let extra_each = flex_extra / flex_count as f32;

            let mut x = rect.origin.x;
            for (i, child) in stack.children.iter_mut().enumerate() {
                let mut sz = sizes[i];
                if matches!(child, Particle::Field(f) if f.flex > 0.0) {
                    sz.x += extra_each;
                }
                let y = match stack.align {
                    StackAlign::Start => rect.origin.y,
                    StackAlign::Center => rect.origin.y + (rect.size.y - sz.y) * 0.5,
                    StackAlign::End => rect.origin.y + rect.size.y - sz.y,
                    StackAlign::Stretch => rect.origin.y,
                };
                let h = if stack.align == StackAlign::Stretch {
                    rect.size.y
                } else {
                    sz.y
                };
                arrange_particle(child, Rect::from_xywh(x, y, sz.x, h));
                x += sz.x + stack.gap;
            }
        }
        StackDirection::Column => {
            let mut y = rect.origin.y;
            let count = stack.children.len();
            for (i, child) in stack.children.iter_mut().enumerate() {
                let remaining = (rect.size.y - (y - rect.origin.y)).max(0.0);
                let is_view = matches!(child, Particle::View(_));
                let avail_h = if is_view {
                    // give remaining space to views
                    let after = count - i - 1;
                    let reserve = after as f32 * 40.0;
                    (remaining - reserve).max(40.0)
                } else {
                    10_000.0
                };
                let sz = measure_particle(child, Vec2::new(rect.size.x, avail_h));
                let h = if is_view { avail_h } else { sz.y };
                let w = match stack.align {
                    StackAlign::Stretch => rect.size.x,
                    _ => sz.x.min(rect.size.x),
                };
                let x = match stack.align {
                    StackAlign::Start | StackAlign::Stretch => rect.origin.x,
                    StackAlign::Center => rect.origin.x + (rect.size.x - w) * 0.5,
                    StackAlign::End => rect.origin.x + rect.size.x - w,
                };
                arrange_particle(child, Rect::from_xywh(x, y, w, h));
                y += h + stack.gap;
            }
        }
    }
}
