use super::FieldBuilderIO;
use crate::components::{engineer_input, engineer_text_input};
use crate::domains::structural::{
    AnalysisAction, BuilderFieldSlot, CustomFieldKind, FieldBuilderDraft,
};
use hyper_ui::particles::{
    Particle, SourceParticle, StackParticle, SurfaceParticle, TriggerParticle,
};
use std::collections::HashMap;

/// Build the inline FieldBuilderIO from the current draft.
pub fn build_field_builder(draft: &FieldBuilderDraft) -> FieldBuilderIO {
    let mut actions = HashMap::new();
    let mut slots = HashMap::new();

    let title = SourceParticle::new("Add field").with_weight(500);

    let label = engineer_text_input("Label", &draft.label, "");
    slots.insert(label.field_id, BuilderFieldSlot::Label);

    let initial = engineer_input("Initial value", draft.initial, "");
    slots.insert(initial.field_id, BuilderFieldSlot::Initial);

    let unit = engineer_text_input("Unit", &draft.unit, "");
    slots.insert(unit.field_id, BuilderFieldSlot::Unit);

    let type_label = SourceParticle::secondary("Type");
    let mut type_chips = vec![Particle::Source(type_label)];
    for (kind, action) in [
        (CustomFieldKind::Number, AnalysisAction::FieldKindNumber),
        (CustomFieldKind::Text, AnalysisAction::FieldKindText),
        (CustomFieldKind::Bool, AnalysisAction::FieldKindBool),
    ] {
        let chip = if draft.kind == kind {
            TriggerParticle::primary(kind.label())
        } else {
            TriggerParticle::new(kind.label())
        };
        actions.insert(chip.id, action);
        type_chips.push(Particle::Trigger(chip));
    }
    let type_row = StackParticle::row(type_chips).with_gap(8.0);

    let min = engineer_input("Min", draft.min, "");
    slots.insert(min.field_id, BuilderFieldSlot::Min);
    let max = engineer_input("Max", draft.max, "");
    slots.insert(max.field_id, BuilderFieldSlot::Max);
    let range_row = StackParticle::row(vec![min.into_particle(), max.into_particle()]).with_gap(8.0);

    let cancel = TriggerParticle::new("Cancel");
    actions.insert(cancel.id, AnalysisAction::CancelFieldBuilder);
    let add = TriggerParticle::primary("Add");
    actions.insert(add.id, AnalysisAction::ConfirmFieldBuilder);
    let buttons = StackParticle::row(vec![Particle::Trigger(cancel), Particle::Trigger(add)])
        .with_gap(8.0);

    let body = StackParticle::column(vec![
        Particle::Source(title),
        label.into_particle(),
        initial.into_particle(),
        unit.into_particle(),
        Particle::Stack(type_row),
        Particle::Stack(range_row),
        Particle::Stack(buttons),
    ])
    .with_gap(8.0);

    let particle = Particle::Surface(
        SurfaceParticle::new([0.14, 0.16, 0.20, 1.0])
            .with_padding(10.0)
            .with_radius(0.0)
            .with_border([0.35, 0.55, 0.75, 1.0], 1.5)
            .with_child(Particle::Stack(body)),
    );

    FieldBuilderIO {
        particle,
        actions,
        slots,
    }
}
