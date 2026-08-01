use super::kind::AnalysisKind;
use super::pages_pod_tree::pages_pod_tree;
use super::AnalysisWorkspace;
use crate::engine::AnalysisOutput;
use crate::results::{load_results_for_wall, parse_checks};
use crate::walls::load_walls;
use crate::workspace::header::build_header;
use crate::workspace::kind::WorkspaceKind;
use crate::workspace::page::Page;
use crate::workspace::size_class::SizeClass;
use crate::workspace::tab::WorkspaceTab;
use crate::workspace::workspace_id::WorkspaceId;
use hyper_ui::InMemoryWorldSpatial;
use hypernode::{HyperNode, PropValue};
use infinite_db::InfiniteDb;
use std::collections::HashMap;

impl AnalysisWorkspace {
    pub fn new(id: WorkspaceId, db: &mut InfiniteDb) -> Self {
        let graph = load_walls(db);
        let active_wall = graph.nodes.keys().next().copied();
        let (last_results, last_analysis) = match active_wall {
            Some(wid) => load_analysis_state(db, wid),
            None => (None, None),
        };
        Self {
            tab: WorkspaceTab::new(id, WorkspaceKind::Analysis),
            header: Some(build_header()),
            pages: Page::all().to_vec(),
            pod_tree: pages_pod_tree(),
            active_analysis: AnalysisKind::SpecialConcreteWall,
            graph,
            active_wall,
            wall_sinks: HashMap::new(),
            nav_triggers: HashMap::new(),
            field_props: HashMap::new(),
            u8_fields: HashMap::new(),
            analysis_actions: HashMap::new(),
            builder_slots: HashMap::new(),
            promote_props: HashMap::new(),
            field_builder: None,
            input_size_class: SizeClass::Full,
            wall_view_sink: None,
            wall_spatial: InMemoryWorldSpatial::default(),
            wall_view_last_pos: None,
            wall_view_panning: false,
            last_results,
            last_analysis,
            results_triggers: HashMap::new(),
        }
    }

    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.tab.title = title.into();
        self
    }

    /// Select a wall; returns true if the selection changed.
    pub fn select_wall(&mut self, id: hypernode::NodeId) -> bool {
        if self.graph.nodes.contains_key(&id) && self.active_wall != Some(id) {
            self.active_wall = Some(id);
            true
        } else {
            false
        }
    }

    pub fn wall_count(&self) -> usize {
        self.graph.nodes.len()
    }

    pub fn next_wall_name(&self) -> String {
        format!("Wall {}", self.wall_count() + 1)
    }
}

fn load_analysis_state(
    db: &mut InfiniteDb,
    wall_id: hypernode::NodeId,
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
