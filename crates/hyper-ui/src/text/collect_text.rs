use crate::geom::Vec2;
use crate::particles::{Particle, ParticleId};

use super::TextRenderer;

/// Collect all text-bearing particles into the text renderer.
pub fn collect_text(particle: &Particle, text: &mut TextRenderer, focused: Option<ParticleId>) {
    match particle {
        Particle::Surface(p) => {
            if let Some(child) = p.child.as_ref() {
                collect_text(child, text, focused);
            }
        }
        Particle::Stack(p) => {
            for child in &p.children {
                collect_text(child, text, focused);
            }
        }
        Particle::Slot(p) => {
            if let Some(child) = p.child.as_ref() {
                collect_text(child, text, focused);
            }
        }
        Particle::Sink(p) => {
            if let Some(child) = p.child.as_ref() {
                collect_text(child, text, focused);
            }
        }
        Particle::View(p) => {
            if let Some(child) = p.child.as_ref() {
                collect_text(child, text, focused);
            }
        }
        Particle::Source(p) => {
            let origin = Vec2::new(p.layout.origin.x, p.layout.origin.y + 2.0);
            text.queue_source(&p.text, origin, p.style, p.font_size, p.weight);
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
            text.queue_text(
                p.display_text(),
                origin,
                14.0,
                400,
                color,
                Some((
                    p.layout.origin.x as i32,
                    p.layout.origin.y as i32,
                    (p.layout.origin.x + p.layout.size.x) as i32,
                    (p.layout.origin.y + p.layout.size.y) as i32,
                )),
            );
        }
        Particle::Trigger(p) => {
            let char_w = 14.0 * 0.55;
            let text_w = p.label.chars().count() as f32 * char_w;
            let origin = Vec2::new(
                p.layout.origin.x + (p.layout.size.x - text_w) * 0.5,
                p.layout.origin.y + 10.0,
            );
            text.queue_text(&p.label, origin, 14.0, 500, [0.95, 0.96, 0.98, 1.0], None);
        }
        Particle::Signal(_) => {}
    }
}
