use super::template_ids::GENERIC;
use super::workspace::StructuralWorkspace;
use crate::pages::registry::{page_templates, template_for};
use crate::pages::template::TemplateCtx;
use hyper_ui::particles::{
    Particle, StackParticle, TriggerParticle, ViewParticle, ViewportParticle,
};
use hyper_ui::{
    build_pod_icon_rail, effective_icon_rail, wrap_pod_column, IconRailSide, PageId, PageNode,
    TemplateId,
};
use hypernode::Graph;
use std::collections::HashMap;

/// Page region particle — one child per **Shown** page, plus a rail for Hidden.
pub fn build_pages(ws: &mut StructuralWorkspace, graph: &Graph) -> Particle {
    ws.icon_rail_triggers.clear();
    ws.pod_collapse_triggers.clear();
    ws.page_split_triggers.clear();
    ws.page_template_menu_triggers.clear();
    ws.page_show_triggers.clear();
    ws.page_viewport_ids.clear();
    ws.analysis_header_status_id = None;

    // Every Structural page needs header chrome for the type switcher + split.
    for page in &mut ws.page_tree.pages {
        if page.header.is_none() {
            page.header = Some(hyper_ui::PageHeaderConfig {
                height: 32.0,
                slots: hyper_ui::PageHeaderSlots::None,
            });
        }
    }

    let shown_ids: Vec<PageId> = ws
        .page_tree
        .leaves()
        .into_iter()
        .filter(|p| p.state.resolved() == hyper_ui::Visibility::Shown)
        .map(|p| p.id)
        .collect();
    let mut children = Vec::with_capacity(shown_ids.len());
    let templates = page_templates();

    for page_id in &shown_ids {
        let page = ws.page_tree.find(*page_id).cloned().expect("page leaf");
        let particle = build_one_page(ws, graph, &page, &templates);
        children.push(particle);
    }

    let pages_row = Particle::Stack(StackParticle::row(children).with_gap(0.0));

    let hidden: Vec<_> = ws
        .page_tree
        .leaves()
        .into_iter()
        .filter(|p| p.state.resolved() == hyper_ui::Visibility::Hidden)
        .cloned()
        .collect();

    let content = if hidden.is_empty() {
        pages_row
    } else {
        let rail = build_page_rail(&hidden, &mut ws.page_show_triggers);
        Particle::Stack(StackParticle::row(vec![rail, pages_row]).with_gap(0.0))
    };

    let mut pages_view = ViewParticle::new("pages");
    pages_view.child = Some(Box::new(content));
    Particle::View(pages_view)
}

/// Icon rail of Hidden pages — click focuses that page so cascade brings it back.
fn build_page_rail(
    hidden: &[PageNode],
    triggers: &mut std::collections::HashMap<hyper_ui::ParticleId, PageId>,
) -> Particle {
    let mut items = Vec::with_capacity(hidden.len());
    for page in hidden {
        let glyph = if page.state.icon.is_empty() {
            "·".to_string()
        } else {
            page.state.icon.clone()
        };
        let t = TriggerParticle::new(glyph);
        triggers.insert(t.id, page.id);
        items.push(Particle::Trigger(t));
    }
    let column = StackParticle::column(items).with_gap(4.0);
    Particle::Surface(
        hyper_ui::particles::SurfaceParticle::new([0.12, 0.13, 0.16, 1.0])
            .with_padding(2.0)
            .with_radius(0.0)
            .with_child(Particle::Stack(column)),
    )
}

fn build_one_page(
    ws: &mut StructuralWorkspace,
    graph: &Graph,
    page: &PageNode,
    templates: &HashMap<TemplateId, Box<dyn crate::pages::template::PageTemplate>>,
) -> Particle {
    let template_id = ws.page_templates.get(&page.id).copied().unwrap_or(GENERIC);
    let template = template_for(templates, template_id);
    let raw_content = {
        let mut ctx = TemplateCtx {
            workspace: ws,
            graph,
            page,
            page_id: page.id,
        };
        template.build_body(&mut ctx)
    };
    let content = wrap_pods_with_shell(ws, page, raw_content);
    let viewport = ViewportParticle::new().with_child(content);
    ws.page_viewport_ids.insert(page.id, viewport.id);
    let content = Particle::Viewport(viewport);

    let mut body_children = Vec::new();
    let rail_cfg = effective_icon_rail(page);
    if let Some(rail) = &rail_cfg {
        if let Some(rail_particle) = build_pod_icon_rail(page, &mut ws.icon_rail_triggers) {
            match rail.side {
                IconRailSide::Left => {
                    body_children.push(rail_particle);
                    body_children.push(content);
                }
                IconRailSide::Right => {
                    body_children.push(content);
                    body_children.push(rail_particle);
                }
            }
        } else {
            body_children.push(content);
        }
    } else {
        body_children.push(content);
    }

    let body = Particle::Stack(StackParticle::row(body_children).with_gap(0.0));

    let mut column = Vec::new();
    if let Some(header_cfg) = &page.header {
        let _configured_slots = header_cfg.slots;
        let _template_slots = template.header_slots();
        let header = {
            let mut ctx = TemplateCtx {
                workspace: ws,
                graph,
                page,
                page_id: page.id,
            };
            template.build_header(&mut ctx)
        };
        column.push(header);
    }
    column.push(body);

    Particle::Stack(StackParticle::column(column).with_gap(0.0))
}

/// Wrap each pod child with uniform bordered chrome (title bar + body).
/// Collapsed pods omit the IO body particle — state lives on the workspace.
fn wrap_pods_with_shell(
    ws: &mut StructuralWorkspace,
    page: &PageNode,
    content: Particle,
) -> Particle {
    let Particle::Stack(stack) = content else {
        return content;
    };
    let (wrapped, triggers) = wrap_pod_column(&page.pods.pods, stack.children);
    for (trigger_id, pod_id) in triggers {
        ws.pod_collapse_triggers.insert(trigger_id, pod_id);
    }
    wrapped
}
