use super::graph_wires::{pod_node_id, wire_run_analysis, wire_workspace};
use super::kind::AnalysisKind;
use super::template_ids::RESULTS_TABLE;
use super::templates::{initial_page_templates, initial_pod_templates};
use super::workspace::StructuralWorkspace;
use crate::engine::AnalysisOutput;
use crate::pages::analysis::input_form::form_density::FormDensity;
use crate::results::{load_results_for_wall, parse_checks};
use crate::workspace::from_seed::page_tree_from_seeds;
use crate::workspace::graph_containers::{
    dual_write_page_tree, entity_count, first_entity_id, insert_uiview, is_entity,
};
use crate::workspace::header::build_header;
use crate::workspace::seed;
use hyper_ui::InMemoryWorldSpatial;
use hypernode::{Graph, HyperNode, NodeId, PropValue};
use infinite_db::InfiniteDb;
use std::collections::HashMap;

impl StructuralWorkspace {
    /// Build from seeds, dual-writing UIView containers into `graph`.
    ///
    /// Domain walls are loaded into the same composed-view graph. Does not own
    /// the graph — `AppShell` does.
    pub fn new(db: &mut InfiniteDb, graph: &mut Graph) -> Self {
        let active_wall = first_entity_id(graph);
        let (last_results, last_analysis) = match active_wall {
            Some(wid) => load_analysis_state(db, wid),
            None => (None, None),
        };
        let mut page_tree = page_tree_from_seeds(seed::STRUCTURAL.pages);
        let node_id = insert_uiview(graph, seed::STRUCTURAL.label);
        dual_write_page_tree(graph, node_id, &mut page_tree);
        let page_templates = initial_page_templates();
        let pod_templates = initial_pod_templates();
        for page in &page_tree.pages {
            if let Some(template) = page_templates.get(&page.id) {
                graph
                    .nodes
                    .get_mut(&page.node_id)
                    .expect("page UIView")
                    .props
                    .insert(
                        "template_id".into(),
                        PropValue::Text(template.as_str().into()),
                    );
            }
            for pod in &page.pods.pods {
                if let Some(template) = pod_templates.get(&(page.id, pod.id)) {
                    graph
                        .nodes
                        .get_mut(&pod.node_id)
                        .expect("pod UIView")
                        .props
                        .insert(
                            "template_id".into(),
                            PropValue::Text(template.as_str().into()),
                        );
                }
            }
        }
        let mut workspace = Self {
            header: Some(build_header()),
            page_tree,
            page_templates,
            pod_templates,
            next_page_id: seed::STRUCTURAL.pages.len() as u32,
            active_analysis: AnalysisKind::SpecialConcreteWall,
            active_wall,
            wall_sinks: HashMap::new(),
            nav_triggers: HashMap::new(),
            field_props: HashMap::new(),
            u8_fields: HashMap::new(),
            analysis_actions: HashMap::new(),
            builder_slots: HashMap::new(),
            promote_props: HashMap::new(),
            field_builder: None,
            input_size_class: FormDensity::Full,
            wall_view_sink: None,
            wall_spatial: InMemoryWorldSpatial::default(),
            wall_view_last_pos: None,
            wall_view_panning: false,
            last_results,
            last_analysis,
            results_triggers: HashMap::new(),
            icon_rail_triggers: HashMap::new(),
            pod_collapse_triggers: HashMap::new(),
            page_viewport_ids: HashMap::new(),
            page_split_triggers: HashMap::new(),
            page_show_triggers: HashMap::new(),
            page_overrides: hyper_ui::Overrides::new(),
            focused_page: hyper_ui::PageId(0),
            analysis_header_status_id: None,
            node_id,
        };
        wire_workspace(graph, &workspace);

        if let (Some(wall), Some(mut results), Some(results_pod)) = (
            workspace.active_wall,
            workspace.last_results.clone(),
            pod_node_id(&workspace, RESULTS_TABLE),
        ) {
            // Result IDs from the results store are not graph-global. Allocate
            // a fresh composed-graph identity before linking the cached result.
            results.id = NodeId(0);
            let results_id = graph.insert_node(results.clone());
            results.id = results_id;
            workspace.last_results = Some(results.clone());
            if let Some(summary) = workspace.last_analysis.as_mut() {
                summary.results_node = results;
            }
            let engine = super::graph_wires::ensure_aci_318_engine(graph);
            wire_run_analysis(graph, wall, engine, results_id, results_pod);
        }

        workspace
    }

    /// Select a wall; returns true if the selection changed.
    pub fn select_wall(&mut self, graph: &Graph, id: NodeId) -> bool {
        if is_entity(graph, id) && self.active_wall != Some(id) {
            self.active_wall = Some(id);
            true
        } else {
            false
        }
    }

    pub fn wall_count(graph: &Graph) -> usize {
        entity_count(graph)
    }

    pub fn next_wall_name(graph: &Graph) -> String {
        format!("Wall {}", Self::wall_count(graph) + 1)
    }

    pub fn alloc_page_id(&mut self) -> hyper_ui::PageId {
        let id = hyper_ui::PageId(self.next_page_id);
        self.next_page_id += 1;
        id
    }
}

fn load_analysis_state(
    db: &mut InfiniteDb,
    wall_id: NodeId,
) -> (Option<hypernode::Node>, Option<AnalysisOutput>) {
    let Some(results) = load_results_for_wall(db, wall_id) else {
        return (None, None);
    };
    let checks = parse_checks(&results);
    let overall_pass = matches!(
        results.get_prop("overall_pass"),
        Some(PropValue::Bool(true))
    );
    let governing = match results.get_prop("governing") {
        Some(PropValue::Text(s)) => s.clone(),
        _ => "—".into(),
    };
    let run_timestamp = match results.get_prop("run_timestamp") {
        Some(PropValue::I64(v)) => *v,
        _ => 0,
    };
    let summary = AnalysisOutput {
        results_node: results.clone(),
        checks,
        overall_pass,
        governing,
        run_timestamp,
    };
    (Some(results), Some(summary))
}
