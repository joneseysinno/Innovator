use crate::walls::format_prop;
use hyper_ui::particles::{
    Particle, SourceParticle, StackParticle, SurfaceParticle, ViewParticle,
};
use hypernode::{Graph, HyperNode, NodeId};

const SUMMARY_KEYS: &[(&str, &str)] = &[
    ("wall_name", "Name"),
    ("wall_type", "Type"),
    ("height", "Height (ft)"),
    ("length", "Length (ft)"),
    ("thickness", "Thickness (in)"),
    ("clear_cover", "Cover (in)"),
    ("fc", "f'c (psi)"),
    ("fy", "fy (psi)"),
    ("vert_bar_size", "Vert bar #"),
    ("vert_spacing", "Vert spacing (in)"),
    ("horiz_bar_size", "Horiz bar #"),
    ("horiz_spacing", "Horiz spacing (in)"),
    ("pu", "Pu (kips)"),
    ("vu", "Vu (kips)"),
    ("mu", "Mu (kip-ft)"),
];

/// Build WallSummaryIO for the active wall (empty state when none selected).
pub fn build_wall_summary(graph: &Graph, active_wall: Option<NodeId>) -> super::WallSummaryIO {
    let title = SourceParticle::new("Summary").with_weight(500);
    let mut lines = vec![Particle::Source(title)];

    match active_wall.and_then(|id| graph.nodes.get(&id)) {
        None => {
            lines.push(Particle::Source(SourceParticle::secondary(
                "Select or create a wall",
            )));
        }
        Some(node) => {
            for (key, label) in SUMMARY_KEYS {
                let value = node
                    .get_prop(key)
                    .map(format_prop)
                    .unwrap_or_else(|| "—".into());
                lines.push(Particle::Source(SourceParticle::secondary(format!(
                    "{label}: {value}"
                ))));
            }
        }
    }

    let body = StackParticle::column(lines).with_gap(4.0);
    let surface = SurfaceParticle::new([0.12, 0.13, 0.16, 1.0])
        .with_padding(10.0)
        .with_radius(0.0)
        .with_border([0.26, 0.28, 0.32, 1.0], 1.0)
        .with_child(Particle::Stack(body));

    let mut view = ViewParticle::new("wall_summary");
    view.child = Some(Box::new(Particle::Surface(surface)));

    super::WallSummaryIO {
        particle: Particle::View(view),
    }
}
