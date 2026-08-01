use super::layout_areas::layout_areas;
use super::rebuild_active::rebuild_active;
use super::AppShell;
use crate::workspace::screen_class::ScreenClass;
use hyper_ui::{HyperRenderer, Rect};
use std::sync::Arc;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;

pub(crate) fn resumed(shell: &mut AppShell, event_loop: &ActiveEventLoop) {
    if shell.window.is_some() {
        return;
    }
    let attrs = Window::default_attributes()
        .with_title("Innovator")
        .with_inner_size(winit::dpi::LogicalSize::new(1280.0, 800.0));
    let window = Arc::new(event_loop.create_window(attrs).unwrap());
    let mut renderer = HyperRenderer::new(window.clone());

    let size = window.inner_size();
    shell.screen_class = ScreenClass::from_width(size.width);
    shell.window_area = Rect::from_xywh(0.0, 0.0, size.width as f32, size.height as f32);
    shell.has_header = shell.active().and_then(|a| a.header()).is_some();
    let (_tabs, _header, pages) = layout_areas(shell.window_area, shell.has_header);
    shell.pages_area = pages;

    rebuild_active(shell, &mut renderer);

    window.request_redraw();
    shell.window = Some(window);
    shell.renderer = Some(renderer);
}
