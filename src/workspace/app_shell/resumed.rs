use super::layout_areas::layout_areas;
use super::rebuild_active::rebuild_active;
use super::AppShell;
use hyper_ui::HyperRenderer;
use std::sync::Arc;
use winit::dpi::LogicalSize;
use winit::event_loop::ActiveEventLoop;
use winit::window::Window;

pub(crate) fn resumed(shell: &mut AppShell, event_loop: &ActiveEventLoop) {
    if shell.window.is_some() {
        return;
    }
    let attrs = Window::default_attributes()
        .with_title("Innovator")
        .with_inner_size(LogicalSize::new(1280.0, 800.0))
        .with_min_inner_size(LogicalSize::new(320.0, 480.0));
    let window = Arc::new(event_loop.create_window(attrs).unwrap());
    let mut renderer = HyperRenderer::new(window.clone());

    let scale = window.scale_factor() as f32;
    let physical = window.inner_size();
    shell.set_physical_size(physical.width, physical.height, scale);

    shell.has_header = shell.active().and_then(|a| a.header()).is_some();
    let layout = shell.layout_area();
    let (_tabs, _header, pages) = layout_areas(layout, shell.has_header);
    shell.pages_area = pages;

    rebuild_active(shell, &mut renderer);

    window.request_redraw();
    shell.window = Some(window);
    shell.renderer = Some(renderer);
}
