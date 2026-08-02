use super::results_table::build_results_table;
use super::status::build_status;
use crate::results::parse_checks;
use crate::domains::structural::StructuralWorkspace;
use hyper_ui::particles::{Particle, StackParticle};

/// Build the Results page (table | status) and wire export triggers.
pub fn build_results(ws: &mut StructuralWorkspace) -> Particle {
    let checks = ws
        .last_results
        .as_ref()
        .map(|n| parse_checks(n))
        .unwrap_or_default();
    let table = build_results_table(&checks);
    let status = build_status(ws.last_analysis.as_ref());

    ws.results_triggers = status.triggers;

    Particle::Stack(
        StackParticle::column(vec![table.particle, status.particle]).with_gap(0.0),
    )
}
