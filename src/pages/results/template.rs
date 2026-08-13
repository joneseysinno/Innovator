use crate::domains::structural::build_page_header::build_split_only_header;
use crate::domains::structural::template_ids::RESULTS;
use crate::pages::template::{PageTemplate, TemplateCtx};
use hyper_ui::{particles::Particle, PageHeaderSlots, TemplateId};

pub struct ResultsTemplate;

impl PageTemplate for ResultsTemplate {
    fn id(&self) -> TemplateId {
        RESULTS
    }

    fn header_slots(&self) -> PageHeaderSlots {
        PageHeaderSlots::None
    }

    fn build_header(&self, ctx: &mut TemplateCtx<'_>) -> Particle {
        build_split_only_header(
            ctx.page_id,
            RESULTS,
            &mut ctx.workspace.page_split_triggers,
            &mut ctx.workspace.page_template_menu_triggers,
        )
    }

    fn build_body(&self, ctx: &mut TemplateCtx<'_>) -> Particle {
        crate::pages::results::build_results(ctx.workspace)
    }
}
