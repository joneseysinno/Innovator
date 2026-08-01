use crate::particles::{Particle, ParticleId};

pub(crate) fn collect_tab_order(particle: &Particle) -> Vec<ParticleId> {
    let mut out = Vec::new();
    collect_tab_order_inner(particle, &mut out);
    out
}

fn collect_tab_order_inner(particle: &Particle, out: &mut Vec<ParticleId>) {
    match particle {
        Particle::Field(f) if !f.read_only => out.push(f.id),
        Particle::Surface(p) => {
            if let Some(c) = p.child.as_ref() {
                collect_tab_order_inner(c, out);
            }
        }
        Particle::Stack(p) => {
            for c in &p.children {
                collect_tab_order_inner(c, out);
            }
        }
        Particle::Slot(p) => {
            if let Some(c) = p.child.as_ref() {
                collect_tab_order_inner(c, out);
            }
        }
        Particle::Sink(p) => {
            if let Some(c) = p.child.as_ref() {
                collect_tab_order_inner(c, out);
            }
        }
        Particle::View(p) => {
            if let Some(c) = p.child.as_ref() {
                collect_tab_order_inner(c, out);
            }
        }
        _ => {}
    }
}
