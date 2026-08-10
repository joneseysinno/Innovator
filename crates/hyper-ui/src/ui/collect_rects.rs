use crate::geom::Rect;
use crate::particles::{Particle, ParticleId};
use crate::renderer::node_pipeline::{NodeInstance, NodePipeline};

pub(crate) fn collect_rects(
    particle: &Particle,
    rects: &mut NodePipeline,
    focused: Option<ParticleId>,
) {
    collect_rects_clipped(particle, rects, focused, None);
}

fn collect_rects_clipped(
    particle: &Particle,
    rects: &mut NodePipeline,
    focused: Option<ParticleId>,
    clip: Option<Rect>,
) {
    match particle {
        Particle::Surface(p) => {
            push_clipped(
                rects,
                NodeInstance {
                    position: [p.layout.origin.x, p.layout.origin.y],
                    size: [p.layout.size.x, p.layout.size.y],
                    color: p.color,
                    border_color: p.border_color,
                    border_radius: p.border_radius,
                    border_width: p.border_width,
                    _pad: [0.0; 2],
                },
                clip,
            );
            if let Some(child) = p.child.as_ref() {
                collect_rects_clipped(child, rects, focused, clip);
            }
        }
        Particle::Stack(p) => {
            for child in &p.children {
                collect_rects_clipped(child, rects, focused, clip);
            }
        }
        Particle::Slot(p) => {
            if let Some(child) = p.child.as_ref() {
                collect_rects_clipped(child, rects, focused, clip);
            }
        }
        Particle::Sink(p) => {
            if let Some(child) = p.child.as_ref() {
                collect_rects_clipped(child, rects, focused, clip);
            }
        }
        Particle::View(p) => {
            push_clipped(
                rects,
                NodeInstance {
                    position: [p.layout.origin.x, p.layout.origin.y],
                    size: [p.layout.size.x, p.layout.size.y],
                    color: [0.12, 0.13, 0.16, 1.0],
                    border_color: [0.28, 0.30, 0.34, 1.0],
                    border_radius: 4.0,
                    border_width: 1.0,
                    _pad: [0.0; 2],
                },
                clip,
            );
            if let Some(child) = p.child.as_ref() {
                collect_rects_clipped(child, rects, focused, clip);
            }
        }
        Particle::Viewport(p) => {
            let vp_clip = Rect::new(p.layout.origin, p.layout.size);
            let child_clip = Some(match clip {
                Some(outer) => outer.intersect(&vp_clip),
                None => vp_clip,
            });
            if let Some(child) = p.child.as_ref() {
                collect_rects_clipped(child, rects, focused, child_clip);
            }
        }
        Particle::Field(p) => {
            if p.read_only {
                return;
            }
            let focused_here = focused == Some(p.id);
            push_clipped(
                rects,
                NodeInstance {
                    position: [p.layout.origin.x, p.layout.origin.y],
                    size: [p.layout.size.x, p.layout.size.y],
                    color: p.background_color(focused_here),
                    border_color: [0.32, 0.34, 0.38, 1.0],
                    border_radius: 4.0,
                    border_width: 1.0,
                    _pad: [0.0; 2],
                },
                clip,
            );
        }
        Particle::Trigger(p) => {
            push_clipped(
                rects,
                NodeInstance {
                    position: [p.layout.origin.x, p.layout.origin.y],
                    size: [p.layout.size.x, p.layout.size.y],
                    color: p.color(),
                    border_color: [0.0; 4],
                    border_radius: 4.0,
                    border_width: 0.0,
                    _pad: [0.0; 2],
                },
                clip,
            );
        }
        Particle::Source(_) | Particle::Signal(_) => {}
    }
}

fn push_clipped(rects: &mut NodePipeline, instance: NodeInstance, clip: Option<Rect>) {
    let Some(clip) = clip else {
        rects.push(instance);
        return;
    };
    let rect = Rect::from_xywh(
        instance.position[0],
        instance.position[1],
        instance.size[0],
        instance.size[1],
    );
    let hit = rect.intersect(&clip);
    if hit.is_empty() {
        return;
    }
    rects.push(NodeInstance {
        position: [hit.origin.x, hit.origin.y],
        size: [hit.size.x, hit.size.y],
        ..instance
    });
}
