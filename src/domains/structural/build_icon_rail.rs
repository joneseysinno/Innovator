use hyper_ui::particles::{Particle, StackParticle, SurfaceParticle, TriggerParticle};
use hyper_ui::{PageId, PageNode, ParticleId, PodId};
use std::collections::HashMap;

/// Build an icon rail column — one trigger per pod, vertical order.
pub fn build_icon_rail(
    page: &PageNode,
    pod_icons: &[(PodId, &'static str)],
    triggers: &mut HashMap<ParticleId, (PageId, PodId)>,
) -> Particle {
    let mut items = Vec::with_capacity(pod_icons.len());
    for (pod_id, glyph) in pod_icons {
        let t = TriggerParticle::new(*glyph);
        triggers.insert(t.id, (page.id, *pod_id));
        items.push(Particle::Trigger(t));
    }

    let column = StackParticle::column(items).with_gap(4.0);
    Particle::Surface(
        SurfaceParticle::new([0.14, 0.15, 0.18, 1.0])
            .with_padding(2.0)
            .with_radius(0.0)
            .with_child(Particle::Stack(column)),
    )
}

/// Default glyphs for known IO roles (by pod id order).
pub fn default_pod_icons(pod_count: usize) -> Vec<(PodId, &'static str)> {
    const GLYPHS: &[&str] = &["≡", "▣", "▤", "▥", "▦", "▧"];
    (0..pod_count as u32)
        .map(|id| (PodId(id), GLYPHS.get(id as usize).copied().unwrap_or("•")))
        .collect()
}
