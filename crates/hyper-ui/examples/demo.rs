//! Kitchen-sink window — exercises all Phase 0 particles and acceptance criteria.
//!
//! ```sh
//! cargo run -p hyper-ui --example demo
//! ```

use hyper_ui::particles::{Particle, SourceParticle, StackParticle, SurfaceParticle, TriggerParticle};
use hyper_ui::seam::PodTree;
use hyper_ui::{
    apply_signal_text, engineer_input, EdgeKindGpu, HyperRenderer, InMemoryWorldSpatial, Rect,
    SceneNode, UiEvent, Vec2, WorldEdge,
};
use std::sync::Arc;
use std::time::{Duration, Instant};
use winit::application::ApplicationHandler;
use winit::event::WindowEvent;
use winit::event_loop::{ActiveEventLoop, ControlFlow, EventLoop};
use winit::window::{Window, WindowId};

fn build_ui_tree() -> (Particle, hyper_ui::ParticleId, hyper_ui::ParticleId) {
    let title = SourceParticle::new("hyper-ui demo").with_weight(500);
    let status = SourceParticle::secondary("signal: waiting…");
    let status_id = status.id;

    let height = engineer_input("Height", 12.0, "ft");
    let field_id = height.field_id;
    let thickness = engineer_input("Thickness", 8.0, "in");
    let run = TriggerParticle::primary("Run Analysis");

    let form = StackParticle::column(vec![
        Particle::Source(title),
        Particle::Source(status),
        height.into_particle(),
        thickness.into_particle(),
        Particle::Trigger(run),
    ])
    .with_gap(12.0);

    let root = Particle::Surface(
        SurfaceParticle::new([0.13, 0.14, 0.17, 1.0])
            .with_padding(16.0)
            .with_radius(0.0)
            .with_child(Particle::Stack(form)),
    );

    (root, status_id, field_id)
}

struct DemoApp {
    window: Option<Arc<Window>>,
    renderer: Option<HyperRenderer>,
    status_id: Option<hyper_ui::ParticleId>,
    signal_rx: flume::Receiver<String>,
    signal_tx: flume::Sender<String>,
    spatial: InMemoryWorldSpatial,
    pods: PodTree,
    pod_area: Rect,
    last_frame: Instant,
}

impl DemoApp {
    fn new() -> Self {
        let (signal_tx, signal_rx) = flume::unbounded();
        // Background thread fires a Signal that updates a source particle.
        let tx = signal_tx.clone();
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_secs(2));
            let _ = tx.send("signal: engine tick @ t+2s".into());
            loop {
                std::thread::sleep(Duration::from_secs(3));
                let _ = tx.send(format!(
                    "signal: live update {}",
                    chrono_like_now()
                ));
            }
        });

        let spatial = InMemoryWorldSpatial {
            nodes: vec![
                SceneNode {
                    world_pos: [0.0, 0.0],
                    size_world: [2.0, 1.2],
                    color: [0.25, 0.45, 0.75, 1.0],
                    border_color: [0.9, 0.9, 0.95, 1.0],
                    border_radius: 0.15,
                    border_width: 2.0,
                    selected: false,
                },
                SceneNode {
                    world_pos: [3.5, 0.5],
                    size_world: [1.6, 1.6],
                    color: [0.55, 0.35, 0.20, 1.0],
                    border_color: [0.2, 0.2, 0.2, 1.0],
                    border_radius: 0.1,
                    border_width: 1.0,
                    selected: true,
                },
            ],
            edges: vec![WorldEdge {
                source: [0.0, 0.0],
                target: [3.5, 0.5],
                curvature: 0.8,
                color: [0.40, 0.75, 0.95, 1.0],
                width: 2.5,
                kind: EdgeKindGpu::Stream,
            }],
        };

        Self {
            window: None,
            renderer: None,
            status_id: None,
            signal_rx,
            signal_tx,
            spatial,
            pods: PodTree::two_column(0.42),
            pod_area: Rect::from_xywh(0.0, 0.0, 800.0, 600.0),
            last_frame: Instant::now(),
        }
    }
}

fn chrono_like_now() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs() % 10000)
        .unwrap_or(0);
    format!("{secs}")
}

