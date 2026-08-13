use crate::domains::structural::template_ids::NAVIGATION;
use crate::domains::structural::build_page_header::build_split_only_header;
use crate::pages::template::{PageTemplate, TemplateCtx};
use hyper_ui::{particles::Particle, PageHeaderSlots, TemplateId};

pub struct NavigationTemplate;

impl PageTemplate for NavigationTemplate {
    fn id(&self) -> TemplateId {
        NAVIGATION
    }

    fn header_slots(&self) -> PageHeaderSlots {
        PageHeaderSlots::None
    }

    fn build_header(&self, ctx: &mut TemplateCtx<'_>) -> Particle {
        build_split_only_header(
            ctx.page_id,
            NAVIGATION,
            &mut ctx.workspace.page_split_triggers,
            &mut ctx.workspace.page_template_menu_triggers,
        )
    }

    fn build_body(&self, ctx: &mut TemplateCtx<'_>) -> Particle {
        crate::pages::navigation::build_navigation(ctx.workspace, ctx.graph)
    }
}
