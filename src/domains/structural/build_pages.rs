use super::build_page_header::{build_analysis_page_header, build_split_only_header};
use super::io_kind::IoKind;
use super::workspace::StructuralWorkspace;
use crate::pages::{build_analysis, build_navigation, build_results};
use crate::pages::placeholder::build_empty_pod;
use hyper_ui::particles::{
    Particle, StackParticle, TriggerParticle, ViewParticle, ViewportParticle,
};
use hyper_ui::{
    build_pod_icon_rail, effective_icon_rail, wrap_pod_column, IconRailSide, PageHeaderSlots,
    PageId, PageNode,
};

/// Page region particle — one child per **Shown** page, plus a rail for Hidden.
pub fn build_pages(ws: &mut StructuralWorkspace) -> Particle {
    ws.icon_rail_triggers.clear();
    ws.pod_collapse_triggers.clear();
    ws.page_split_triggers.clear();
    ws.page_show_triggers.clear();
    ws.page_viewport_ids.clear();
    ws.analysis_header_status_id = None;

    let shown_ids: Vec<PageId> = ws
        .page_tree
        .leaves()
        .into_iter()
        .filter(|p| p.state.resolved() == hyper_ui::Visibility::Shown)
        .map(|p| p.id)
        .collect();
    let mut children = Vec::with_capacity(shown_ids.len());

    for page_id in &shown_ids {
        let page = ws.page_tree.find(*page_id).cloned().expect("page leaf");
        let ios = ws.page_ios.get(page_id).cloned().unwrap_or_default();
        let particle = build_one_page(ws, &page, &ios);
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
    page: &PageNode,
    ios: &[(hyper_ui::PodId, IoKind)],
) -> Particle {
    let raw_content = build_page_content(ws, ios);
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
        let header = match header_cfg.slots {
            PageHeaderSlots::Custom => {
                // Analysis-style header with live ratios when this page has InputForm/WallView.
                let is_analysis = ios.iter().any(|(_, k)| {
                    matches!(k, IoKind::InputForm | IoKind::WallView)
                });
                if is_analysis {
                    let (p, status_id) = build_analysis_page_header(
                        page.id,
                        ws.last_analysis.as_ref(),
                        &mut ws.page_split_triggers,
                    );
                    ws.analysis_header_status_id = Some(status_id);
                    p
                } else {
                    build_split_only_header(page.id, &mut ws.page_split_triggers)
                }
            }
            PageHeaderSlots::None => {
                build_split_only_header(page.id, &mut ws.page_split_triggers)
            }
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

fn build_page_content(
    ws: &mut StructuralWorkspace,
    ios: &[(hyper_ui::PodId, IoKind)],
) -> Particle {
    let kinds: Vec<IoKind> = ios.iter().map(|(_, k)| *k).collect();
    match kinds.as_slice() {
        [IoKind::WallList, IoKind::WallSummary] => build_navigation(ws),
        [IoKind::InputForm, IoKind::WallView] => build_analysis(ws),
        [IoKind::ResultsTable, IoKind::Status] => build_results(ws),
        [IoKind::Empty] | [] => build_empty_pod("Empty page"),
        _ => {
            // Generic: build each IO independently in pod order (vertical stack).
            let mut pods = Vec::new();
            for (_, kind) in ios {
                pods.push(build_single_io(ws, *kind));
            }
            Particle::Stack(StackParticle::column(pods).with_gap(0.0))
        }
    }
}

fn build_single_io(ws: &mut StructuralWorkspace, kind: IoKind) -> Particle {
    match kind {
        IoKind::WallList => {
            let list = crate::pages::navigation::wall_list::build_wall_list(
                &ws.graph,
                ws.active_wall,
            );
            ws.wall_sinks.extend(list.sinks);
            ws.nav_triggers.extend(list.triggers);
            list.particle
        }
        IoKind::WallSummary => {
            crate::pages::navigation::wall_summary::build_wall_summary(&ws.graph, ws.active_wall)
                .particle
        }
        IoKind::InputForm => {
            let node = ws
                .active_wall
                .and_then(|id| ws.graph.nodes.get(&id))
                .cloned();
            let form = crate::pages::analysis::input_form::build_input_form(
                node.as_ref(),
                ws.input_size_class,
                ws.field_builder.as_ref(),
            );
            ws.field_props.extend(form.field_props);
            ws.u8_fields.extend(form.u8_fields);
            ws.analysis_actions.extend(form.actions);
            ws.builder_slots.extend(form.builder_slots);
            ws.promote_props.extend(form.promote_props);
            form.particle
        }
        IoKind::WallView => {
            let node = ws
                .active_wall
                .and_then(|id| ws.graph.nodes.get(&id))
                .cloned();
            let view = crate::pages::analysis::wall_view::build_wall_view(node.as_ref());
            ws.wall_view_sink = Some(view.sink_id);
            ws.wall_spatial = view.spatial;
            view.particle
        }
        IoKind::ResultsTable => {
            let checks = ws
                .last_results
                .as_ref()
                .map(crate::results::parse_checks)
                .unwrap_or_default();
            crate::pages::results::results_table::build_results_table(&checks).particle
        }
        IoKind::Status => {
            let status = crate::pages::results::status::build_status(ws.last_analysis.as_ref());
            ws.results_triggers.extend(status.triggers);
            status.particle
        }
        IoKind::Empty => build_empty_pod("Empty"),
    }
}
