use super::AppShell;
use crate::workspace::field_builder_draft::BuilderFieldSlot;
use crate::workspace::instance::WorkspaceInstance;
use hyper_ui::{FieldValue, ParticleId};

/// Update FieldBuilder draft from a committed builder field.
pub fn handle_builder_field(shell: &mut AppShell, field_id: ParticleId, value: FieldValue) {
    let active_id = shell.active_id;
    let Some(idx) = shell.workspaces.iter().position(|w| w.id() == active_id) else {
        return;
    };
    let WorkspaceInstance::Analysis(ws) = &mut shell.workspaces[idx] else {
        return;
    };
    let Some(slot) = ws.builder_slots.get(&field_id).copied() else {
        return;
    };
    let Some(draft) = ws.field_builder.as_mut() else {
        return;
    };
    match slot {
        BuilderFieldSlot::Label => {
            if let FieldValue::Text(s) = value {
                draft.label = s;
            } else {
                draft.label = value.display();
            }
        }
        BuilderFieldSlot::Initial => {
            draft.initial = value.as_f64().unwrap_or(draft.initial);
        }
        BuilderFieldSlot::Unit => {
            if let FieldValue::Text(s) = value {
                draft.unit = s;
            } else {
                draft.unit = value.display();
            }
        }
        BuilderFieldSlot::Min => {
            draft.min = value.as_f64().unwrap_or(draft.min);
        }
        BuilderFieldSlot::Max => {
            draft.max = value.as_f64().unwrap_or(draft.max);
        }
    }
}