impl ApplicationHandler for DemoApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        if self.window.is_some() {
            return;
        }
        let attrs = Window::default_attributes()
            .with_title("hyper-ui demo")
            .with_inner_size(winit::dpi::LogicalSize::new(1100.0, 700.0));
        let window = Arc::new(event_loop.create_window(attrs).unwrap());
        let mut renderer = HyperRenderer::new(window.clone());

        let (root, status_id, _field_id) = build_ui_tree();
        self.status_id = Some(status_id);
        renderer.ui.set_tree(root);

        let size = window.inner_size();
        self.pod_area = Rect::from_xywh(0.0, 0.0, size.width as f32, size.height as f32);
        renderer
            .ui
            .pod_seams
            .rebuild_from_pods(&self.pods, self.pod_area);

        // Left pod = UI form, right pod = scene canvas region (drawn via Layer A)
        let leaves = self.pods.leaf_rects(self.pod_area);
        if let Some((_, left)) = leaves.first() {
            renderer.ui.layout(*left);
        }

        window.request_redraw();
        self.window = Some(window);
        self.renderer = Some(renderer);
        let _ = &self.signal_tx;
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        let Some(window) = self.window.clone() else {
            return;
        };
        let Some(renderer) = self.renderer.as_mut() else {
            return;
        };

        match &event {
            WindowEvent::CloseRequested => event_loop.exit(),
            WindowEvent::Resized(size) => {
                renderer.resize(*size);
                self.pod_area =
                    Rect::from_xywh(0.0, 0.0, size.width as f32, size.height as f32);
                renderer
                    .ui
                    .pod_seams
                    .rebuild_from_pods(&self.pods, self.pod_area);
                if let Some((_, left)) = self.pods.leaf_rects(self.pod_area).first().copied() {
                    renderer.ui.layout(left);
                }
                window.request_redraw();
            }
            WindowEvent::RedrawRequested => {
                // Drain background signals — partial text dirty only.
                while let Ok(msg) = self.signal_rx.try_recv() {
                    if let Some(id) = self.status_id {
                        apply_signal_text(&mut renderer.ui.tree, id, msg);
                    }
                }

                let leaves = self.pods.leaf_rects(self.pod_area);
                if let Some((_, left)) = leaves.first().copied() {
                    if renderer.ui.tree.dirty.needs_layout() {
                        renderer.ui.layout(left);
                    }
                }

                // Scene in the right pod
                if let Some((_, right)) = leaves.get(1).copied() {
                    renderer.scene.camera.screen_px = hyper_ui::UVec2::new(
                        right.size.x.max(1.0) as u32,
                        right.size.y.max(1.0) as u32,
                    );
                    // Offset camera draw by translating after — for demo, fill whole window
                    // scene layer still draws full-surface; nodes demonstrate Layer A.
                    let _ = right;
                }

                renderer
                    .scene
                    .cull_and_upload(&renderer.device, &renderer.queue, &self.spatial);

                let focused = renderer.ui.input.focused;
                let screen = [
                    renderer.config.width as f32,
                    renderer.config.height as f32,
                ];
                // Re-layout left pod every frame so resize/seam stay correct
                if let Some((_, left)) = leaves.first().copied() {
                    renderer.ui.layout(left);
                }
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

                // Aim ~60fps
                let elapsed = self.last_frame.elapsed();
                if elapsed < Duration::from_millis(16) {
                    std::thread::sleep(Duration::from_millis(16) - elapsed);
                }
                self.last_frame = Instant::now();
                window.request_redraw();
            }
            other => {
                let cursor = renderer.ui.input.cursor;
                let seam_events = renderer.ui.pod_seams.handle_event(
                    other,
                    cursor,
                    &mut self.pods,
                    self.pod_area,
                );
                for ev in &seam_events {
                    if matches!(ev, UiEvent::SeamDrag { .. } | UiEvent::SeamReset { .. }) {
                        renderer.ui.tree.mark_all_dirty();
                        if let Some((_, left)) = self.pods.leaf_rects(self.pod_area).first().copied()
                        {
                            renderer.ui.layout(left);
                        }
                    }
                }
                if let Some(icon) = renderer.ui.pod_seams.cursor_icon() {
                    window.set_cursor(icon);
                }

                // Pan/zoom scene with sink-like pointer on right half
                if let WindowEvent::MouseWheel { delta, .. } = other {
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
                    let anchor = renderer.ui.input.cursor;
                    renderer.scene.camera.zoom_at(anchor, factor);
                }

                let events = renderer.ui.input.route(other, &mut renderer.ui.tree);
                for ev in events {
                    match ev {
                        UiEvent::TriggerFired(_) => {
                            if let Some(id) = self.status_id {
                                apply_signal_text(
                                    &mut renderer.ui.tree,
                                    id,
                                    "signal: Run Analysis fired".into(),
                                );
                            }
                            let _ = self.signal_tx.send("signal: run ack".into());
                        }
                        UiEvent::FieldCommit { value, .. } => {
                            if let Some(id) = self.status_id {
                                apply_signal_text(
                                    &mut renderer.ui.tree,
                                    id,
                                    format!("signal: committed {}", value.display()),
                                );
                            }
                        }
                        UiEvent::SinkPointer { pos, kind, .. } => {
                            if matches!(kind, hyper_ui::particles::PointerKind::Move) {
                                let _ = pos;
                            }
                        }
                        _ => {}
                    }
                }

                // Middle-drag pan when cursor is over the right pod
                if let WindowEvent::CursorMoved { position, .. } = other {
                    let pos = Vec2::new(position.x as f32, position.y as f32);
                    if let Some((_, right)) = self.pods.leaf_rects(self.pod_area).get(1) {
                        if right.contains(pos) {
                            // small auto-idle pan unused — wheel zoom is enough for demo
                        }
                    }
                }

                window.request_redraw();
            }
        }
    }
}

fn main() {
    let _ = env_logger::try_init();
    let event_loop = EventLoop::new().unwrap();
    event_loop.set_control_flow(ControlFlow::Poll);
    let mut app = DemoApp::new();
    event_loop.run_app(&mut app).unwrap();
}
