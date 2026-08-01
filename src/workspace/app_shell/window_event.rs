use super::handle_analysis_action::{handle_analysis_action, handle_promote_prop};
use super::handle_app_signal::handle_app_signal;
use super::handle_builder_field::handle_builder_field;
use super::handle_value_changed::handle_value_changed;
use super::handle_workspace_signal::handle_workspace_signal;
use super::layout_areas::layout_areas;
use super::rebuild_active::rebuild_active;
use super::rebuild_seams::rebuild_seams;
use super::sync_chrome_layouts::sync_chrome_layouts;
use super::sync_page_layouts::sync_page_layouts;
use super::AppShell;
use crate::workspace::instance::WorkspaceInstance;
use crate::workspace::screen_class::ScreenClass;
use crate::workspace::signal::WorkspaceSignal;
use crate::workspace::size_class::SizeClass;
use hyper_ui::{apply_signal_text, PointerKind, Rect, UVec2, UiEvent, Vec2};
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;

const LEAF_INPUT_FORM: u32 = 4;
const LEAF_WALL_VIEW: u32 = 5;

pub(crate) fn window_event(
    shell: &mut AppShell,
    event_loop: &ActiveEventLoop,
    _id: WindowId,
    event: WindowEvent,
) {
    let Some(window) = shell.window.clone() else {
        return;
    };

    match &event {
        WindowEvent::CloseRequested => event_loop.exit(),
        WindowEvent::Resized(size) => {
            shell.screen_class = ScreenClass::from_width(size.width);
            shell.window_area =
                Rect::from_xywh(0.0, 0.0, size.width as f32, size.height as f32);
            shell.has_header = shell.active().and_then(|a| a.header()).is_some();
            let (_tabs, _header, pages) = layout_areas(shell.window_area, shell.has_header);
            shell.pages_area = pages;

            let pod = shell.active().and_then(|a| a.pod_tree()).cloned();
            let pages_area = shell.pages_area;
            let window_area = shell.window_area;
            let has_header = shell.has_header;

            let Some(renderer) = shell.renderer.as_mut() else {
                return;
            };
            renderer.resize(*size);
            if let Some(pod) = pod.as_ref() {
                rebuild_seams(pod, pages_area, renderer);
            }
            renderer.ui.layout(window_area);
            if let Some(root) = renderer.ui.tree.root.as_mut() {
                sync_chrome_layouts(root, window_area, has_header);
                if let Some(pod) = pod.as_ref() {
                    sync_page_layouts(root, &pod.leaf_rects(pages_area));
                }
            }
            maybe_update_size_class(shell);
            window.request_redraw();
        }
        WindowEvent::RedrawRequested => {
            let status_id = shell.active().and_then(|a| a.status_id());
            let pages_area = shell.pages_area;
            let window_area = shell.window_area;
            let has_header = shell.has_header;

            let ui_pods = shell.renderer.as_ref().map(|r| r.ui.pods.clone());
            if let (Some(pods), Some(dst)) = (ui_pods, shell.active_mut().and_then(|a| a.pod_tree_mut()))
            {
                *dst = pods;
            }

            let leaves = shell
                .active()
                .and_then(|a| a.pod_tree())
                .map(|p| p.leaf_rects(pages_area));
            let spatial = shell
                .active()
                .and_then(|a| a.wall_spatial())
                .cloned()
                .unwrap_or_default();

            let Some(renderer) = shell.renderer.as_mut() else {
                return;
            };

            while let Ok(msg) = shell.signal_rx.try_recv() {
                if let Some(id) = status_id {
                    apply_signal_text(&mut renderer.ui.tree, id, msg);
                }
            }

            renderer.ui.layout(window_area);
            if let Some(root) = renderer.ui.tree.root.as_mut() {
                sync_chrome_layouts(root, window_area, has_header);
                if let Some(leaves) = leaves.as_ref() {
                    sync_page_layouts(root, leaves);
                }
            }

            // Layer A — wall section camera sized to WallView leaf.
            if let Some(leaves) = leaves.as_ref() {
                if let Some((_, view_rect)) = leaves.iter().find(|(id, _)| *id == LEAF_WALL_VIEW) {
                    renderer.scene.camera.set_screen_size(UVec2::new(
                        view_rect.size.x.max(1.0) as u32,
                        view_rect.size.y.max(1.0) as u32,
                    ));
                    // Fit zoom roughly to a typical 8×144 in section.
                    if renderer.scene.camera.zoom < 3.0 {
                        renderer.scene.camera.zoom = 4.0;
                    }
                }
            }
            renderer
                .scene
                .cull_and_upload(&renderer.device, &renderer.queue, &spatial);

            let focused = renderer.ui.input.focused;
            let screen = [
                renderer.config.width as f32,
                renderer.config.height as f32,
            ];
            renderer.ui.rebuild_draw_lists(
                &renderer.device,
                &renderer.queue,
                &mut renderer.text,
                screen,
                focused,
            );

            if let Some(ctx) = renderer.begin_frame() {
                renderer.end_frame(ctx);
            }
            window.request_redraw();
        }
        other => {
            let tab_triggers = shell.tab_strip.triggers.clone();
            let header_triggers = shell
                .active()
                .and_then(|a| a.header())
                .map(|h| h.triggers.clone())
                .unwrap_or_default();
            let home_actions = shell
                .active()
                .and_then(|a| a.home_actions())
                .cloned()
                .unwrap_or_default();
            let wall_sinks = shell
                .active()
                .and_then(|a| a.wall_sinks())
                .cloned()
                .unwrap_or_default();
            let nav_triggers = shell
                .active()
                .and_then(|a| a.nav_triggers())
                .cloned()
                .unwrap_or_default();
            let results_triggers = shell
                .active()
                .and_then(|a| a.results_triggers())
                .cloned()
                .unwrap_or_default();
            let analysis_actions = shell
                .active()
                .and_then(|a| a.analysis_actions())
                .cloned()
                .unwrap_or_default();
            let promote_props = shell
                .active()
                .and_then(|a| a.promote_props())
                .cloned()
                .unwrap_or_default();
            let field_props = shell
                .active()
                .and_then(|a| a.field_props())
                .cloned()
                .unwrap_or_default();
            let builder_slots = shell
                .active()
                .and_then(|a| a.builder_slots())
                .cloned()
                .unwrap_or_default();
            let wall_view_sink = shell.active().and_then(|a| a.wall_view_sink());
            let pages_area = shell.pages_area;
            let has_pods = shell.active().and_then(|a| a.pod_tree()).is_some();

            let mut app_signal = None;
            let mut ws_signal = None;
            let mut analysis_action = None;
            let mut promote_key = None;
            let mut field_commit = None;
            let mut builder_commit = None;
            let mut size_class_rebuild = false;
            let mut wall_view_ptr: Option<(PointerKind, Vec2)> = None;
            let mut wheel_zoom: Option<(Vec2, f32)> = None;

            // Wheel zoom over wall view leaf
            if let WindowEvent::MouseWheel { delta, .. } = other {
                let cursor = shell
                    .renderer
                    .as_ref()
                    .map(|r| r.ui.input.cursor)
                    .unwrap_or(Vec2::ZERO);
                let over_view = shell
                    .active()
                    .and_then(|a| a.pod_tree())
                    .and_then(|p| {
                        p.leaf_rects(pages_area)
                            .into_iter()
                            .find(|(id, _)| *id == LEAF_WALL_VIEW)
                    });
                if let Some((_, view_rect)) = over_view {
                    if view_rect.contains(cursor) {
                        let factor = match delta {
                            winit::event::MouseScrollDelta::LineDelta(_, y) => {
                                if *y > 0.0 {
                                    1.1
                                } else {
                                    1.0 / 1.1
                                }
                            }
                            winit::event::MouseScrollDelta::PixelDelta(p) => {
                                if p.y > 0.0 {
                                    1.05
                                } else {
                                    1.0 / 1.05
                                }
                            }
                        };
                        wheel_zoom = Some((cursor, factor));
                    }
                }
            }

            {
                let Some(renderer) = shell.renderer.as_mut() else {
                    return;
                };

                if let Some((cursor, factor)) = wheel_zoom {
                    renderer.scene.camera.zoom_at(cursor, factor);
                }

                if has_pods {
                    let seam_events = renderer.ui.seams.handle_event(
                        other,
                        renderer.ui.input.cursor,
                        &mut renderer.ui.pods,
                        pages_area,
                    );
                    if !seam_events.is_empty() {
                        renderer.ui.tree.mark_all_dirty();
                        size_class_rebuild = true;
                    }
                }
                if let Some(icon) = renderer.ui.seams.cursor_icon() {
                    window.set_cursor(icon);
                }

                let events = renderer.ui.input.route(other, &mut renderer.ui.tree);
                for ev in events {
                    match ev {
                        UiEvent::TriggerFired(id) => {
                            if let Some(sig) = tab_triggers.get(&id).copied() {
                                app_signal = Some(sig);
                            } else if let Some(sig) = home_actions.get(&id).copied() {
                                app_signal = Some(sig);
                            } else if let Some(sig) = header_triggers.get(&id).copied() {
                                ws_signal = Some(sig);
                            } else if let Some(sig) = nav_triggers.get(&id).copied() {
                                ws_signal = Some(sig);
                            } else if let Some(sig) = results_triggers.get(&id).copied() {
                                ws_signal = Some(sig);
                            } else if let Some(action) = analysis_actions.get(&id).copied() {
                                analysis_action = Some(action);
                            } else if let Some(key) = promote_props.get(&id).cloned() {
                                promote_key = Some(key);
                            }
                        }
                        UiEvent::FieldCommit { id, value } => {
                            if builder_slots.contains_key(&id) {
                                builder_commit = Some((id, value));
                            } else if field_props.contains_key(&id) {
                                field_commit = Some((id, value));
                            }
                        }
                        UiEvent::SinkPointer { id, pos, kind } => {
                            if Some(id) == wall_view_sink {
                                wall_view_ptr = Some((kind, pos));
                            } else if matches!(kind, PointerKind::Up) {
                                if let Some(node_id) = wall_sinks.get(&id).copied() {
                                    ws_signal = Some(WorkspaceSignal::WallSelected(node_id));
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }

            if let Some((kind, pos)) = wall_view_ptr {
                apply_wall_view_pointer(shell, kind, pos);
            }

            if size_class_rebuild {
                maybe_update_size_class(shell);
            }
            if let Some((id, value)) = builder_commit {
                handle_builder_field(shell, id, value);
            }
            if let Some((id, value)) = field_commit {
                handle_value_changed(shell, id, value);
            }
            if let Some(action) = analysis_action {
                handle_analysis_action(shell, action);
            }
            if let Some(key) = promote_key {
                handle_promote_prop(shell, key);
            }
            if let Some(sig) = ws_signal {
                handle_workspace_signal(shell, sig);
            }
            if let Some(sig) = app_signal {
                handle_app_signal(shell, sig);
            }
            window.request_redraw();
        }
    }
}

fn apply_wall_view_pointer(shell: &mut AppShell, kind: PointerKind, pos: Vec2) {
    let active_id = shell.active_id;
    let Some(idx) = shell.workspaces.iter().position(|w| w.id() == active_id) else {
        return;
    };

    let mut pan_delta = None;
    let mut zoom = None;
    {
        let WorkspaceInstance::Analysis(ws) = &mut shell.workspaces[idx] else {
            return;
        };
        match kind {
            PointerKind::Down => {
                ws.wall_view_panning = true;
                ws.wall_view_last_pos = Some(pos);
            }
            PointerKind::Up => {
                ws.wall_view_panning = false;
                ws.wall_view_last_pos = None;
            }
            PointerKind::Move if ws.wall_view_panning => {
                if let Some(prev) = ws.wall_view_last_pos {
                    pan_delta = Some(Vec2::new(pos.x - prev.x, pos.y - prev.y));
                }
                ws.wall_view_last_pos = Some(pos);
            }
            PointerKind::Scroll { delta_y } => {
                let factor = if delta_y > 0.0 { 1.1 } else { 1.0 / 1.1 };
                zoom = Some((pos, factor));
            }
            _ => {}
        }
    }

    if let Some(renderer) = shell.renderer.as_mut() {
        if let Some(delta) = pan_delta {
            renderer.scene.camera.pan(delta);
        }
        if let Some((anchor, factor)) = zoom {
            renderer.scene.camera.zoom_at(anchor, factor);
        }
    }
}

fn maybe_update_size_class(shell: &mut AppShell) {
    let pages_area = shell.pages_area;
    let width = shell
        .active()
        .and_then(|a| a.pod_tree())
        .and_then(|p| {
            p.leaf_rects(pages_area)
                .into_iter()
                .find(|(id, _)| *id == LEAF_INPUT_FORM)
                .map(|(_, r)| r.size.x)
        });
    let Some(width) = width else {
        return;
    };
    let new_class = SizeClass::from_width(width);
    let changed = match shell.active_mut() {
        Some(WorkspaceInstance::Analysis(ws)) if ws.input_size_class != new_class => {
            ws.input_size_class = new_class;
            true
        }
        _ => false,
    };
    if changed {
        let mut renderer = match shell.renderer.take() {
            Some(r) => r,
            None => return,
        };
        rebuild_active(shell, &mut renderer);
        shell.renderer = Some(renderer);
    }
}
