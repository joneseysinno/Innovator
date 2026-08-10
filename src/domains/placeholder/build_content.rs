//! Build particle tree for a placeholder workspace (stub IO in pods).

use super::stub_io::build_stub_stack;
use super::PlaceholderWorkspace;
use hyper_ui::particles::{
    Particle, StackParticle, SurfaceParticle, TriggerParticle, ViewParticle, ViewportParticle,
};
use hyper_ui::{PageId, PageNode, Visibility};

pub fn build_content(ws: &mut PlaceholderWorkspace) -> Particle {
    ws.page_show_triggers.clear();
    ws.pod_collapse_triggers.clear();
    ws.page_viewport_ids.clear();

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
        children.push(build_one_page(ws, &page));
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

fn build_one_page(ws: &mut PlaceholderWorkspace, page: &PageNode) -> Particle {
    let mut pod_children = Vec::new();
    for pod in &page.pods.pods {
        let labels = ws
            .stub_ios
            .get(&(page.id, pod.id))
            .map(|v| v.iter().map(|s| s.as_str()).collect::<Vec<_>>())
            .unwrap_or_default();
        let content = build_stub_stack(&labels, pod);
        // Collapse trigger title bar
        let title = TriggerParticle::new(pod.title.clone());
        ws.pod_collapse_triggers.insert(title.id, pod.id);
        let column = StackParticle::column(vec![Particle::Trigger(title), content]).with_gap(2.0);
        pod_children.push(Particle::Surface(
            SurfaceParticle::new([0.10, 0.11, 0.13, 1.0])
                .with_padding(2.0)
                .with_radius(0.0)
                .with_child(Particle::Stack(column)),
        ));
    }

    let stack = Particle::Stack(StackParticle::column(pod_children).with_gap(4.0));
    let viewport = ViewportParticle::new().with_child(stack);
    ws.page_viewport_ids.insert(page.id, viewport.id);
    Particle::Viewport(viewport)
}
