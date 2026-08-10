use super::rebuild_active::rebuild_active;
use super::AppShell;
use crate::domains::structural::{AnalysisAction, CustomFieldKind, FieldBuilderDraft};
use crate::walls::{persist_wall, slug_key};
use hyper_ui::apply_signal_text;
use hypernode::{EdgeId, EdgeKind, HyperEdge, HyperNode, NodeId, PropValue};

/// Handle Analysis-page triggers (FieldBuilder / type chips).
pub fn handle_analysis_action(shell: &mut AppShell, action: AnalysisAction) {
    let Some(idx) = shell.workspaces.iter().position(|w| w.is_active()) else {
        return;
    };

    let mut rebuild = false;
    let mut status = None;

    {
        let Some(ws) = shell.workspaces[idx].structural_mut() else {
            return;
        };

        match action {
            AnalysisAction::OpenFieldBuilder => {
                ws.field_builder = Some(FieldBuilderDraft::default());
                rebuild = true;
                status = Some("signal: FieldBuilder open".into());
            }
            AnalysisAction::CancelFieldBuilder => {
                ws.field_builder = None;
                rebuild = true;
                status = Some("signal: FieldBuilder cancel".into());
            }
            AnalysisAction::ConfirmFieldBuilder => {
                if let Some(draft) = ws.field_builder.take() {
                    if let Some(wall_id) = ws.active_wall {
                        let key = slug_key(&draft.label);
                        let value = match draft.kind {
                            CustomFieldKind::Number => PropValue::F64(draft.initial),
                            CustomFieldKind::Text => PropValue::Text(String::new()),
                            CustomFieldKind::Bool => PropValue::Bool(draft.initial != 0.0),
                        };
                        if let Some(node) = ws.graph.nodes.get_mut(&wall_id) {
                            node.set_prop(key.clone(), value);
                            if !draft.unit.is_empty() && draft.kind == CustomFieldKind::Number {
                                node.set_prop(
                                    format!("{key}__unit"),
                                    PropValue::Text(draft.unit.clone()),
                                );
                            }
                            let _ = persist_wall(&mut shell.db, node);
                        }
                        status = Some(format!("signal: custom field {key}"));
                    }
                }
                rebuild = true;
            }
            AnalysisAction::FieldKindNumber
            | AnalysisAction::FieldKindText
            | AnalysisAction::FieldKindBool => {
                if let Some(draft) = ws.field_builder.as_mut() {
                    if let Some(kind) = CustomFieldKind::from_action(action) {
                        draft.kind = kind;
                        rebuild = true;
                        status = Some(format!("signal: type {}", kind.label()));
                    }
                }
            }
        }
    }

    if rebuild {
        let mut renderer = match shell.renderer.take() {
            Some(r) => r,
            None => return,
        };
        rebuild_active(shell, &mut renderer);
        if let Some(msg) = status {
            if let Some(id) = shell.active().and_then(|a| a.status_id()) {
                apply_signal_text(&mut renderer.ui.tree, id, msg);
            }
        }
        shell.renderer = Some(renderer);
    }
}

/// Promote a custom property to all walls via a Wave hyperedge.
pub fn handle_promote_prop(shell: &mut AppShell, key: String) {
    let Some(idx) = shell.workspaces.iter().position(|w| w.is_active()) else {
        return;
    };

    let status;
    {
        let Some(ws) = shell.workspaces[idx].structural_mut() else {
            return;
        };
        let Some(wall_id) = ws.active_wall else {
            return;
        };
        let default = ws
            .graph
            .nodes
            .get(&wall_id)
            .and_then(|n| n.get_prop(&key).cloned())
            .unwrap_or(PropValue::F64(0.0));

        let ids: Vec<NodeId> = ws.graph.nodes.keys().copied().collect();
        for id in &ids {
            if let Some(node) = ws.graph.nodes.get_mut(id) {
                if node.get_prop(&key).is_none() {
                    node.set_prop(key.clone(), default.clone());
                    let _ = persist_wall(&mut shell.db, node);
                }
            }
        }
        ws.graph.insert_edge(HyperEdge {
            id: EdgeId(0),
            kind: EdgeKind::Wave,
            sources: vec![wall_id],
            targets: ids,
            curvature: 0.2,
            label: Some(format!("Promote:{key}")),
        });
        status = format!("signal: Wave promote {key}");
    }

    let mut renderer = match shell.renderer.take() {
        Some(r) => r,
        None => return,
    };
    rebuild_active(shell, &mut renderer);
    if let Some(id) = shell.active().and_then(|a| a.status_id()) {
        apply_signal_text(&mut renderer.ui.tree, id, status);
    }
    shell.renderer = Some(renderer);
}
