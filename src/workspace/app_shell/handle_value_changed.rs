use super::rebuild_active::rebuild_active;
use super::AppShell;
use crate::pages::analysis::wall_view::build_section_spatial;
use crate::walls::{field_value_to_prop, is_geometry_or_rebar_key, persist_wall};
use hyper_ui::{apply_signal_text, FieldValue, ParticleId};
use hypernode::HyperNode;

/// Commit a form field to the active wall HyperNode and persist.
pub fn handle_value_changed(shell: &mut AppShell, field_id: ParticleId, value: FieldValue) {
    let Some(idx) = shell.workspaces.iter().position(|w| w.is_active()) else {
        return;
    };

    let (rebuild_ui, status) = {
        let Some(ws) = shell.workspaces[idx].structural_mut() else {
            return;
        };
        let Some(key) = ws.field_props.get(&field_id).cloned() else {
            return;
        };
        let prefer_u8 = ws.u8_fields.contains_key(&field_id);
        let Some(wall_id) = ws.active_wall else {
            return;
        };
        let Some(node) = ws.graph.nodes.get_mut(&wall_id) else {
            return;
        };
        let prop = field_value_to_prop(&value, prefer_u8);
        node.set_prop(key.clone(), prop.clone());
        let mut rebuild_ui = false;
        if key == "wall_name" {
            if let hypernode::PropValue::Text(s) = prop {
                node.label = s;
            }
            rebuild_ui = true;
        }
        let _ = persist_wall(&mut shell.db, node);
        if is_geometry_or_rebar_key(&key) {
            ws.wall_spatial = build_section_spatial(node);
        }
        let status = format!("signal: ValueChanged {key}={}", value.display());
        (rebuild_ui, status)
    };

    if rebuild_ui {
        let mut renderer = match shell.renderer.take() {
            Some(r) => r,
            None => return,
        };
        rebuild_active(shell, &mut renderer);
        if let Some(id) = shell.active().and_then(|a| a.status_id()) {
            apply_signal_text(&mut renderer.ui.tree, id, status);
        }
        shell.renderer = Some(renderer);
    } else if let Some(id) = shell.active().and_then(|a| a.status_id()) {
        if let Some(renderer) = shell.renderer.as_mut() {
            apply_signal_text(&mut renderer.ui.tree, id, status);
        }
    }
}
