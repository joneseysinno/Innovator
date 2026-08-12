use super::rebuild_active::rebuild_active;
use super::AppShell;
use crate::domains::structural::graph_wires::{
    clear_results_pod, ensure_aci_318_engine, pod_node_id, wire_active_wall_binding,
    wire_run_analysis, wire_wall_list_streams,
};
use crate::domains::structural::template_ids::{INPUT_FORM, RESULTS_TABLE, WALL_LIST};
use crate::domains::structural::StructuralWorkspace;
use crate::engine::{run_analysis, AnalysisOutput};
use crate::results::{export_results_pdf, load_results_for_wall, parse_checks, persist_results};
use crate::walls::new_wall;
use crate::workspace::signal::WorkspaceSignal;
use hyper_ui::apply_signal_text;
use hypernode::PropValue;

pub fn handle_workspace_signal(shell: &mut AppShell, signal: WorkspaceSignal) {
    let mut rebuild = false;
    let mut status_msg: Option<String> = None;

    let idx = shell.workspaces.iter().position(|w| w.is_active());

    match signal {
        WorkspaceSignal::NewWall => {
            if let Some(idx) = idx {
                let name = StructuralWorkspace::next_wall_name(&shell.graph);
                let id = new_wall(&mut shell.graph, &mut shell.db, name);
                if let Some(ws) = shell.workspaces[idx].structural_mut() {
                    ws.active_wall = Some(id);
                    ws.last_results = None;
                    ws.last_analysis = None;
                    status_msg = Some(format!("signal: New Wall ({})", id.0));
                    rebuild = true;
                }
                if let Some(ws) = shell.workspaces[idx].structural() {
                    if let Some(wall_list) = pod_node_id(ws, WALL_LIST) {
                        wire_wall_list_streams(&mut shell.graph, wall_list);
                    }
                    if let Some(input) = pod_node_id(ws, INPUT_FORM) {
                        wire_active_wall_binding(&mut shell.graph, input, id);
                    }
                    if let Some(results_pod) = pod_node_id(ws, RESULTS_TABLE) {
                        clear_results_pod(&mut shell.graph, results_pod);
                    }
                }
            }
        }
        WorkspaceSignal::WallSelected(id) => {
            if let Some(idx) = idx {
                let mut selected = false;
                if let Some(ws) = shell.workspaces[idx].structural_mut() {
                    if ws.select_wall(&shell.graph, id) {
                        let loaded = load_results_for_wall(&mut shell.db, id);
                        ws.last_analysis = loaded.as_ref().map(summary_from_results);
                        ws.last_results = loaded;
                        status_msg = Some(format!("signal: Wall Selected ({})", id.0));
                        rebuild = true;
                        selected = true;
                    }
                }
                if selected {
                    if let Some(ws) = shell.workspaces[idx].structural() {
                        if let Some(wall_list) = pod_node_id(ws, WALL_LIST) {
                            wire_wall_list_streams(&mut shell.graph, wall_list);
                        }
                        if let Some(input) = pod_node_id(ws, INPUT_FORM) {
                            wire_active_wall_binding(&mut shell.graph, input, id);
                        }
                        if let Some(results_pod) = pod_node_id(ws, RESULTS_TABLE) {
                            clear_results_pod(&mut shell.graph, results_pod);
                        }
                        if let (Some(mut results), Some(results_pod)) =
                            (ws.last_results.clone(), pod_node_id(ws, RESULTS_TABLE))
                        {
                            // Persisted result IDs are local to the results
                            // store, so allocate a composed-graph identity.
                            results.id = hypernode::NodeId(0);
                            let results_id = shell.graph.insert_node(results.clone());
                            results.id = results_id;
                            if let Some(ws) = shell.workspaces[idx].structural_mut() {
                                ws.last_results = Some(results.clone());
                                if let Some(summary) = ws.last_analysis.as_mut() {
                                    summary.results_node = results;
                                }
                            }
                            let engine = ensure_aci_318_engine(&mut shell.graph);
                            wire_run_analysis(
                                &mut shell.graph,
                                id,
                                engine,
                                results_id,
                                results_pod,
                            );
                        }
                    }
                }
            }
        }
        WorkspaceSignal::RunAnalysis => {
            if let Some(idx) = idx {
                let wall = shell.workspaces[idx]
                    .structural()
                    .and_then(|ws| ws.active_wall)
                    .and_then(|id| shell.graph.nodes.get(&id).cloned());
                match wall {
                    None => {
                        status_msg = Some("signal: Run Analysis — no active wall".into());
                    }
                    Some(wall) => {
                        let mut output = run_analysis(&wall);
                        let results_id = shell.graph.insert_node(output.results_node.clone());
                        let results = shell
                            .graph
                            .nodes
                            .get(&results_id)
                            .cloned()
                            .expect("inserted results node");
                        let _ = persist_results(&mut shell.db, &results);
                        output.results_node = results.clone();

                        if let Some(ws) = shell.workspaces[idx].structural_mut() {
                            ws.last_results = Some(results);
                            ws.last_analysis = Some(output);
                        }
                        if let Some(ws) = shell.workspaces[idx].structural() {
                            if let Some(results_pod) = pod_node_id(ws, RESULTS_TABLE) {
                                let engine = ensure_aci_318_engine(&mut shell.graph);
                                wire_run_analysis(
                                    &mut shell.graph,
                                    wall.id,
                                    engine,
                                    results_id,
                                    results_pod,
                                );
                            }
                        }
                        status_msg = Some("signal: AnalysisComplete".into());
                        rebuild = true;
                        let _ = WorkspaceSignal::AnalysisComplete;
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
