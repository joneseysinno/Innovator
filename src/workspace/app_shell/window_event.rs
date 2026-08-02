use super::handle_analysis_action::{handle_analysis_action, handle_promote_prop};
use super::handle_app_signal::handle_app_signal;
use super::handle_builder_field::handle_builder_field;
use super::handle_page_signal::handle_page_signal;
use super::handle_value_changed::handle_value_changed;
use super::handle_workspace_signal::handle_workspace_signal;
use super::layout_areas::layout_areas;
use super::page_context_menu::PageContextMenu;
use super::rebuild_active::rebuild_active;
use super::rebuild_seams::rebuild_seams;
use super::sync_chrome_layouts::sync_chrome_layouts;
use super::sync_from_page_tree::sync_from_page_tree;
use super::AppShell;
use crate::workspace::analysis::IoKind;
use crate::workspace::instance::WorkspaceInstance;
use crate::workspace::screen_class::ScreenClass;
use crate::workspace::signal::WorkspaceSignal;
use crate::workspace::size_class::SizeClass;
use hyper_ui::layout::{arrange_particle, LayoutBox};
use hyper_ui::{
    apply_signal_text, PageSide, PointerKind, Rect, SeamDirection, SeamRatioAction, UVec2, UiEvent,
    Vec2,
};
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::WindowId;

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

            let mut renderer = match shell.renderer.take() {
                Some(r) => r,
                None => return,
            };
            renderer.resize(*size);
            rebuild_active(shell, &mut renderer);
            shell.renderer = Some(renderer);

            maybe_update_size_class(shell);
            window.request_redraw();
        }
        WindowEvent::RedrawRequested => {
            let status_id = shell.active().and_then(|a| a.status_id());
            let pages_area = shell.pages_area;
            let window_area = shell.window_area;
            let has_header = shell.has_header;
            let pending_menu = shell.pending_context_menu.clone();

            let spatial = shell
                .active()
                .and_then(|a| a.wall_spatial())
                .cloned()
                .unwrap_or_default();

            let wall_view_rect = shell
                .active()
                .and_then(|a| a.analysis())
                .and_then(|ws| ws.io_rect(pages_area, IoKind::WallView));

            // Take renderer so we can sync from analysis without dual borrows.
            let mut renderer = match shell.renderer.take() {
                Some(r) => r,
                None => return,
            };

            while let Ok(msg) = shell.signal_rx.try_recv() {
                if let Some(id) = status_id {
                    apply_signal_text(&mut renderer.ui.tree, id, msg);
                }
            }

            renderer.ui.layout(window_area);
            if let Some(root) = renderer.ui.tree.root.as_mut() {
                sync_chrome_layouts(root, window_area, has_header);
                if let Some(WorkspaceInstance::Analysis(ws)) = shell.active() {
                    sync_from_page_tree(root, ws, pages_area);
                }
                if let Some(menu) = pending_menu.as_ref() {
                    let menu_rect = Rect::from_xywh(menu.cursor.x, menu.cursor.y, 180.0, 200.0);
                    position_context_menu(root, menu_rect);
                }
            }

            if let Some(view_rect) = wall_view_rect {
                renderer.scene.camera.set_screen_size(UVec2::new(
                    view_rect.size.x.max(1.0) as u32,
                    view_rect.size.y.max(1.0) as u32,
                ));
                if renderer.scene.camera.zoom < 3.0 {
                    renderer.scene.camera.zoom = 4.0;
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
            shell.renderer = Some(renderer);
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
            let icon_rail_triggers = shell
                .active()
                .and_then(|a| a.analysis())
                .map(|ws| ws.icon_rail_triggers.clone())
                .unwrap_or_default();
            let page_split_triggers = shell
                .active()
                .and_then(|a| a.analysis())
                .map(|ws| ws.page_split_triggers.clone())
                .unwrap_or_default();
            let context_menu_triggers = shell.context_menu_triggers.clone();
            let pages_area = shell.pages_area;
            let has_pages = shell.active().and_then(|a| a.page_tree()).is_some();

            let mut app_signal = None;
            let mut ws_signal = None;
            let mut page_signal = None;
            let mut analysis_action = None;
            let mut promote_key = None;
            let mut field_commit = None;
            let mut builder_commit = None;
            let mut size_class_rebuild = false;
            let mut wall_view_ptr: Option<(PointerKind, Vec2)> = None;
            let mut wheel_zoom: Option<(Vec2, f32)> = None;
            let mut open_context_menu: Option<PageContextMenu> = None;
            let mut dismiss_context_menu = false;
            let mut page_ratio_action: Option<(usize, SeamRatioAction)> = None;
            let mut pod_ratio_action: Option<(usize, SeamRatioAction)> = None;

            if let WindowEvent::MouseWheel { delta, .. } = other {
                let cursor = shell
                    .renderer
                    .as_ref()
                    .map(|r| r.ui.input.cursor)
                    .unwrap_or(Vec2::ZERO);
                let over_view = shell
                    .active()
                    .and_then(|a| a.analysis())
                    .and_then(|ws| ws.io_rect(pages_area, IoKind::WallView));
                if let Some(view_rect) = over_view {
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

            if let WindowEvent::MouseInput {
                state: winit::event::ElementState::Pressed,
                button: winit::event::MouseButton::Left,
                ..
            } = other
            {
                if shell.pending_context_menu.is_some() {
                    dismiss_context_menu = true;
                }
            }

            {
                let Some(renderer) = shell.renderer.as_mut() else {
                    return;
                };

                if let Some((cursor, factor)) = wheel_zoom {
                    renderer.scene.camera.zoom_at(cursor, factor);
                }

                if has_pages {
                    let cursor = renderer.ui.input.cursor;
                    let (page_events, page_action) =
                        renderer.ui.page_seams.handle_event_with(other, cursor);
                    for ev in &page_events {
                        if let UiEvent::PageSeamRightClick {
                            seam_id,
                            cursor,
                            direction,
                        } = ev
                        {
                            let side = page_side_under_cursor(
                                pages_area,
                                *cursor,
                                *direction,
                            );
                            open_context_menu = Some(PageContextMenu {
                                seam_id: *seam_id,
                                cursor: *cursor,
                                direction: *direction,
                                side,
                            });
                        }
                    }
                    page_ratio_action = page_action;

                    let cursor = renderer.ui.input.cursor;
                    let (_pod_events, pod_action) =
                        renderer.ui.pod_seams.handle_event_with(other, cursor);
                    pod_ratio_action = pod_action;

                    let icon = renderer
                        .ui
                        .page_seams
                        .cursor_icon()
                        .or_else(|| renderer.ui.pod_seams.cursor_icon());
                    if let Some(icon) = icon {
                        window.set_cursor(icon);
                    }
                }

                let events = renderer.ui.input.route(other, &mut renderer.ui.tree);
                for ev in events {
                    match ev {
                        UiEvent::TriggerFired(id) => {
                            if let Some(sig) = context_menu_triggers.get(&id).copied() {
                                page_signal = Some(sig);
                                dismiss_context_menu = true;
                            } else if let Some(sig) = tab_triggers.get(&id).copied() {
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
                            } else if let Some((page_id, pod_leaf_id)) =
                                icon_rail_triggers.get(&id).copied()
                            {
                                page_signal =
                                    Some(crate::workspace::analysis::PageSignal::ScrollToPod {
                                        page_id,
                                        pod_leaf_id,
                                    });
                            } else if let Some(page_id) = page_split_triggers.get(&id).copied() {
                                let direction =
                                    split_direction_for_page(shell, pages_area, page_id);
                                page_signal =
                                    Some(crate::workspace::analysis::PageSignal::SplitPage {
                                        page_id,
                                        direction,
                                    });
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

            // Apply page seam ratio mutations, then rebuild seam draw lists.
            if let Some((idx, act)) = page_ratio_action {
                if let Some(ws) = shell.active_mut().and_then(|a| a.analysis_mut()) {
                    match act {
                        SeamRatioAction::Set(r) => ws.page_tree.set_ratio_index(idx, r),
                        SeamRatioAction::Reset => ws.page_tree.reset_ratio_index(idx),
                    }
                }
                if let Some(mut renderer) = shell.renderer.take() {
                    if let Some(ws) = shell.active().and_then(|a| a.analysis()) {
                        rebuild_seams(ws, pages_area, &mut renderer);
                        renderer.ui.page_seams.mark_dragging(idx);
                    }
                    shell.renderer = Some(renderer);
                    size_class_rebuild = true;
                }
            }

            if let Some((idx, act)) = pod_ratio_action {
                let owner = shell
                    .renderer
                    .as_ref()
                    .and_then(|r| r.ui.pod_seams.pod_owners.get(idx).copied());
                if let Some((page_id, local)) = owner {
                    if let Some(ws) = shell.active_mut().and_then(|a| a.analysis_mut()) {
                        if let Some(page) = ws.page_tree.find_mut(page_id) {
                            match act {
                                SeamRatioAction::Set(r) => page.pod_tree.set_ratio(local, r),
                                SeamRatioAction::Reset => page.pod_tree.reset_ratio(local),
                            }
                        }
                    }
                }
                if let Some(mut renderer) = shell.renderer.take() {
                    if let Some(ws) = shell.active().and_then(|a| a.analysis()) {
                        rebuild_seams(ws, pages_area, &mut renderer);
                        if matches!(act, SeamRatioAction::Set(_)) {
                            renderer.ui.pod_seams.mark_dragging(idx);
                        }
                    }
                    shell.renderer = Some(renderer);
                    size_class_rebuild = true;
                }
            }

            if let Some(menu) = open_context_menu {
                shell.pending_context_menu = Some(menu);
                dismiss_context_menu = false;
                // Rebuild so menu triggers exist in the particle tree.
                if let Some(mut renderer) = shell.renderer.take() {
                    rebuild_active(shell, &mut renderer);
                    shell.renderer = Some(renderer);
                }
            }
            if dismiss_context_menu && page_signal.is_none() {
                shell.pending_context_menu = None;
                shell.context_menu_triggers.clear();
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
            if let Some(sig) = page_signal {
                handle_page_signal(shell, sig);
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

fn position_context_menu(root: &mut hyper_ui::Particle, rect: Rect) {
    let hyper_ui::Particle::Surface(surface) = root else {
        return;
    };
    let Some(hyper_ui::Particle::Stack(column)) = surface.child.as_deref_mut() else {
        return;
    };
    // Context menu is the last column child when pending.
    if let Some(menu) = column.children.last_mut() {
        menu.set_layout(LayoutBox {
            origin: rect.origin,
            size: rect.size,
        });
        arrange_particle(menu, rect);
    }
}

fn page_side_under_cursor(
    pages_area: Rect,
    cursor: Vec2,
    direction: SeamDirection,
) -> PageSide {
    match direction {
        SeamDirection::Vertical => {
            if cursor.x < pages_area.origin.x + pages_area.size.x * 0.5 {
                PageSide::First
            } else {
                PageSide::Second
            }
        }
        SeamDirection::Horizontal => {
            if cursor.y < pages_area.origin.y + pages_area.size.y * 0.5 {
                PageSide::First
            } else {
                PageSide::Second
            }
        }
    }
}

fn split_direction_for_page(
    shell: &AppShell,
    pages_area: Rect,
    page_id: hyper_ui::PageId,
) -> SeamDirection {
    let Some(ws) = shell.active().and_then(|a| a.analysis()) else {
        return SeamDirection::Vertical;
    };
    let rect = ws
        .page_tree
        .leaf_rects(pages_area)
        .into_iter()
        .find(|(id, _)| *id == page_id)
        .map(|(_, r)| r);
    match rect {
        Some(r) if r.size.x >= r.size.y => SeamDirection::Vertical,
        _ => SeamDirection::Horizontal,
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
        .and_then(|a| a.analysis())
        .and_then(|ws| ws.io_rect(pages_area, IoKind::InputForm))
        .map(|r| r.size.x);
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
