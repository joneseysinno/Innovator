use super::build_section::{build_section_spatial, empty_section_spatial};
use super::WallViewIO;
use hyper_ui::particles::{
    Particle, SinkParticle, SourceParticle, StackParticle, SurfaceParticle, ViewParticle,
};
use hypernode::Node;

/// Build WallViewIO — sink owns pan/zoom; Layer A draws the section.
pub fn build_wall_view(node: Option<&Node>) -> WallViewIO {
    let spatial = match node {
        Some(n) => build_section_spatial(n),
        None => empty_section_spatial(),
    };

    let title = SourceParticle::new("Section").with_weight(500);
    let hint = SourceParticle::secondary(if node.is_some() {
        "Scroll to zoom · drag to pan"
    } else {
        "Select a wall to view section"
    });

    let chrome = StackParticle::column(vec![Particle::Source(title), Particle::Source(hint)])
        .with_gap(4.0);

    let surface = SurfaceParticle::new([0.10, 0.11, 0.14, 1.0])
        .with_padding(8.0)
        .with_radius(0.0)
        .with_border([0.26, 0.28, 0.32, 1.0], 1.0)
        .with_child(Particle::Stack(chrome));

    let sink = SinkParticle::new().with_child(Particle::Surface(surface));
    let sink_id = sink.id;

    let mut view = ViewParticle::new("wall_view");
    view.child = Some(Box::new(Particle::Sink(sink)));

    WallViewIO {
        particle: Particle::View(view),
        sink_id,
        spatial,
    }
}
