use hyper_ui::particles::{Particle, StackParticle, SurfaceParticle, TriggerParticle};
use hyper_ui::{PageId, PageNode, ParticleId};
use std::collections::HashMap;

/// Build an icon rail column — one trigger per pod leaf, vertical order.
pub fn build_icon_rail(
    page: &PageNode,
    pod_icons: &[(u32, &'static str)],
    triggers: &mut HashMap<ParticleId, (PageId, u32)>,
) -> Particle {
    let mut items = Vec::with_capacity(pod_icons.len());
    for (leaf_id, glyph) in pod_icons {
        let t = TriggerParticle::new(*glyph);
        triggers.insert(t.id, (page.id, *leaf_id));
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

/// Default glyphs for known IO leaf roles (by leaf id order).
pub fn default_pod_icons(leaf_count: usize) -> Vec<(u32, &'static str)> {
    const GLYPHS: &[&str] = &["≡", "▣", "▤", "▥", "▦", "▧"];
    (0..leaf_count as u32)
        .map(|id| (id, GLYPHS.get(id as usize).copied().unwrap_or("•")))
        .collect()
}
