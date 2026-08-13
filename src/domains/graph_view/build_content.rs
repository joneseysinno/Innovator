//! Build particle tree for the graph-view workspace.

use super::GraphViewWorkspace;
use crate::pages::graph_view::{build_canvas_pod, build_inspector_pod, sync_graph_view};
use hyper_ui::particles::{
    Particle, StackParticle, SurfaceParticle, TriggerParticle, ViewParticle, ViewportParticle,
};
use hyper_ui::{
    build_pod_icon_rail, effective_icon_rail, wrap_pod_column, IconRailSide, PageId, PageNode,
    Visibility,
};
use hypernode::{Graph, NodeId};

pub fn build_content(
    ws: &mut GraphViewWorkspace,
    graph: &Graph,
    active_workspace: Option<NodeId>,
) -> Particle {
    ws.page_show_triggers.clear();
    ws.pod_collapse_triggers.clear();
    ws.icon_rail_triggers.clear();
    ws.page_viewport_ids.clear();

    sync_graph_view(ws, graph, active_workspace);

    let shown_ids: Vec<PageId> = ws
        .page_tree
        .leaves()
        .into_iter()
        .filter(|p| p.state.resolved() == Visibility::Shown)
        .map(|p| p.id)
        .collect();

    let mut children = Vec::with_capacity(shown_ids.len());
    for page_id in &shown_ids {
        let page = ws.page_tree.find(*page_id).cloned().expect("page leaf");
        children.push(build_one_page(ws, graph, &page));
    }

    let pages_row = Particle::Stack(StackParticle::row(children).with_gap(0.0));

    let hidden: Vec<_> = ws
        .page_tree
        .leaves()
        .into_iter()
        .filter(|p| p.state.resolved() == Visibility::Hidden)
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
        SurfaceParticle::new([0.12, 0.13, 0.16, 1.0])
            .with_padding(2.0)
            .with_radius(0.0)
            .with_child(Particle::Stack(column)),
    )
}

fn build_one_page(
    ws: &mut GraphViewWorkspace,
    graph: &Graph,
    page: &PageNode,
) -> Particle {
    let mut bodies = Vec::with_capacity(page.pods.pods.len());
    for (i, _) in page.pods.pods.iter().enumerate() {
        if i == 0 {
            bodies.push(build_canvas_pod(ws));
        } else {
            bodies.push(build_inspector_pod(ws, graph));
        }
    }
    if bodies.is_empty() {
        bodies.push(build_canvas_pod(ws));
    }

    let (stack, triggers) = wrap_pod_column(&page.pods.pods, bodies);
    for (trigger_id, pod_id) in triggers {
        ws.pod_collapse_triggers.insert(trigger_id, pod_id);
    }

    let viewport = ViewportParticle::new().with_child(stack);
    ws.page_viewport_ids.insert(page.id, viewport.id);
    let content = Particle::Viewport(viewport);

    let mut body_children = Vec::new();
    if let Some(rail) = effective_icon_rail(page) {
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

    Particle::Stack(StackParticle::row(body_children).with_gap(0.0))
}
