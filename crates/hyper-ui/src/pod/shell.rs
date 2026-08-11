//! Uniform pod chrome — bordered shell with collapsible title bar.

use crate::particles::{
    Particle, ParticleId, StackParticle, SurfaceParticle, TriggerParticle,
};

use super::Pod;

/// Fill for the pod title bar.
pub const POD_TITLE_FILL: [f32; 4] = [0.16, 0.17, 0.20, 1.0];
/// Fill for the bordered pod frame.
pub const POD_FRAME_FILL: [f32; 4] = [0.12, 0.13, 0.16, 1.0];
/// Border color separating pods visually.
pub const POD_FRAME_BORDER: [f32; 4] = [0.32, 0.34, 0.40, 1.0];
pub const POD_FRAME_BORDER_WIDTH: f32 = 1.0;
pub const POD_TITLE_PADDING: f32 = 4.0;
pub const POD_BODY_PADDING: f32 = 6.0;
/// Gap between stacked pod shells on a page.
pub const POD_STACK_GAP: f32 = 4.0;

/// Result of wrapping a pod body in uniform chrome.
pub struct PodShell {
    pub particle: Particle,
    pub collapse_trigger_id: ParticleId,
}

/// Build a full-width bordered pod: title bar + optional body.
///
/// When `collapsed` is true, the body is omitted so only the title bar
/// occupies vertical space.
pub fn pod_shell(title: impl Into<String>, collapsed: bool, body: Particle) -> PodShell {
    let trigger = TriggerParticle::new(title);
    let collapse_trigger_id = trigger.id;
    let bar = Particle::Surface(
        SurfaceParticle::new(POD_TITLE_FILL)
            .with_padding(POD_TITLE_PADDING)
            .with_radius(0.0)
            .with_child(Particle::Trigger(trigger)),
    );

    let inner = if collapsed {
        Particle::Stack(StackParticle::column(vec![bar]).with_gap(0.0))
    } else {
        let padded_body = Particle::Surface(
            SurfaceParticle::new([0.0, 0.0, 0.0, 0.0])
                .with_padding(POD_BODY_PADDING)
                .with_radius(0.0)
                .with_child(body),
        );
        Particle::Stack(StackParticle::column(vec![bar, padded_body]).with_gap(0.0))
    };

    let particle = Particle::Surface(
        SurfaceParticle::new(POD_FRAME_FILL)
            .with_padding(0.0)
            .with_radius(0.0)
            .with_border(POD_FRAME_BORDER, POD_FRAME_BORDER_WIDTH)
            .with_child(inner),
    );

    PodShell {
        particle,
        collapse_trigger_id,
    }
}

/// Wrap each body particle with [`pod_shell`] using the matching [`Pod`] metadata.
///
/// Returns the column stack and `(collapse_trigger_id, pod_id)` pairs for wiring.
/// Extra bodies beyond `pods.len()` get synthetic titles; missing bodies are skipped.
pub fn wrap_pod_column(
    pods: &[Pod],
    bodies: impl IntoIterator<Item = Particle>,
) -> (Particle, Vec<(ParticleId, crate::pod::PodId)>) {
    let mut triggers = Vec::new();
    let mut children = Vec::new();
    for (i, body) in bodies.into_iter().enumerate() {
        let (pod_id, title, collapsed) = match pods.get(i) {
            Some(p) => (p.id, p.title.clone(), p.collapsed),
            None => (
                crate::pod::PodId(i as u32),
                format!("Pod {i}"),
                false,
            ),
        };
        let shell = pod_shell(title, collapsed, body);
        triggers.push((shell.collapse_trigger_id, pod_id));
        children.push(shell.particle);
    }
    let stack = Particle::Stack(StackParticle::column(children).with_gap(POD_STACK_GAP));
    (stack, triggers)
}

/// Collect `(pod_id, glyph)` for pods that opted into the page icon rail.
pub fn pod_nav_icons(pods: &[Pod]) -> Vec<(crate::pod::PodId, String)> {
    pods.iter()
        .filter_map(|p| p.nav_icon.as_ref().map(|icon| (p.id, icon.clone())))
        .collect()
}
