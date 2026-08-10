use super::field_defs::{GEOMETRY, LOADING, MATERIAL, REINFORCEMENT};
use super::section::build_section;
use super::InputFormIO;
use crate::components::{engineer_input, engineer_text_input};
use crate::pages::analysis::field_builder::build_field_builder;
use crate::walls::{format_prop, is_standard_key};
use crate::domains::structural::{AnalysisAction, FieldBuilderDraft};
use crate::pages::analysis::input_form::form_density::FormDensity;
use hyper_ui::particles::{
    Particle, SourceParticle, StackParticle, SurfaceParticle, TriggerParticle, ViewParticle,
};
use hypernode::{HyperNode, Node, PropValue};
use std::collections::HashMap;

/// Build InputFormIO for the active wall (empty hint when none).
pub fn build_input_form(
    node: Option<&Node>,
    size: FormDensity,
    field_builder: Option<&FieldBuilderDraft>,
) -> InputFormIO {
    let mut field_props = HashMap::new();
    let mut u8_fields = HashMap::new();
    let mut actions = HashMap::new();
    let mut builder_slots = HashMap::new();
    let mut promote_props = HashMap::new();

    let mut children = Vec::new();

    if node.is_none() {
        children.push(Particle::Source(SourceParticle::new("Inputs").with_weight(500)));
        children.push(Particle::Source(SourceParticle::secondary(
            "Select or create a wall",
        )));
    } else {
        // Name (text)
        let name = node
            .and_then(|n| n.get_prop("wall_name"))
            .map(format_prop)
            .unwrap_or_default();
        let name_label = if size.hide_labels() {
            ""
        } else if size.abbreviate() {
            "Name"
        } else {
            "Wall name"
        };
        let name_row = engineer_text_input(name_label, &name, "");
        field_props.insert(name_row.field_id, "wall_name".into());
        children.push(name_row.into_particle());

        children.push(build_section(
            "Geometry",
            GEOMETRY,
            node,
            size,
            &mut field_props,
            &mut u8_fields,
        ));
        children.push(build_section(
            "Material",
            MATERIAL,
            node,
            size,
            &mut field_props,
            &mut u8_fields,
        ));
        children.push(build_section(
            "Reinforcement",
            REINFORCEMENT,
            node,
            size,
            &mut field_props,
            &mut u8_fields,
        ));
        children.push(build_section(
            "Loading",
            LOADING,
            node,
            size,
            &mut field_props,
            &mut u8_fields,
        ));

        // Custom section
        let mut custom_children = vec![Particle::Source(
            SourceParticle::new("Custom ✦").with_weight(500),
        )];
        if let Some(n) = node {
            for (key, value) in n.props() {
                if is_standard_key(key) || key.ends_with("__unit") {
                    continue;
                }
                let label = key.strip_prefix("custom:").unwrap_or(key);
                match value {
                    PropValue::Text(s) => {
                        let row = engineer_text_input(label, s, "");
                        field_props.insert(row.field_id, key.clone());
                        custom_children.push(row.into_particle());
                    }
                    PropValue::Bool(b) => {
                        let row = engineer_text_input(label, if *b { "true" } else { "false" }, "");
                        field_props.insert(row.field_id, key.clone());
                        custom_children.push(row.into_particle());
                    }
                    PropValue::F64(v) => {
                        let row = engineer_input(label, *v, "");
                        field_props.insert(row.field_id, key.clone());
                        custom_children.push(row.into_particle());
                    }
                    PropValue::U8(v) => {
                        let row = engineer_input(label, *v as f64, "");
                        field_props.insert(row.field_id, key.clone());
                        u8_fields.insert(row.field_id, ());
                        custom_children.push(row.into_particle());
                    }
                    PropValue::I64(v) => {
                        let row = engineer_input(label, *v as f64, "");
                        field_props.insert(row.field_id, key.clone());
                        custom_children.push(row.into_particle());
                    }
                }
                let promote = TriggerParticle::new("Promote");
                promote_props.insert(promote.id, key.clone());
                custom_children.push(Particle::Trigger(promote));
            }
        }

        if let Some(draft) = field_builder {
            let builder = build_field_builder(draft);
            actions.extend(builder.actions);
            builder_slots.extend(builder.slots);
            custom_children.push(builder.particle);
        } else {
            let add = TriggerParticle::new("+ Add field");
            actions.insert(add.id, AnalysisAction::OpenFieldBuilder);
            custom_children.push(Particle::Trigger(add));
        }

        children.push(Particle::Surface(
            SurfaceParticle::new([0.13, 0.14, 0.18, 1.0])
                .with_padding(8.0)
                .with_radius(0.0)
                .with_border([0.35, 0.55, 0.75, 1.0], 1.5)
                .with_child(Particle::Stack(
                    StackParticle::column(custom_children).with_gap(6.0),
                )),
        ));
    }

    let body = StackParticle::column(children).with_gap(10.0);
    let surface = SurfaceParticle::new([0.12, 0.13, 0.16, 1.0])
        .with_padding(10.0)
        .with_radius(0.0)
        .with_border([0.26, 0.28, 0.32, 1.0], 1.0)
        .with_child(Particle::Stack(body));

    let mut view = ViewParticle::new("input_form");
    view.child = Some(Box::new(Particle::Surface(surface)));

    InputFormIO {
        particle: Particle::View(view),
        field_props,
        u8_fields,
        actions,
        builder_slots,
        promote_props,
    }
}
