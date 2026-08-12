use super::wall_list::build_wall_list;
use super::wall_summary::build_wall_summary;
use crate::domains::structural::StructuralWorkspace;
use hyper_ui::particles::{Particle, StackParticle};
use hypernode::Graph;

/// Build the Navigation page (WallList | WallSummary) and wire interaction maps.
pub fn build_navigation(ws: &mut StructuralWorkspace, graph: &Graph) -> Particle {
    let list = build_wall_list(graph, ws.active_wall);
    let summary = build_wall_summary(graph, ws.active_wall);

    ws.wall_sinks = list.sinks;
    ws.nav_triggers = list.triggers;

    Particle::Stack(
        StackParticle::column(vec![list.particle, summary.particle]).with_gap(0.0),
    )
}
