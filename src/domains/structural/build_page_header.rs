use crate::domains::structural::templates::page_template_glyph;
use crate::results::parse_checks;
use hyper_ui::particles::{
    Particle, ParticleId, SourceParticle, StackParticle, SurfaceParticle, TriggerParticle,
};
use hyper_ui::{PageId, TemplateId};
use hypernode::{HyperNode, Node, PropValue};
use std::cmp::Ordering;
use std::collections::HashMap;

/// Analysis page header — type switcher + live result ratios + split trigger.
pub fn build_analysis_page_header(
    page_id: PageId,
    template_id: TemplateId,
    results: Option<&Node>,
    split_triggers: &mut HashMap<ParticleId, PageId>,
    template_menu_triggers: &mut HashMap<ParticleId, PageId>,
) -> (Particle, ParticleId) {
    let ratio_text = match results {
        Some(results) => {
            let overall = if matches!(
                results.get_prop("overall_pass"),
                Some(PropValue::Bool(true))
            ) {
                "PASS"
            } else {
                "FAIL"
            };
            let checks = parse_checks(results);
            let top = checks
                .iter()
                .filter(|c| !c.informational)
                .max_by(|a, b| a.ratio.partial_cmp(&b.ratio).unwrap_or(Ordering::Equal));
            match top {
                Some(c) => format!("{overall}  {}  {:.3}", c.name, c.ratio),
                None => format!("{overall}  —"),
            }
        }
        None => "No results".into(),
    };

    let status = SourceParticle::secondary(ratio_text);
    let status_id = status.id;

    let type_btn = TriggerParticle::new(page_template_glyph(template_id));
    template_menu_triggers.insert(type_btn.id, page_id);

    let split = TriggerParticle::new("⧉");
    split_triggers.insert(split.id, page_id);

    let row = StackParticle::row(vec![
        Particle::Trigger(type_btn),
        Particle::Source(status),
        Particle::Trigger(split),
    ])
    .with_gap(8.0)
    .with_align(hyper_ui::particles::StackAlign::Center);

    let particle = Particle::Surface(
        SurfaceParticle::new([0.15, 0.16, 0.19, 1.0])
            .with_padding(6.0)
            .with_radius(0.0)
            .with_border([0.28, 0.30, 0.34, 1.0], 1.0)
            .with_child(Particle::Stack(row)),
    );

    (particle, status_id)
}

/// Header with editor-type switcher + split trigger.
pub fn build_split_only_header(
    page_id: PageId,
    template_id: TemplateId,
    split_triggers: &mut HashMap<ParticleId, PageId>,
    template_menu_triggers: &mut HashMap<ParticleId, PageId>,
) -> Particle {
    let type_btn = TriggerParticle::new(page_template_glyph(template_id));
    template_menu_triggers.insert(type_btn.id, page_id);

    let split = TriggerParticle::new("⧉");
    split_triggers.insert(split.id, page_id);

    let row = StackParticle::row(vec![Particle::Trigger(type_btn), Particle::Trigger(split)])
        .with_gap(8.0)
        .with_align(hyper_ui::particles::StackAlign::Center);

    Particle::Surface(
        SurfaceParticle::new([0.15, 0.16, 0.19, 1.0])
            .with_padding(4.0)
            .with_radius(0.0)
            .with_child(Particle::Stack(row)),
    )
}
