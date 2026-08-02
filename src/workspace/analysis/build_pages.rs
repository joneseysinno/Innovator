use super::build_icon_rail::{build_icon_rail, default_pod_icons};
use super::build_page_header::{build_analysis_page_header, build_split_only_header};
use super::io_kind::IoKind;
use super::AnalysisWorkspace;
use crate::pages::{build_analysis, build_navigation, build_results};
use crate::pages::placeholder::build_empty_pod;
use hyper_ui::particles::{Particle, StackParticle, ViewParticle};
use hyper_ui::{IconRailSide, PageHeaderSlots, PageId, PageNode};

/// Page region particle for an Analysis workspace — one child per PageTree leaf.
pub fn build_pages(ws: &mut AnalysisWorkspace) -> Particle {
    ws.icon_rail_triggers.clear();
    ws.page_split_triggers.clear();
    ws.analysis_header_status_id = None;

    let page_ids: Vec<PageId> = ws.page_tree.leaves().iter().map(|p| p.id).collect();
    let mut children = Vec::with_capacity(page_ids.len());

    for page_id in page_ids {
        let page = ws.page_tree.find(page_id).cloned().expect("page leaf");
        let ios = ws.page_ios.get(&page_id).cloned().unwrap_or_default();
        let particle = build_one_page(ws, &page, &ios);
        children.push(particle);
    }

    let mut pages_view = ViewParticle::new("pages");
    pages_view.child = Some(Box::new(Particle::Stack(
        StackParticle::row(children).with_gap(0.0),
    )));
    Particle::View(pages_view)
}

fn build_one_page(
    ws: &mut AnalysisWorkspace,
    page: &PageNode,
    ios: &[(u32, IoKind)],
) -> Particle {
    let content = build_page_content(ws, ios);

    let mut body_children = Vec::new();

    if let Some(rail) = &page.icon_rail {
        let leaf_count = page.pod_tree.leaf_rects(hyper_ui::Rect::from_xywh(0.0, 0.0, 1.0, 1.0)).len();
        let icons = default_pod_icons(leaf_count);
        let rail_particle = build_icon_rail(page, &icons, &mut ws.icon_rail_triggers);
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

fn build_page_content(ws: &mut AnalysisWorkspace, ios: &[(u32, IoKind)]) -> Particle {
    let kinds: Vec<IoKind> = ios.iter().map(|(_, k)| *k).collect();
    match kinds.as_slice() {
        [IoKind::WallList, IoKind::WallSummary] => build_navigation(ws),
        [IoKind::InputForm, IoKind::WallView] => build_analysis(ws),
        [IoKind::ResultsTable, IoKind::Status] => build_results(ws),
        [IoKind::Empty] | [] => build_empty_pod("Empty page"),
        _ => {
            // Generic: build each IO independently in pod order.
            let mut pods = Vec::new();
            for (_, kind) in ios {
                pods.push(build_single_io(ws, *kind));
            }
            // Prefer row for two vertical-split style pages, column otherwise.
            if pods.len() == 2 {
                Particle::Stack(StackParticle::row(pods).with_gap(0.0))
            } else {
                Particle::Stack(StackParticle::column(pods).with_gap(0.0))
            }
        }
    }
}

fn build_single_io(ws: &mut AnalysisWorkspace, kind: IoKind) -> Particle {
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
