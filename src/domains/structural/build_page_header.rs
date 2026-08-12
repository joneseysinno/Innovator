use crate::results::parse_checks;
use hyper_ui::particles::{
    Particle, ParticleId, SourceParticle, StackParticle, SurfaceParticle, TriggerParticle,
};
use hyper_ui::PageId;
use hypernode::{HyperNode, Node, PropValue};
use std::cmp::Ordering;
use std::collections::HashMap;

/// Analysis page header — live result ratios + split trigger.
pub fn build_analysis_page_header(
    page_id: PageId,
    results: Option<&Node>,
    split_triggers: &mut HashMap<ParticleId, PageId>,
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

    let split = TriggerParticle::new("⧉");
    split_triggers.insert(split.id, page_id);

    let row = StackParticle::row(vec![Particle::Source(status), Particle::Trigger(split)])
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

/// Minimal header with only a split trigger (for empty / generic pages).
pub fn build_split_only_header(
    page_id: PageId,
    split_triggers: &mut HashMap<ParticleId, PageId>,
) -> Particle {
    let split = TriggerParticle::new("⧉");
    split_triggers.insert(split.id, page_id);
    let row = StackParticle::row(vec![Particle::Trigger(split)]).with_gap(0.0);
    Particle::Surface(
        SurfaceParticle::new([0.15, 0.16, 0.19, 1.0])
            .with_padding(4.0)
            .with_radius(0.0)
            .with_child(Particle::Stack(row)),
    )
}
