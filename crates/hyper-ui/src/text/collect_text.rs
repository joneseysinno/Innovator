use crate::geom::{Rect, Vec2};
use crate::particles::{Particle, ParticleId};

use super::TextRenderer;

/// Collect all text-bearing particles into the text renderer.
pub fn collect_text(particle: &Particle, text: &mut TextRenderer, focused: Option<ParticleId>) {
    collect_text_clipped(particle, text, focused, None);
}

fn collect_text_clipped(
    particle: &Particle,
    text: &mut TextRenderer,
    focused: Option<ParticleId>,
    clip: Option<Rect>,
) {
    match particle {
        Particle::Surface(p) => {
            if let Some(child) = p.child.as_ref() {
                collect_text_clipped(child, text, focused, clip);
            }
        }
        Particle::Stack(p) => {
            for child in &p.children {
                collect_text_clipped(child, text, focused, clip);
            }
        }
        Particle::Slot(p) => {
            if let Some(child) = p.child.as_ref() {
                collect_text_clipped(child, text, focused, clip);
            }
        }
        Particle::Sink(p) => {
            if let Some(child) = p.child.as_ref() {
                collect_text_clipped(child, text, focused, clip);
            }
        }
        Particle::View(p) => {
            if let Some(child) = p.child.as_ref() {
                collect_text_clipped(child, text, focused, clip);
            }
        }
        Particle::Viewport(p) => {
            let vp_clip = Rect::new(p.layout.origin, p.layout.size);
            let child_clip = Some(match clip {
                Some(outer) => outer.intersect(&vp_clip),
                None => vp_clip,
            });
            if let Some(child) = p.child.as_ref() {
                collect_text_clipped(child, text, focused, child_clip);
            }
        }
        Particle::Source(p) => {
            let origin = Vec2::new(p.layout.origin.x, p.layout.origin.y + 2.0);
            let bounds = clip_to_bounds(clip);
            text.queue_text(
                &p.text,
                origin,
                p.font_size,
                p.weight,
                p.color(),
                bounds,
            );
        }
        Particle::Field(p) => {
            let origin = Vec2::new(p.layout.origin.x + 8.0, p.layout.origin.y + 10.0);
            let color = if p.read_only {
                [0.70, 0.72, 0.76, 1.0]
            } else if focused == Some(p.id) {
                [0.95, 0.96, 0.98, 1.0]
            } else {
                [0.88, 0.90, 0.93, 1.0]
            };
            let field_clip = Some((
                p.layout.origin.x as i32,
                p.layout.origin.y as i32,
                (p.layout.origin.x + p.layout.size.x) as i32,
                (p.layout.origin.y + p.layout.size.y) as i32,
            ));
            let bounds = match (clip_to_bounds(clip), field_clip) {
                (Some(a), Some(b)) => Some(intersect_bounds(a, b)),
                (Some(a), None) | (None, Some(a)) => Some(a),
                (None, None) => None,
            };
            text.queue_text(p.display_text(), origin, 14.0, 400, color, bounds);
        }
        Particle::Trigger(p) => {
            let char_w = 14.0 * 0.55;
            let text_w = p.label.chars().count() as f32 * char_w;
            let origin = Vec2::new(
                p.layout.origin.x + (p.layout.size.x - text_w) * 0.5,
                p.layout.origin.y + 10.0,
            );
            text.queue_text(
                &p.label,
                origin,
                14.0,
                500,
                [0.95, 0.96, 0.98, 1.0],
                clip_to_bounds(clip),
            );
        }
        Particle::Signal(_) => {}
    }
}

fn clip_to_bounds(clip: Option<Rect>) -> Option<(i32, i32, i32, i32)> {
    clip.filter(|r| !r.is_empty()).map(|r| {
        (
            r.origin.x as i32,
            r.origin.y as i32,
            (r.origin.x + r.size.x) as i32,
            (r.origin.y + r.size.y) as i32,
        )
    })
}

fn intersect_bounds(
    a: (i32, i32, i32, i32),
    b: (i32, i32, i32, i32),
) -> (i32, i32, i32, i32) {
    (a.0.max(b.0), a.1.max(b.1), a.2.min(b.2), a.3.min(b.3))
}
