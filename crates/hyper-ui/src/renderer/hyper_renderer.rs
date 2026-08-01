mod begin_frame;
mod end_frame;
mod new;
mod new_async;
mod resize;
mod set_clear_color;

use crate::text::TextRenderer;
use crate::ui::UiRenderer;
use std::sync::Arc;
use winit::window::Window;

use super::SceneRenderer;

/// Top-level renderer: Layer A scene + Layer B UI + text, sharing one device/queue/surface.
pub struct HyperRenderer {
    pub instance: wgpu::Instance,
    pub device: wgpu::Device,
    pub queue: wgpu::Queue,
    pub surface: wgpu::Surface<'static>,
    pub config: wgpu::SurfaceConfiguration,
    pub scene: SceneRenderer,
    pub ui: UiRenderer,
    pub text: TextRenderer,
    pub window: Arc<Window>,
    clear_color: wgpu::Color,
}
