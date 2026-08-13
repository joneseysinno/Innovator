use super::workspace::HomeWorkspace;
use crate::workspace::seed;
use hyper_ui::particles::{
    Particle, SourceParticle, StackParticle, SurfaceParticle, TriggerParticle, ViewParticle,
};
use std::collections::HashMap;

/// Build the Home dashboard — one OpenWorkspace trigger per launchable seed.
pub fn build_content(ws: &mut HomeWorkspace) -> Particle {
    let mut launcher_triggers = HashMap::new();

    let title = SourceParticle::new("Innovator").with_weight(500);
    let subtitle = SourceParticle::secondary("Home — open a workspace (visibility write)");

    let mut buttons = Vec::new();
    for (i, seed) in seed::LAUNCHABLE.iter().enumerate() {
        let trigger = if i == 0 {
            TriggerParticle::primary(seed.label)
        } else {
            TriggerParticle::new(seed.label)
        };
        launcher_triggers.insert(trigger.id, seed.open_id);
        buttons.push(Particle::Trigger(trigger));
    }

    // Wrap buttons in rows of 3.
    let mut rows = Vec::new();
    for chunk in buttons.chunks(3) {
        rows.push(Particle::Stack(
            StackParticle::row(chunk.to_vec()).with_gap(12.0),
        ));
    }

    let body = StackParticle::column(
        std::iter::once(Particle::Source(title))
            .chain(std::iter::once(Particle::Source(subtitle)))
            .chain(rows)
            .collect(),
    )
    .with_gap(14.0);

    ws.launcher_triggers = launcher_triggers;

    // Wrap in a View so the root column stretches the dashboard to fill the full
    // workspace area (matching Structural / Placeholder). A bare Surface only
    // measures to its content height, leaving empty space below.
    let surface = Particle::Surface(
        SurfaceParticle::new([0.12, 0.13, 0.16, 1.0])
            .with_padding(24.0)
            .with_radius(0.0)
            .with_border([0.26, 0.28, 0.32, 1.0], 1.0)
            .with_child(Particle::Stack(body)),
    );

    let mut view = ViewParticle::new("home");
    view.child = Some(Box::new(surface));
    Particle::View(view)
}
