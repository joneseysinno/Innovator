use super::StatusIO;
use crate::engine::AnalysisOutput;
use crate::workspace::signal::WorkspaceSignal;
use hyper_ui::particles::{
    Particle, SourceParticle, StackParticle, SurfaceParticle, TriggerParticle, ViewParticle,
};
use std::collections::HashMap;

/// Build StatusIO from the latest analysis output (or empty).
pub fn build_status(output: Option<&AnalysisOutput>) -> StatusIO {
    let mut triggers = HashMap::new();
    let title = SourceParticle::new("Status").with_weight(500);
    let mut children = vec![Particle::Source(title)];

    match output {
        None => {
            children.push(Particle::Source(SourceParticle::secondary(
                "No run yet",
            )));
            children.push(Particle::Source(SourceParticle::muted(
                "Code: ACI 318-19 (simplified)",
            )));
        }
        Some(out) => {
            let overall = if out.overall_pass { "PASS" } else { "FAIL" };
            let color = if out.overall_pass {
                [0.18, 0.42, 0.28, 1.0]
            } else {
                [0.50, 0.20, 0.20, 1.0]
            };
            children.push(Particle::Surface(
                SurfaceParticle::new(color)
                    .with_padding(8.0)
                    .with_radius(0.0)
                    .with_child(Particle::Source(
                        SourceParticle::new(format!("Overall: {overall}")).with_weight(500),
                    )),
            ));
            children.push(Particle::Source(SourceParticle::secondary(format!(
                "Governing: {}",
                out.governing
            ))));
            children.push(Particle::Source(SourceParticle::secondary(format!(
                "Run time: {}",
                out.run_timestamp
            ))));
            children.push(Particle::Source(SourceParticle::muted(
                "Code: ACI 318-19 (simplified)",
            )));
        }
    }

    let export = TriggerParticle::new("Export PDF");
    triggers.insert(export.id, WorkspaceSignal::Export);
    children.push(Particle::Trigger(export));

    let body = StackParticle::column(children).with_gap(8.0);
    let surface = SurfaceParticle::new([0.12, 0.13, 0.16, 1.0])
        .with_padding(10.0)
        .with_radius(0.0)
        .with_border([0.26, 0.28, 0.32, 1.0], 1.0)
        .with_child(Particle::Stack(body));

    let mut view = ViewParticle::new("results_status");
    view.child = Some(Box::new(Particle::Surface(surface)));

    StatusIO {
        particle: Particle::View(view),
        triggers,
    }
}
