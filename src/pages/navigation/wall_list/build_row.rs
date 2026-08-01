use hyper_ui::particles::{
    Particle, SinkParticle, SourceParticle, StackParticle, SurfaceParticle,
};
use hypernode::{HyperNode, Node, NodeId};

const ROW_IDLE: [f32; 4] = [0.14, 0.15, 0.18, 1.0];
const ROW_ACTIVE: [f32; 4] = [0.18, 0.28, 0.42, 1.0];

/// One selectable wall row: sink → surface → stack(row) → [name, badge].
pub fn build_row(node: &Node, active: Option<NodeId>) -> (Particle, hyper_ui::ParticleId) {
    let selected = active == Some(node.id());
    let name = node
        .get_prop("wall_name")
        .and_then(|v| match v {
            hypernode::PropValue::Text(s) => Some(s.as_str()),
            _ => None,
        })
        .unwrap_or(node.label());
    let name_src = SourceParticle::new(name);
    let badge = SourceParticle::muted("—");

    let row = StackParticle::row(vec![Particle::Source(name_src), Particle::Source(badge)])
        .with_gap(10.0);

    let surface = SurfaceParticle::new(if selected { ROW_ACTIVE } else { ROW_IDLE })
        .with_padding(8.0)
        .with_radius(0.0)
        .with_border([0.26, 0.28, 0.32, 1.0], 1.0)
        .with_child(Particle::Stack(row));

    let sink = SinkParticle::new().with_child(Particle::Surface(surface));
    let sink_id = sink.id;
    (Particle::Sink(sink), sink_id)
}
