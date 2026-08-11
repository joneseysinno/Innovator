use super::handle_analysis_action::{handle_analysis_action, handle_promote_prop};
use super::handle_app_signal::handle_app_signal;
use super::handle_builder_field::handle_builder_field;
use super::handle_page_signal::handle_page_signal;
use super::handle_value_changed::handle_value_changed;
use super::handle_workspace_signal::handle_workspace_signal;
use super::page_context_menu::PageContextMenu;
use super::rebuild_active::rebuild_active;
use super::rebuild_seams::rebuild_seams;
use super::sync_chrome_layouts::sync_chrome_layouts;
use super::update_focus::update_focus_from_pointer;
use super::AppShell;
use crate::domains::structural::IoKind;
use crate::pages::analysis::input_form::FormDensity;
use crate::workspace::signal::WorkspaceSignal;
use hyper_ui::layout::{arrange_particle, LayoutBox};
use hyper_ui::{
    apply_signal_text, sync_from_page_tree, PageSide, PointerKind, Rect, SeamDirection,
    SeamRatioAction, UVec2, UiEvent, Vec2,
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
            let scale = window.scale_factor() as f32;
            shell.set_physical_size(size.width, size.height, scale);
            shell.has_header = shell.active().and_then(|a| a.header()).is_some();

            let mut renderer = match shell.renderer.take() {
                Some(r) => r,
                None => return,
            };
            renderer.resize(*size, scale);
            rebuild_active(shell, &mut renderer);

            // Render immediately — Windows modal resize loop may delay RedrawRequested.
            render_frame(shell, &mut renderer);
            shell.renderer = Some(renderer);
            window.request_redraw();
        }
        WindowEvent::ScaleFactorChanged { scale_factor, .. } => {
            let scale = *scale_factor as f32;
            let physical = window.inner_size();
            shell.set_physical_size(physical.width, physical.height, scale);
            if let Some(renderer) = shell.renderer.as_mut() {
                renderer.resize(physical, scale);
            }
            window.request_redraw();
        }
        WindowEvent::Touch(_) => {
            shell.promote_input_to_hybrid();
        }
        WindowEvent::KeyboardInput { event, .. } => {
            if event.state == winit::event::ElementState::Pressed && !event.repeat {
                match event.physical_key {
                    winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::F9) => {
                        shell.cycle_preview();
                        let mut renderer = match shell.renderer.take() {
                            Some(r) => r,
                            None => return,
                        };
                        rebuild_active(shell, &mut renderer);
                        shell.renderer = Some(renderer);
                        window.request_redraw();
                    }
                    winit::keyboard::PhysicalKey::Code(winit::keyboard::KeyCode::F10) => {
                        shell.overlay_open = !shell.overlay_open;
                        let mut renderer = match shell.renderer.take() {
                            Some(r) => r,
                            None => return,
                        };
                        rebuild_active(shell, &mut renderer);
                        shell.renderer = Some(renderer);
                        window.request_redraw();
                    }
                    _ => {}
                }
            }
        }
        WindowEvent::RedrawRequested => {
            let mut renderer = match shell.renderer.take() {
                Some(r) => r,
                None => return,
            };
            render_frame(shell, &mut renderer);
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
                .and_then(|a| a.home_ws())
                .map(|ws| ws.actions.clone())
                .unwrap_or_default();
            let analysis = shell.active().and_then(|a| a.structural());
            let placeholder = shell.active().and_then(|a| a.placeholder());
            let wall_sinks = analysis.map(|ws| ws.wall_sinks.clone()).unwrap_or_default();
            let nav_triggers = analysis.map(|ws| ws.nav_triggers.clone()).unwrap_or_default();
            let results_triggers = analysis
                .map(|ws| ws.results_triggers.clone())
                .unwrap_or_default();
            let analysis_actions = analysis
                .map(|ws| ws.analysis_actions.clone())
                .unwrap_or_default();
            let promote_props = analysis
                .map(|ws| ws.promote_props.clone())
                .unwrap_or_default();
            let field_props = analysis.map(|ws| ws.field_props.clone()).unwrap_or_default();
            let builder_slots = analysis
                .map(|ws| ws.builder_slots.clone())
                .unwrap_or_default();
            let wall_view_sink = analysis.and_then(|ws| ws.wall_view_sink);
            let icon_rail_triggers = analysis
                .map(|ws| ws.icon_rail_triggers.clone())
                .or_else(|| placeholder.map(|ws| ws.icon_rail_triggers.clone()))
                .unwrap_or_default();
            let pod_collapse_triggers = analysis
                .map(|ws| ws.pod_collapse_triggers.clone())
                .or_else(|| placeholder.map(|ws| ws.pod_collapse_triggers.clone()))
                .unwrap_or_default();
            let page_split_triggers = analysis
                .map(|ws| ws.page_split_triggers.clone())
                .unwrap_or_default();
            let page_show_triggers = analysis
                .map(|ws| ws.page_show_triggers.clone())
                .or_else(|| placeholder.map(|ws| ws.page_show_triggers.clone()))
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
            let mut pod_divider_events: Vec<UiEvent> = Vec::new();

            if let WindowEvent::MouseWheel { delta, .. } = other {
                let cursor = shell
                    .renderer
                    .as_ref()
                    .map(|r| r.ui.input.cursor)
                    .unwrap_or(Vec2::ZERO);
                let over_view = shell
                    .active()
                    .and_then(|a| a.structural())
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
                let cursor = shell
                    .renderer
                    .as_ref()
                    .map(|r| r.ui.input.cursor)
                    .unwrap_or(Vec2::ZERO);
                if update_focus_from_pointer(shell, cursor) {
                    size_class_rebuild = true;
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
                    pod_divider_events =
                        renderer.ui.pod_dividers.handle_event(other, cursor);

                    let icon = renderer
                        .ui
                        .page_seams
                        .cursor_icon()
                        .or_else(|| renderer.ui.pod_dividers.cursor_icon());
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
                            } else if let Some((page_id, pod_id)) =
                                icon_rail_triggers.get(&id).copied()
                            {
                                page_signal =
                                    Some(crate::domains::structural::PageSignal::ScrollToPod {
                                        page_id,
                                        pod_id,
                                    });
                            } else if let Some(pod_id) = pod_collapse_triggers.get(&id).copied() {
                                // Handled below via synthetic divider-style event.
                                pod_divider_events.push(UiEvent::PodCollapse { id: pod_id });
                            } else if let Some(page_id) = page_show_triggers.get(&id).copied() {
                                let ws_id = shell.active().map(|a| a.state.id);
                                if let Some(ws) = shell.active_mut().and_then(|a| a.structural_mut())
                                {
                                    ws.focused_page = page_id;
                                    size_class_rebuild = true;
                                } else if let Some(ws) =
                                    shell.active_mut().and_then(|a| a.placeholder_mut())
                                {
                                    ws.focused_page = page_id;
                                    size_class_rebuild = true;
                                }
                                if let Some(ws_id) = ws_id {
                                    shell.focus = hyper_ui::FocusPath::new(vec![
                                        ws_id,
                                        hyper_ui::PageNode::container_id(page_id),
                                    ]);
                                }
                            } else if let Some(page_id) = page_split_triggers.get(&id).copied() {
                                let direction =
                                    split_direction_for_page(shell, pages_area, page_id);
                                page_signal =
                                    Some(crate::domains::structural::PageSignal::SplitPage {
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

            // Apply page seam override mutations, then rebuild seam draw lists.
            if let Some((idx, act)) = page_ratio_action {
                let focus = shell.focus.clone();
                let viewport = shell.resolve_viewport();
                let mut layout_changed = false;
                if let Some(ws) = shell
                    .active_mut()
                    .and_then(|a| a.structural_mut())
                {
                    match act {
                        SeamRatioAction::Set(r) => {
                            ws.apply_page_seam_drag(idx, r, pages_area);
                            let (_, report) = ws.layout_pages(pages_area, &focus, &viewport);
                            shell.last_report = report;
                            layout_changed = true;
                        }
                        SeamRatioAction::Reset => {
                            ws.reset_page_seam(idx, pages_area);
                            let (_, report) = ws.layout_pages(pages_area, &focus, &viewport);
                            shell.last_report = report;
                            layout_changed = true;
                        }
                    }
                }
                if layout_changed {
                    shell.persist_layout();
                }
                if let Some(mut renderer) = shell.renderer.take() {
                    if let Some(tree) = shell.active().and_then(|a| a.page_tree()) {
                        rebuild_seams(tree, pages_area, &mut renderer);
                        renderer.ui.page_seams.mark_dragging(idx);
                    }
                    shell.renderer = Some(renderer);
                    size_class_rebuild = true;
                }
            }

            if !pod_divider_events.is_empty() {
                let mut changed = false;
                if let Some(ws) = shell
                    .active_mut()
                    .and_then(|a| a.structural_mut())
                {
                    for ev in &pod_divider_events {
                        match ev {
                            UiEvent::PodCollapse { id } => {
                                if toggle_pod_collapse(&mut ws.page_tree, pages_area, *id) {
                                    changed = true;
                                }
                            }
                            UiEvent::PodDividerDrag { above, delta } => {
                                let owner = ws
                                    .page_tree
                                    .leaf_rects(pages_area)
                                    .into_iter()
                                    .find_map(|(page_id, page_rect)| {
                                        let page = ws.page_tree.find(page_id)?;
                                        if page.pods.pods.iter().any(|p| p.id == *above) {
                                            let content = page.content_rect(page_rect);
                                            Some((page_id, content.size.y, content.size.x))
                                        } else {
                                            None
                                        }
                                    });
                                if let Some((page_id, area_h, area_w)) = owner {
                                    if let Some(page) = ws.page_tree.find_mut(page_id) {
                                        let class = hyper_ui::SizeClass::from_width(area_w);
                                        page.pods.apply_divider_drag(
                                            *above, *delta, area_h, class,
                                        );
                                        changed = true;
                                    }
                                }
                            }
                            UiEvent::PodDividerEqualize { above } => {
                                let owner = ws
                                    .page_tree
                                    .leaf_rects(pages_area)
                                    .into_iter()
                                    .find_map(|(page_id, page_rect)| {
                                        let page = ws.page_tree.find(page_id)?;
                                        if page.pods.pods.iter().any(|p| p.id == *above) {
                                            Some((page_id, page.content_rect(page_rect).size.x))
                                        } else {
                                            None
                                        }
                                    });
                                if let Some((page_id, area_w)) = owner {
                                    if let Some(page) = ws.page_tree.find_mut(page_id) {
                                        page.pods
                                            .equalize(hyper_ui::SizeClass::from_width(area_w));
                                        changed = true;
                                    }
                                }
                            }
                            _ => {}
                        }
                    }
                } else if let Some(ws) = shell.active_mut().and_then(|a| a.placeholder_mut()) {
                    for ev in &pod_divider_events {
                        if let UiEvent::PodCollapse { id } = ev {
                            if toggle_pod_collapse(&mut ws.page_tree, pages_area, *id) {
                                changed = true;
                            }
                        }
                    }
                }
                if changed {
                    shell.persist_layout();
                    if let Some(mut renderer) = shell.renderer.take() {
                        if let Some(tree) = shell.active().and_then(|a| a.page_tree()) {
                            rebuild_seams(tree, pages_area, &mut renderer);
                        }
                        shell.renderer = Some(renderer);
                        size_class_rebuild = true;
                    }
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

fn toggle_pod_collapse(
    tree: &mut hyper_ui::PageTree,
    pages_area: Rect,
    pod_id: hyper_ui::PodId,
) -> bool {
    let owner = tree.leaf_rects(pages_area).into_iter().find_map(|(page_id, page_rect)| {
        let page = tree.find(page_id)?;
        if page.pods.pods.iter().any(|p| p.id == pod_id) {
            Some((page_id, page.content_rect(page_rect)))
        } else {
            None
        }
    });
    let Some((page_id, content_rect)) = owner else {
        return false;
    };
    let Some(page) = tree.find_mut(page_id) else {
        return false;
    };
    let rects = page.pods.layout_rects(content_rect);
    let content_y =
        hyper_ui::PodList::content_y_of(&rects, pod_id, content_rect).unwrap_or(0.0);
    let screen_y = content_y - page.pods.scroll_offset;
    page.pods.toggle(pod_id);
    page.pods
        .anchor_scroll_on_toggle(pod_id, content_rect, screen_y);
    true
}

fn render_frame(shell: &mut AppShell, renderer: &mut hyper_ui::HyperRenderer) {
    let status_id = shell.active().and_then(|a| a.status_id());
    let pages_area = shell.pages_area;
    let layout_area = shell.layout_area();
    let has_header = shell.has_header;
    let pending_menu = shell.pending_context_menu.clone();

    let spatial = shell
        .active()
        .and_then(|a| a.structural())
        .map(|ws| ws.wall_spatial.clone())
        .unwrap_or_default();

    let wall_view_rect = shell
        .active()
        .and_then(|a| a.structural())
        .and_then(|ws| ws.io_rect(pages_area, IoKind::WallView));

    while let Ok(msg) = shell.signal_rx.try_recv() {
        if let Some(id) = status_id {
            apply_signal_text(&mut renderer.ui.tree, id, msg);
        }
    }

    renderer.ui.layout(layout_area);
    if let Some(root) = renderer.ui.tree.root.as_mut() {
        sync_chrome_layouts(root, layout_area, has_header);
    }
    if let Some(tree) = shell.active_mut().and_then(|a| a.page_tree_mut()) {
        if let Some(root) = renderer.ui.tree.root.as_mut() {
            sync_from_page_tree(root, tree, pages_area);
        }
    }
    if let Some(root) = renderer.ui.tree.root.as_mut() {
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
    // UI layouts are logical; screen_size must match or HiDPI leaves empty right/bottom.
    let screen = [layout_area.size.x.max(1.0), layout_area.size.y.max(1.0)];
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
    let Some(ws) = shell
        .active()
        .and_then(|a| a.structural())
    else {
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
    let mut pan_delta = None;
    let mut zoom = None;
    {
        let Some(ws) = shell.active_mut().and_then(|a| a.structural_mut()) else {
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
        .and_then(|a| a.structural())
        .and_then(|ws| ws.io_rect(pages_area, IoKind::InputForm))
        .map(|r| r.size.x);
    let Some(width) = width else {
        return;
    };
    let new_class = FormDensity::from_width(width);
    let changed = match shell
        .active_mut()
        .and_then(|a| a.structural_mut())
    {
        Some(ws) if ws.input_size_class != new_class => {
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
