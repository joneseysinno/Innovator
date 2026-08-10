use super::rebuild_active::rebuild_active;
use super::AppShell;
use crate::engine::{run_analysis, AnalysisOutput};
use crate::results::{export_results_pdf, load_results_for_wall, parse_checks, persist_results};
use crate::walls::new_wall;
use crate::workspace::signal::WorkspaceSignal;
use hyper_ui::apply_signal_text;
use hypernode::{Graph, PropValue};

pub fn handle_workspace_signal(shell: &mut AppShell, signal: WorkspaceSignal) {
    let mut rebuild = false;
    let mut status_msg: Option<String> = None;

    let idx = shell.workspaces.iter().position(|w| w.is_active());

    match signal {
        WorkspaceSignal::NewWall => {
            if let Some(idx) = idx {
                if let Some(ws) = shell.workspaces[idx].structural_mut() {
                    let name = ws.next_wall_name();
                    let id = new_wall(&mut ws.graph, &mut shell.db, name);
                    ws.active_wall = Some(id);
                    ws.last_results = None;
                    ws.last_analysis = None;
                    status_msg = Some(format!("signal: New Wall ({})", id.0));
                    rebuild = true;
                }
            }
        }
        WorkspaceSignal::WallSelected(id) => {
            if let Some(idx) = idx {
                if let Some(ws) = shell.workspaces[idx].structural_mut() {
                    if ws.select_wall(id) {
                        let loaded = load_results_for_wall(&mut shell.db, id);
                        ws.last_analysis = loaded.as_ref().map(summary_from_results);
                        ws.last_results = loaded;
                        status_msg = Some(format!("signal: Wall Selected ({})", id.0));
                        rebuild = true;
                    }
                }
            }
        }
        WorkspaceSignal::RunAnalysis => {
            if let Some(idx) = idx {
                if let Some(ws) = shell.workspaces[idx].structural_mut() {
                    let wall = ws
                        .active_wall
                        .and_then(|id| ws.graph.nodes.get(&id).cloned());
                    match wall {
                        None => {
                            status_msg = Some("signal: Run Analysis — no active wall".into());
                        }
                        Some(wall) => {
                            let mut output = run_analysis(&wall);
                            let mut scratch = Graph::new();
                            let rid = scratch.insert_node(output.results_node.clone());
                            if let Some(node) = scratch.nodes.get(&rid).cloned() {
                                let _ = persist_results(&mut shell.db, &node);
                                output.results_node = node.clone();
                                ws.last_results = Some(node);
                            }
                            ws.last_analysis = Some(output);
                            status_msg = Some("signal: AnalysisComplete".into());
                            rebuild = true;
                            let _ = WorkspaceSignal::AnalysisComplete;
                        }
                    }
                }
            }
        }
        WorkspaceSignal::AnalysisComplete => {
            status_msg = Some("signal: AnalysisComplete".into());
            rebuild = true;
        }
        WorkspaceSignal::Save => status_msg = Some("signal: Save".into()),
        WorkspaceSignal::Export => {
            if let Some(idx) = idx {
                if let Some(ws) = shell.workspaces[idx].structural() {
                    if let Some(results) = ws.last_results.as_ref() {
                        match export_results_pdf(results) {
                            Ok(path) => {
                                status_msg =
                                    Some(format!("signal: Export PDF → {}", path.display()));
                            }
                            Err(e) => {
                                status_msg = Some(format!("signal: Export failed ({e})"));
                            }
                        }
                    } else {
                        status_msg = Some("signal: Export — no results".into());
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
        if let Some(msg) = status_msg {
            if let Some(id) = shell.active().and_then(|a| a.status_id()) {
                apply_signal_text(&mut renderer.ui.tree, id, msg);
            }
        }
        shell.renderer = Some(renderer);
    } else if let Some(msg) = status_msg {
        let status_id = shell.active().and_then(|a| a.status_id());
        if let (Some(id), Some(renderer)) = (status_id, shell.renderer.as_mut()) {
            apply_signal_text(&mut renderer.ui.tree, id, msg);
        }
    }
}

fn summary_from_results(results: &hypernode::Node) -> AnalysisOutput {
    let checks = parse_checks(results);
    let overall_pass = matches!(
        results.props.get("overall_pass"),
        Some(PropValue::Bool(true))
    );
    let governing = match results.props.get("governing") {
        Some(PropValue::Text(s)) => s.clone(),
        _ => "—".into(),
    };
    let run_timestamp = match results.props.get("run_timestamp") {
        Some(PropValue::I64(v)) => *v,
        _ => 0,
    };
    AnalysisOutput {
        results_node: results.clone(),
        checks,
        overall_pass,
        governing,
        run_timestamp,
    }
}
