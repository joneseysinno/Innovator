use crate::domains::structural::build_page_header::build_analysis_page_header;
use crate::domains::structural::graph_wires::{pod_node_id, results_for_pod};
use crate::domains::structural::template_ids::ANALYSIS;
use crate::domains::structural::template_ids::RESULTS_TABLE;
use crate::pages::template::{PageTemplate, TemplateCtx};
use hyper_ui::{particles::Particle, PageHeaderSlots, TemplateId};

pub struct AnalysisTemplate;

impl PageTemplate for AnalysisTemplate {
    fn id(&self) -> TemplateId {
        ANALYSIS
    }

    fn header_slots(&self) -> PageHeaderSlots {
        PageHeaderSlots::Custom
    }

    fn build_header(&self, ctx: &mut TemplateCtx<'_>) -> Particle {
        // Header status is supplied by the Results table UIView's role-tagged
        // Binding child, rather than the workspace's cached analysis output.
        let results = pod_node_id(ctx.workspace, RESULTS_TABLE)
            .and_then(|pod| results_for_pod(ctx.graph, pod));
        let (header, status_id) = build_analysis_page_header(
            ctx.page_id,
            results,
            &mut ctx.workspace.page_split_triggers,
        );
        ctx.workspace.analysis_header_status_id = Some(status_id);
        header
    }

    fn build_body(&self, ctx: &mut TemplateCtx<'_>) -> Particle {
        crate::pages::analysis::build_analysis(ctx.workspace, ctx.graph)
    }
}
