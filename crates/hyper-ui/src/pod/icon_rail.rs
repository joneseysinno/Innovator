//! Page icon rail built from pods that opted in via [`super::Pod::nav_icon`].

use crate::page::{IconRailConfig, IconRailSide, PageId, PageNode};
use crate::particles::{Particle, ParticleId, StackParticle, SurfaceParticle, TriggerParticle};
use crate::pod::PodId;

use super::shell::pod_nav_icons;

/// Build an icon rail column — one trigger per pod that has a `nav_icon`.
///
/// Returns `None` when no pods opted in.
pub fn build_pod_icon_rail(
    page: &PageNode,
    triggers: &mut std::collections::HashMap<ParticleId, (PageId, PodId)>,
) -> Option<Particle> {
    let icons = pod_nav_icons(&page.pods.pods);
    if icons.is_empty() {
        return None;
    }

    let mut items = Vec::with_capacity(icons.len());
    for (pod_id, glyph) in &icons {
        let t = TriggerParticle::new(glyph.clone());
        triggers.insert(t.id, (page.id, *pod_id));
        items.push(Particle::Trigger(t));
    }

    let column = StackParticle::column(items).with_gap(4.0);
    Some(Particle::Surface(
        SurfaceParticle::new([0.14, 0.15, 0.18, 1.0])
            .with_padding(2.0)
            .with_radius(0.0)
            .with_child(Particle::Stack(column)),
    ))
}

/// Default rail geometry when a page has opted-in pod icons but no explicit config.
pub fn default_icon_rail_config() -> IconRailConfig {
    IconRailConfig {
        side: IconRailSide::Left,
        width: 34.0,
    }
}

/// Effective rail config: explicit page config if present, else default when any
/// pod has a nav icon, else `None`.
pub fn effective_icon_rail(page: &PageNode) -> Option<IconRailConfig> {
    if pod_nav_icons(&page.pods.pods).is_empty() {
        return None;
    }
    Some(
        page.icon_rail
            .clone()
            .unwrap_or_else(default_icon_rail_config),
    )
}
