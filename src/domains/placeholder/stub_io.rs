//! Stub IO — label + extent demands + resolved visibility readout.

use hyper_ui::particles::{Particle, SourceParticle, StackParticle, SurfaceParticle};
use hyper_ui::Pod;

/// One stub IO readout inside a pod. No ContainerState — particle layout only.
pub fn build_stub_io(label: &str, pod: &Pod) -> Particle {
    let title = SourceParticle::new(label).with_weight(500);
    let readout = SourceParticle::muted(format!(
        "min {:.0} / ideal {:.0} / resolved {:?} @ {:.0}px",
        pod.state.extent.min,
        pod.state.extent.ideal,
        pod.state.resolved(),
        pod.state.rect().size.y.max(pod.state.rect().size.x),
    ));
    let body = StackParticle::column(vec![Particle::Source(title), Particle::Source(readout)])
        .with_gap(4.0);
    Particle::Surface(
        SurfaceParticle::new([0.12, 0.13, 0.16, 1.0])
            .with_padding(10.0)
            .with_radius(0.0)
            .with_border([0.26, 0.28, 0.32, 1.0], 1.0)
            .with_child(Particle::Stack(body)),
    )
}

/// Stack several stub IO top-to-bottom inside a pod.
pub fn build_stub_stack(labels: &[&str], pod: &Pod) -> Particle {
    if labels.is_empty() {
        return build_stub_io(pod.title.as_str(), pod);
    }
    let children: Vec<_> = labels
        .iter()
        .map(|label| build_stub_io(label, pod))
        .collect();
    Particle::Stack(StackParticle::column(children).with_gap(6.0))
}
