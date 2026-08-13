use crate::domains::structural::template_ids::GRAPH_VIEW;
use crate::pages::template::{PageTemplate, TemplateCtx};
use hyper_ui::{particles::Particle, PageHeaderSlots, TemplateId};

/// Page template for the graph-view workspace.
pub struct GraphViewTemplate;

impl PageTemplate for GraphViewTemplate {
    fn id(&self) -> TemplateId {
        GRAPH_VIEW
    }

    fn header_slots(&self) -> PageHeaderSlots {
        PageHeaderSlots::None
    }

    fn build_header(&self, _ctx: &mut TemplateCtx<'_>) -> Particle {
        Particle::Source(hyper_ui::particles::SourceParticle::muted(""))
    }

    fn build_body(&self, ctx: &mut TemplateCtx<'_>) -> Particle {
        // Graph-view body is built by GraphViewWorkspace::build_content, not
        // Structural's page registry. This arm exists so the TemplateId is
        // registered; Structural should never dispatch here.
        let _ = ctx;
        Particle::Source(hyper_ui::particles::SourceParticle::secondary(
            "graph_view template — use GraphView workspace",
        ))
    }
}
