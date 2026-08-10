use super::field_defs::FieldDef;
use crate::components::{engineer_input, engineer_text_input};
use crate::walls::format_prop;
use crate::pages::analysis::input_form::form_density::FormDensity;
use hyper_ui::particles::{Particle, ParticleId, SourceParticle, StackParticle};
use hypernode::{HyperNode, Node, PropValue};
use std::collections::HashMap;

/// Build a titled section of engineer inputs; registers field→prop maps.
pub fn build_section(
    title: &str,
    defs: &[FieldDef],
    node: Option<&Node>,
    size: FormDensity,
    field_props: &mut HashMap<ParticleId, String>,
    u8_fields: &mut HashMap<ParticleId, ()>,
) -> Particle {
    let mut children = vec![Particle::Source(
        SourceParticle::new(title).with_weight(500),
    )];

    for def in defs {
        let label = def.display_label(size);
        if def.is_text {
            let text = node
                .and_then(|n| n.get_prop(def.key))
                .map(format_prop)
                .unwrap_or_default();
            let row = engineer_text_input(label, &text, def.unit);
            field_props.insert(row.field_id, def.key.into());
            children.push(row.into_particle());
        } else {
            let value = match node.and_then(|n| n.get_prop(def.key)) {
                Some(PropValue::F64(v)) => *v,
                Some(PropValue::U8(v)) => *v as f64,
                Some(PropValue::I64(v)) => *v as f64,
                _ => 0.0,
            };
            let row = engineer_input(label, value, def.unit);
            field_props.insert(row.field_id, def.key.into());
            if def.is_u8 {
                u8_fields.insert(row.field_id, ());
            }
            children.push(row.into_particle());
        }
    }

    Particle::Stack(StackParticle::column(children).with_gap(6.0))
}
