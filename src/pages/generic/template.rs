use crate::domains::structural::build_page_header::build_split_only_header;
use crate::domains::structural::template_ids::GENERIC;
use crate::pages::placeholder::build_empty_pod;
use crate::pages::template::{PageTemplate, TemplateCtx};
use crate::workspace::graph_containers::binding_children;
use hyper_ui::{particles::{Particle, StackParticle}, PageHeaderSlots, TemplateId};
use hypernode::{HyperNode, PropValue};

pub struct GenericTemplate;

impl PageTemplate for GenericTemplate {
    fn id(&self) -> TemplateId {
        GENERIC
    }

    fn header_slots(&self) -> PageHeaderSlots {
        PageHeaderSlots::None
    }

    fn build_header(&self, ctx: &mut TemplateCtx<'_>) -> Particle {
        build_split_only_header(
            ctx.page_id,
            GENERIC,
            &mut ctx.workspace.page_split_triggers,
            &mut ctx.workspace.page_template_menu_triggers,
        )
    }

    fn build_body(&self, ctx: &mut TemplateCtx<'_>) -> Particle {
        let pods = binding_children(ctx.graph, ctx.page.node_id)
            .into_iter()
            .filter_map(|id| ctx.graph.nodes.get(&id))
            .map(|node| {
                let template = match node.get_prop("template_id") {
                    Some(PropValue::Text(id)) => id.as_str(),
                    _ => "generic",
                };
                build_empty_pod(&format!("{} ({template})", node.label))
            })
            .collect::<Vec<_>>();
        if pods.is_empty() {
            build_empty_pod("Empty page")
        } else {
            Particle::Stack(StackParticle::column(pods).with_gap(0.0))
        }
    }
}
