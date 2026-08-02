use super::input_form::build_input_form;
use super::wall_view::build_wall_view;
use crate::domains::structural::StructuralWorkspace;
use hyper_ui::particles::{Particle, StackParticle};

/// Build the Analysis page (InputForm | WallView) and wire interaction maps.
pub fn build_analysis(ws: &mut StructuralWorkspace) -> Particle {
    let node = ws
        .active_wall
        .and_then(|id| ws.graph.nodes.get(&id))
        .cloned();
    let form = build_input_form(
        node.as_ref(),
        ws.input_size_class,
        ws.field_builder.as_ref(),
    );
    let view = build_wall_view(node.as_ref());

    ws.field_props = form.field_props;
    ws.u8_fields = form.u8_fields;
    ws.analysis_actions = form.actions;
    ws.builder_slots = form.builder_slots;
    ws.promote_props = form.promote_props;
    ws.wall_view_sink = Some(view.sink_id);
    ws.wall_spatial = view.spatial;

    Particle::Stack(
        StackParticle::column(vec![form.particle, view.particle]).with_gap(0.0),
    )
}
