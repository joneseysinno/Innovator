use crate::particles::{Particle, ParticleId};
use crate::renderer::node_pipeline::{NodeInstance, NodePipeline};

pub(crate) fn collect_rects(
    particle: &Particle,
    rects: &mut NodePipeline,
    focused: Option<ParticleId>,
) {
    match particle {
        Particle::Surface(p) => {
            rects.push(NodeInstance {
                position: [p.layout.origin.x, p.layout.origin.y],
                size: [p.layout.size.x, p.layout.size.y],
                color: p.color,
                border_color: p.border_color,
                border_radius: p.border_radius,
                border_width: p.border_width,
                _pad: [0.0; 2],
            });
            if let Some(child) = p.child.as_ref() {
                collect_rects(child, rects, focused);
            }
        }
        Particle::Stack(p) => {
            for child in &p.children {
                collect_rects(child, rects, focused);
            }
        }
        Particle::Slot(p) => {
            if let Some(child) = p.child.as_ref() {
                collect_rects(child, rects, focused);
            }
        }
        Particle::Sink(p) => {
            if let Some(child) = p.child.as_ref() {
                collect_rects(child, rects, focused);
            }
        }
        Particle::View(p) => {
            // Subtle fill so the view region is visible
            rects.push(NodeInstance {
                position: [p.layout.origin.x, p.layout.origin.y],
                size: [p.layout.size.x, p.layout.size.y],
                color: [0.12, 0.13, 0.16, 1.0],
                border_color: [0.28, 0.30, 0.34, 1.0],
                border_radius: 4.0,
                border_width: 1.0,
                _pad: [0.0; 2],
            });
            if let Some(child) = p.child.as_ref() {
                collect_rects(child, rects, focused);
            }
        }
        Particle::Field(p) => {
            if p.read_only {
                return;
            }
            let focused_here = focused == Some(p.id);
            rects.push(NodeInstance {
                position: [p.layout.origin.x, p.layout.origin.y],
                size: [p.layout.size.x, p.layout.size.y],
                color: p.background_color(focused_here),
                border_color: [0.32, 0.34, 0.38, 1.0],
                border_radius: 4.0,
                border_width: 1.0,
                _pad: [0.0; 2],
            });
        }
        Particle::Trigger(p) => {
            rects.push(NodeInstance {
                position: [p.layout.origin.x, p.layout.origin.y],
                size: [p.layout.size.x, p.layout.size.y],
                color: p.color(),
                border_color: [0.0; 4],
                border_radius: 4.0,
                border_width: 0.0,
                _pad: [0.0; 2],
            });
        }
        Particle::Source(_) | Particle::Signal(_) => {}
    }
}
