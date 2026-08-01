use crate::geom::UVec2;
use crate::text::TextRenderer;
use crate::ui::UiRenderer;
use std::sync::Arc;
use winit::window::Window;

use super::super::SceneRenderer;
use super::HyperRenderer;

impl HyperRenderer {
    pub(super) async fn new_async(window: Arc<Window>) -> Self {
        let size = window.inner_size();
        let width = size.width.max(1);
        let height = size.height.max(1);

        let mut instance_desc = wgpu::InstanceDescriptor::new_without_display_handle();
        instance_desc.backends = wgpu::Backends::PRIMARY;
        let instance = wgpu::Instance::new(instance_desc);

        let surface = instance
            .create_surface(window.clone())
            .expect("failed to create wgpu surface");

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
                apply_limit_buckets: true,
            })
            .await
            .expect("no suitable GPU adapter");

        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default())
            .await
            .expect("failed to create device");

        let caps = surface.get_capabilities(&adapter);
        let format = caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format,
            color_space: wgpu::SurfaceColorSpace::Auto,
            width,
            height,
            present_mode: wgpu::PresentMode::AutoVsync,
            alpha_mode: caps.alpha_modes[0],
            view_formats: vec![],
            desired_maximum_frame_latency: 2,
        };
        surface.configure(&device, &config);

        let screen = UVec2::new(width, height);
        let scene = SceneRenderer::new(&device, format, screen);
        let ui = UiRenderer::new(&device, format);
        let text = TextRenderer::new(&device, &queue, format);

        Self {
            instance,
            device,
            queue,
            surface,
            config,
            scene,
            ui,
            text,
            window,
            clear_color: wgpu::Color {
                r: 0.10,
                g: 0.11,
                b: 0.13,
                a: 1.0,
            },
        }
    }
}
