# Concrete wall analysis sheet — build plan

## What we're building

A special concrete wall analysis sheet inside the spatial hypergraph UI.
The workspace shows **all three pages simultaneously** — Navigation, Analysis,
and Results are tiled on screen at the same time, like Blender's areas.
No switching. You see your wall list, your inputs, and your results all at once.

The entire UI — every surface, field, trigger, and view — is rendered by a
purpose-built wgpu UI library. That library is the foundation everything else
stands on and must be built first.

---

## Vocabulary recap

| Level | Owns | Behavior |
|---|---|---|
| **Workspace** | OS window, top bar, screen-class logic | Multi-window, responsive breakpoints |
| **Page** | A named spatial region, split/merged like Blender editors | Spatial subdivision, seam dragging |
| **Pod** | A leaf region inside a page | Adaptive sizing, size-class rules |
| **IO** | The thing that fills a pod and does something | Assembled from particles |

**Particles:** `source`, `field`, `trigger`, `signal`, `view`, `sink`, `slot`, `stack`, `surface`

---

## Workspace layout — concrete wall analysis

```
┌─────────────────────────────────────────────────────────────┐
│  top bar  [New Wall]  [Save]  [Run Analysis]  [Export]      │  ← workspace
├──────────────┬──────────────────────────┬────────────────────┤
│              │                          │                    │
│  NAVIGATION  │       ANALYSIS           │     RESULTS        │
│     page     │         page             │      page          │
│              │                          │                    │
│  wall list   │  inputs  │  wall view    │  summary table     │
│  pod         │  pod     │  pod          │  pod               │
│              │          │               │                    │
│              │          │               │  status pod        │
│              │          │               │                    │
└──────────────┴──────────┴───────────────┴────────────────────┘
```

All three pages tile the workspace simultaneously. Seams between pages
are draggable. Each page can be independently split into pods.

---

## Phase 0 — wgpu hypergraph UI library  ← build this first

### Purpose

Every IO, every particle, every pod border, every seam handle is drawn
by this library. It is a standalone Rust crate — `hyper-ui` — that sits
between wgpu and the application layer. Nothing in Phases 1–7 can exist
without it.

The library has two layers that must stay architecturally separate:

- **Layer A — hypergraph scene renderer:** draws nodes as positioned geometry,
  hyperedges as directed curves, supports spatial navigation (pan, zoom).
  This is the "infinite canvas" that the spatial hypergraph lives on.

- **Layer B — UI particle renderer:** draws the widget tree — surfaces, stacks,
  fields, triggers, sources. This is a retained-mode UI system whose output
  is wgpu draw calls. It sits on top of Layer A and can be embedded inside
  any node in the scene.

They share the same wgpu `Device`, `Queue`, and swap chain. Layer B widgets
are themselves nodes in Layer A's scene — a `surface` particle is a rect
in screen space that is also a `HyperNode` in the graph.

### Workspace conventions

**Edition:** Rust 2024 throughout. Every `Cargo.toml` declares:
```toml
[package]
edition = "2024"
```

**Crates are publishable, standalone libraries.** Every crate in `crates/`
is fully independent — no knowledge of Innovator, no knowledge of other
crates in this workspace unless there is a genuine architectural dependency.
`hyper-ui` can be used by any wgpu app. `hypernode` can be used by any
graph application. Each can be published to crates.io as-is. The test:
could someone `cargo add hyper-ui` and build a completely different app?
If yes, the boundary is right.

**`infinite-db` comes from crates.io — not a local path.** It is already
published. It is declared as a version dependency like any external crate.
It does not live in `crates/`.

**`Innovator/src/` holds all app-specific code as modules.** Everything
too specific to Innovator to ever be its own publishable crate lives here —
the concrete wall workspace, pages, IO types, analysis engine, composite
components like `engineer_input`. These are Rust 2024 sibling-file modules
declared from `lib.rs`.

**Module files:** no `mod.rs` files anywhere. A module `workspace` with
submodules is expressed as:
```
src/
├── workspace.rs        ← declares submodules with `pub mod pages;` etc.
└── workspace/
    ├── pages.rs
    └── shell.rs
```
Never `src/workspace/mod.rs`.

**Module vs crate boundary rule:**
- Module: app-specific, single consumer, not independently useful
- Crate: general-purpose, publishable, works without Innovator context

### Workspace layout

```
Innovator/
├── Cargo.toml                  ← workspace root AND app package
├── src/
│   ├── main.rs                 ← entry point · wires crates together · stays thin
│   ├── workspace.rs            ← WorkspaceShell, ScreenClass, active_wall
│   ├── workspace/
│   │   └── top_bar.rs
│   ├── pages.rs                ← page module declarations
│   ├── pages/
│   │   ├── navigation.rs       ← WallListIO, WallSummaryIO
│   │   ├── analysis.rs         ← InputFormIO, WallViewIO, FieldBuilderIO
│   │   └── results.rs          ← ResultsTableIO, StatusIO
│   ├── engine.rs               ← ACI 318 analysis logic (Function HyperNode)
│   └── components.rs           ← engineer_input and other composites
│
└── crates/
    ├── hyper-ui/               ← wgpu UI library · publishable
    │   ├── Cargo.toml
    │   ├── examples/
    │   │   └── demo.rs         ← kitchen-sink window, exercises all particles
    │   └── src/
    │       ├── lib.rs
    │       ├── renderer.rs     ← Layer A: declares scene submodules
    │       ├── renderer/
    │       │   ├── camera.rs
    │       │   ├── node_pipeline.rs
    │       │   └── edge_pipeline.rs
    │       ├── particles.rs    ← Layer B: declares particle submodules
    │       ├── particles/
    │       │   ├── surface.rs
    │       │   ├── stack.rs
    │       │   ├── source.rs
    │       │   ├── field.rs
    │       │   ├── trigger.rs
    │       │   ├── sink.rs
    │       │   ├── view.rs
    │       │   └── signal.rs
    │       ├── layout.rs       ← measure + arrange engine
    │       ├── input.rs        ← winit → UiEvent routing
    │       ├── text.rs         ← glyphon wrapper
    │       └── seam.rs         ← pod seam renderer + drag handling
    │
    ├── hypernode/              ← HyperNode trait + Hilbert indexing · publishable
    │   ├── Cargo.toml
    │   ├── examples/
    │   │   └── graph.rs        ← builds and queries a node graph
    │   └── src/
    │       └── lib.rs
    │
    └── p2p-swarm/              ← P2P transport · publishable  (future phase)
        ├── Cargo.toml
        ├── examples/
        │   └── peer.rs         ← spins up two peers and exchanges a message
        └── src/
            └── lib.rs
```

**Root `Cargo.toml` — workspace root and app package:**

```toml
[package]
name        = "innovator"
version     = "0.1.0"
edition     = "2024"
description = "Spatial hypergraph engineering platform"

[[bin]]
name = "innovator"
path = "src/main.rs"

[dependencies]
hyper-ui    = { workspace = true }
hypernode   = { workspace = true }
infinite-db = { workspace = true }
winit       = { workspace = true }

[workspace]
resolver = "3"
members  = [
    ".",
    "crates/hyper-ui",
    "crates/hypernode",
    "crates/p2p-swarm",
]

[workspace.dependencies]
# local publishable crates
hyper-ui  = { path = "crates/hyper-ui",  version = "0.1" }
hypernode = { path = "crates/hypernode", version = "0.1" }
p2p-swarm = { path = "crates/p2p-swarm", version = "0.1" }

# crates.io — used as published versions
infinite-db  = "0.2"     # already on crates.io · do not add to crates/

# shared external dependencies
wgpu         = "30"
winit        = "0.30"
glyphon      = "0.7"
bytemuck     = { version = "1", features = ["derive"] }
cosmic-text  = "0.12"
flume        = "0.11"
dashmap      = "6"
tokio        = { version = "1", features = ["full"] }
```

Each publishable crate's `Cargo.toml` is self-contained — it declares its
own `description`, `license`, `repository`, and only the dependencies it
actually needs:

```toml
# crates/hyper-ui/Cargo.toml
[package]
name        = "hyper-ui"
version     = "0.1.0"
edition     = "2024"
description = "Retained-mode wgpu UI library for spatial hypergraph applications"
license     = "MIT OR Apache-2.0"
repository  = "https://github.com/yourname/innovator"

[dependencies]
wgpu.workspace        = true
winit.workspace       = true
glyphon.workspace     = true
bytemuck.workspace    = true
cosmic-text.workspace = true
# no innovator-specific deps — publishable as-is
```

**`src/main.rs` — wires crates, contains no logic:**
```rust
fn main() {
    let event_loop = winit::event_loop::EventLoop::new().unwrap();
    let renderer   = hyper_ui::HyperRenderer::new(&event_loop);
    let db         = infinite_db::InfiniteDb::open_or_create("innovator.db");
    let workspace  = crate::workspace::WorkspaceShell::new(renderer, db);
    event_loop.run_app(&mut workspace).unwrap();
}
```

`main.rs` wires. It does not compute. Every `if`, every `match`, every
meaningful type lives in a module under `src/` or a crate under `crates/`.

### 0.1 Renderer bootstrap (winit + wgpu surface)

The entry point every workspace uses to get a wgpu surface on a Win32 window.

```rust
pub struct HyperRenderer {
    pub device:   wgpu::Device,
    pub queue:    wgpu::Queue,
    pub surface:  wgpu::Surface<'static>,
    pub config:   wgpu::SurfaceConfiguration,
    pub scene:    SceneRenderer,    // Layer A
    pub ui:       UiRenderer,       // Layer B
    pub text:     TextRenderer,     // glyphon
}

impl HyperRenderer {
    pub fn new(window: Arc<winit::window::Window>) -> Self { ... }
    pub fn resize(&mut self, size: PhysicalSize<u32>) { ... }
    pub fn begin_frame(&mut self) -> FrameCtx { ... }
    pub fn end_frame(&mut self, ctx: FrameCtx) { ... }
}
```

Every frame: `begin_frame` → application submits draw commands to
`FrameCtx` → `end_frame` flushes to the swap chain. The application
never touches wgpu directly after setup.

**Dependencies:**
```toml
wgpu         = "30"
winit        = "0.30"
glyphon      = "0.7"
bytemuck     = { version = "1", features = ["derive"] }
cosmic-text  = "0.12"   # backing glyphon's font system
```

### 0.2 Layer A — scene renderer (hypergraph canvas)

Renders the spatial hypergraph as a navigable 2D scene.
All coordinates are in **world space** (f64 Hilbert-derived).
The scene camera converts world → screen for each frame.

#### 0.2.1 Camera

```rust
pub struct SceneCamera {
    pub center:    Vec2,   // world-space center of viewport
    pub zoom:      f32,    // pixels per world unit
    pub screen_px: UVec2, // current viewport size in pixels
}

impl SceneCamera {
    pub fn world_to_screen(&self, p: Vec2) -> Vec2 { ... }
    pub fn screen_to_world(&self, p: Vec2) -> Vec2 { ... }
    pub fn pan(&mut self, delta_screen: Vec2) { ... }
    pub fn zoom_at(&mut self, anchor_screen: Vec2, factor: f32) { ... }
}
```

#### 0.2.2 Node geometry pipeline

Nodes are rendered as instanced quads. One draw call per node `SpaceClass`
bucket (all `UIView` nodes share one pipeline, all `Function` nodes share
another). Each instance carries: world position, size, color, border radius,
border width, selection state.

```rust
#[repr(C)]
#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct NodeInstance {
    pub position:      [f32; 2],  // screen space after camera transform
    pub size:          [f32; 2],
    pub color:         [f32; 4],  // RGBA
    pub border_color:  [f32; 4],
    pub border_radius: f32,
    pub border_width:  f32,
    pub _pad:          [f32; 2],
}
```

Shader: SDF rounded rect. One vertex shader, one fragment shader.
No per-node draw calls — the CPU builds the instance buffer each frame
from the visible node set (culled by camera frustum).

#### 0.2.3 Hyperedge pipeline

Directed hyperedges rendered as cubic Bézier strips with an arrowhead.
Each edge: source centroid → control points → target centroid.
Control points computed from the edge's source/target positions and
a curvature parameter on the `HyperEdge`.

```rust
pub struct EdgeDrawCmd {
    pub p0:        [f32; 2],  // source
    pub p1:        [f32; 2],  // control 1
    pub p2:        [f32; 2],  // control 2
    pub p3:        [f32; 2],  // target
    pub color:     [f32; 4],
    pub width:     f32,
    pub arrow:     bool,
    pub edge_kind: EdgeKindGpu,  // u32: Signal=0 Stream=1 Wave=2 Binding=3
}
```

`EdgeKindGpu` drives the dash pattern in the fragment shader:
solid for `Signal`, animated dash for `Stream`, pulsing opacity for `Wave`.

#### 0.2.4 Spatial culling

The scene can contain millions of nodes. Only nodes inside the camera
frustum (with a margin) are uploaded to the instance buffer each frame.
Culling uses the Hilbert-indexed `infinite-db` range query —
"give me all nodes with Hilbert coordinates intersecting this screen rect."

```rust
impl SceneRenderer {
    pub fn cull_and_upload(&mut self, camera: &SceneCamera, db: &InfiniteDb) {
        let world_rect = camera.visible_world_rect();
        let nodes = db.query_spatial_range(world_rect);
        self.upload_node_instances(nodes);
        let edges = db.query_edges_for_nodes(&nodes);
        self.upload_edge_commands(edges);
    }
}
```

**Deliverables — Layer A:**
- [ ] `HyperRenderer` bootstrap: wgpu device, surface, swap chain
- [ ] `SceneCamera`: pan, zoom, world↔screen transforms
- [ ] Node instance pipeline: SDF rounded rect shader, instanced draw
- [ ] Hyperedge pipeline: Bézier strip, arrowhead, per-kind dash pattern
- [ ] Spatial culling via `infinite-db` Hilbert range query
- [ ] winit event loop integration (`ApplicationHandler` trait, wgpu 30 style)

### 0.3 Layer B — UI particle renderer

A retained-mode widget system. The application builds a **particle tree**
each frame (or on dirty). The renderer diffs it against the previous frame
and emits the minimal set of wgpu draw calls.

The particle tree is a lightweight description — no GPU state. The renderer
owns all GPU resources.

#### 0.3.1 Particle tree node

```rust
pub enum Particle {
    Surface(SurfaceParticle),
    Stack(StackParticle),
    Slot(SlotParticle),
    Source(SourceParticle),
    Field(FieldParticle<dyn NumericValue>),
    Trigger(TriggerParticle),
    Sink(SinkParticle),
    View(ViewParticle),
    Signal(SignalParticle),
}

// Every particle has layout output after measure/arrange pass
pub struct LayoutBox {
    pub origin: Vec2,   // screen space, top-left
    pub size:   Vec2,
}
```

#### 0.3.2 Layout engine

Two-pass: **measure** (bottom-up, children report desired size) then
**arrange** (top-down, parent assigns final rects).

```rust
pub trait ParticleLayout {
    // bottom-up: what size do I want given available space?
    fn measure(&self, available: Vec2, children: &[Particle]) -> Vec2;
    // top-down: given my assigned rect, place my children
    fn arrange(&mut self, rect: Rect, children: &mut [Particle]);
}
```

`Stack(row)` measure: sum of child widths + gaps, max child height.
`Stack(column)` measure: max child width, sum of child heights + gaps.
`Surface` measure: delegates to its single child + padding.
`Source` measure: text bounds from glyphon's `Buffer::layout_runs()`.
`Field` measure: fixed height (36px), flex width (fills available or fixed).
`Trigger` measure: text bounds + 24px horizontal padding, 36px height.
`View` measure: fills all available space (greedy).

#### 0.3.3 Render pass order

Per frame, after layout:

```
1. surface backgrounds    (SDF rounded rect, same pipeline as Layer A nodes)
2. text                   (glyphon TextAtlas, one draw call for all text)
3. field backgrounds      (separate pipeline — has focus ring state)
4. field text             (glyphon, separate layer so it clips to field bounds)
5. trigger backgrounds    (hover/active state drives color)
6. trigger text
7. view particles         (each view owns its own wgpu render pass, composited here)
8. focus ring             (drawn last, above everything)
```

#### 0.3.4 Text rendering (glyphon)

All text in the UI goes through a single `glyphon::TextAtlas` and
`glyphon::TextRenderer`. Font: system UI font (Segoe UI on Windows),
fallback to embedded subset. Two sizes: 14px body, 12px secondary.
Two weights: 400 regular, 500 medium.

```rust
pub struct TextRenderer {
    atlas:    glyphon::TextAtlas,
    renderer: glyphon::TextRenderer,
    cache:    HashMap<TextKey, glyphon::Buffer>, // keyed by (text, size, weight)
}
```

Text is cached by content. A `source` particle with the same string
reuses the cached `glyphon::Buffer` — no re-layout unless the text changes.

#### 0.3.5 Input routing

winit `WindowEvent`s are routed to `sink` particles by hit-testing the
layout tree. The hit test walks the particle tree in reverse paint order
(topmost first) and finds the first `sink` or `field` or `trigger` whose
`LayoutBox` contains the cursor position.

```rust
pub struct InputRouter {
    pub focused: Option<ParticleId>,  // field or sink with keyboard focus
    pub hovered: Option<ParticleId>,  // trigger or sink under cursor
    pub pressed: Option<ParticleId>,  // trigger being held
}

impl InputRouter {
    pub fn route(&mut self, event: &WindowEvent, tree: &ParticleTree)
        -> Vec<UiEvent> { ... }
}

pub enum UiEvent {
    TriggerFired(ParticleId),
    FieldCommit { id: ParticleId, value: FieldValue },
    FieldEditing { id: ParticleId, raw: String },
    SinkPointer { id: ParticleId, pos: Vec2, kind: PointerKind },
    FocusChanged { from: Option<ParticleId>, to: Option<ParticleId> },
}
```

`UiEvent`s are what the application layer sees — never raw winit events.
Each `UiEvent` can fire a `HyperEdge` signal in the hypergraph.

#### 0.3.6 Dirty tracking

The particle tree uses a generational dirty flag. When a `FieldParticle`
value changes or a `SourceParticle` text changes, it marks itself dirty.
The renderer only re-lays-out and re-uploads dirty subtrees.

```rust
pub struct DirtyFlags {
    pub layout:  BitVec,   // indexed by ParticleId
    pub paint:   BitVec,
    pub text:    BitVec,
}
```

A `Signal` hyperedge arriving from the engine marks the relevant
`source` particles dirty. Only those particles re-render; nothing else.

**Deliverables — Layer B:**
- [ ] `Particle` enum and `ParticleTree` retained structure
- [ ] `LayoutEngine`: measure + arrange passes for all particle types
- [ ] `SurfaceParticle` render: SDF rect, background color, border, clip
- [ ] `StackParticle` render: row and column modes, gap, alignment
- [ ] `SourceParticle` render: glyphon text, color variants (primary/secondary/muted)
- [ ] `FieldParticle` render: background, focus ring, edit/idle/invalid states
- [ ] `TriggerParticle` render: idle/hover/active/disabled states
- [ ] `ViewParticle` render: delegates to IO-supplied render pass
- [ ] `InputRouter`: hit test, focus, hover, UiEvent dispatch
- [ ] `DirtyFlags`: generational dirty tracking, partial re-render
- [ ] Text cache: glyphon Buffer reuse by content key

### 0.4 Seam renderer (pod borders)

Pod seams are drawn by the library, not the application. The seam is a
1px line between adjacent pods with a 6px invisible hit area for dragging.
When hovered, the hit area highlights. When dragged, the seam updates
the `PodTree` split ratio in real time and triggers a layout recompute.

```rust
pub struct SeamRenderer {
    pub seams: Vec<SeamDrawCmd>,
}

pub struct SeamDrawCmd {
    pub start:     Vec2,
    pub end:       Vec2,
    pub direction: SeamDirection,  // Horizontal | Vertical
    pub hovered:   bool,
    pub dragging:  bool,
}
```

**Deliverables — Seam renderer:**
- [ ] Seam line draw (1px, `--border-strong` equivalent color)
- [ ] Seam hit area (6px invisible, cursor changes to resize on hover)
- [ ] Seam drag updates `PodTree` ratio, broadcasts layout recompute
- [ ] Double-click seam → reset to 50/50 split

### 0.5 engineer_input — first composite particle

Build and test this before any IO. It is the proof that the particle
system composes correctly. If `engineer_input` works, the rest of the
form follows.

```rust
pub struct EngineerInput {
    pub label: SourceParticle,    // flex: fixed, style: secondary
    pub value: FieldParticle<f64>,
    pub unit:  SourceParticle,    // flex: fixed, style: muted
}

// Renders as:
// surface → stack(row) → [label | field | unit]
// field.on_commit fires Signal hyperedge with ValueChanged(key, f64)
```

`EngineerInput` is not a new `Particle` variant — it is a function
that returns a `Particle::Surface(...)` subtree. Composition, not
inheritance.

**Deliverables — engineer_input:**
- [ ] `engineer_input(label, value, unit, on_commit) -> Particle` builder fn
- [ ] `FieldParticle<f64>` edit state machine: Idle → Editing → Committed | Invalid
- [ ] Keyboard nav: Tab moves focus to next field, Shift+Tab backward
- [ ] Escape reverts to `committed_value`
- [ ] Read-only mode: field renders as source, no focus ring

### 0.6 Library acceptance criteria

The library is ready for Phase 1 when all of the following pass:

- [ ] A window opens with a wgpu surface on Windows
- [ ] A `surface → stack(column) → [source, engineer_input, trigger]` tree
      renders correctly at 60fps
- [ ] Resizing the window re-lays-out without artifacts
- [ ] Typing in a `field`, committing with Enter, and reverting with Escape
      all work correctly
- [ ] A `trigger` shows idle/hover/active states on mouse interaction
- [ ] A seam between two mock pods is draggable and updates their sizes
- [ ] A hyperedge `Signal` arriving from a background thread updates a
      `source` particle's text on the next frame without a full re-render

---

## Phase 1 — Workspace shell

### 1.1 Workspace struct

```rust
pub struct ConcreteWallWorkspace {
    pub id:           WorkspaceId,
    pub renderer:     HyperRenderer,    // from hyper-ui
    pub top_bar:      TopBarIO,
    pub pages:        Vec<Page>,        // Navigation, Analysis, Results
    pub pod_tree:     PodTree,          // layout state for all pages
    pub screen_class: ScreenClass,
    pub active_wall:  Option<NodeId>,
}

pub enum ScreenClass {
    Desktop,   // all 3 pages visible simultaneously
    Tablet,    // Navigation collapses to icon strip
    Mobile,    // one page fills screen, swipe or trigger to switch
}
```

### 1.2 Top bar IO

```
surface
└── stack (row)
    ├── source  · "Wall Analysis"      (title, fixed)
    ├── trigger · "New Wall"           → Signal::NewWall
    ├── trigger · "Save"               → Signal::Save
    ├── trigger · "Run"                → Signal::RunAnalysis  (primary)
    └── trigger · "Export"             → Signal::Export
```

`Run` is the only primary-colored trigger.
`Run` fires a `Signal` hyperedge subscribed to by the analysis engine,
the Results page, and the Navigation page (for live status badges).

### 1.3 Deliverables

- [ ] `WorkspaceShell` creates window, boots `HyperRenderer`, renders top bar
- [ ] `ScreenClass` detection on window resize event
- [ ] Top bar fixed height (44px); pages fill remaining area
- [ ] Page seams draggable via `SeamRenderer`

---

## Phase 2 — Navigation page

### Purpose

All walls in the analysis visible at once. Click to set active wall.
Active wall state propagates to Analysis and Results pages immediately.

### Pod layout

```
Navigation page
└── V-split 0.35 / 0.65
    ├── pod · WallListIO
    └── pod · WallSummaryIO
```

### 2.1 WallListIO

```
surface
└── stack (column)
    ├── source · "Walls"
    ├── [per wall: surface(selectable) → stack(row) → [source·name, source·badge]]
    └── trigger · "+ New Wall" → Signal::NewWall
```

Each row is a `sink`. Click fires `Signal::WallSelected(NodeId)`.
Active row: accent surface tint.

### 2.2 WallSummaryIO

Subscribes to `Signal::WallSelected`. Re-renders read-only summary of
the active wall from its `HyperNode` properties. All `source` particles.

### Deliverables

- [ ] `WallListIO`: list renders, selection fires signal
- [ ] `WallSummaryIO`: re-renders on `WallSelected`
- [ ] New wall: creates `HyperNode` in `infinite-db`, adds to list
- [ ] Active wall at workspace level — all pages read the same `NodeId`

---

## Phase 3 — Analysis page

### Purpose

Input all parameters for the active wall. View live cross-section.
Custom fields buildable at runtime.

### Pod layout

```
Analysis page
└── V-split 0.30 / 0.70
    ├── pod · InputFormIO
    └── pod · WallViewIO
```

### 3.1 InputFormIO — standard sections

Geometry, Material, Reinforcement, Loading sections.
Each row is an `engineer_input` from Phase 0.

See data model for full field list. Every field's `on_commit` fires
`Signal::ValueChanged(prop_key, FieldValue)` which writes back to the
wall's `HyperNode` in `infinite-db`.

### 3.2 InputFormIO — custom section (runtime)

```
surface(accent border)
└── stack(column)
    ├── source · "Custom ✦"
    ├── [dynamic: engineer_input per runtime field]
    └── trigger · "+ Add field" → opens FieldBuilderIO inline
```

**FieldBuilderIO** — inline IO, assembled from particles, no modal:

```
surface → stack(column) → [
    field·text  "Label",
    field·f64   "Initial value",
    field·text  "Unit",
    sink·chips  "Type"   (Number | Text | Bool),
    stack(row)  [field·f64 "Min", field·f64 "Max"],
    stack(row)  [trigger·"Cancel", trigger·"Add"]
]
```

"Add" appends a new property edge to the wall `HyperNode`.
Right-click custom label → "Promote to workspace" →
fires `Wave` hyperedge → all wall nodes get the property with default value.

### 3.3 WallViewIO

`view` particle. 2D canvas. Redraws on `Signal::ValueChanged` from
geometry or reinforcement fields. Rendered by `HyperRenderer` Layer A.
Draws concrete outline, rebar, dimension lines, cover indicators.
`sink` · pointer owns pan/zoom.

### Deliverables

- [ ] `InputFormIO` all standard sections, fields commit to `HyperNode`
- [ ] `FieldBuilderIO` creates runtime fields on the node
- [ ] "Promote to workspace" `Wave` hyperedge
- [ ] `WallViewIO` subscribes to geometry signals, redraws section
- [ ] Pod width < 280px: labels abbreviate (pod-level `SizeClass`)

---

## Phase 4 — Results page

### Purpose

Live analysis output. Updates on `Signal::AnalysisComplete`.

### Pod layout

```
Results page
└── H-split 0.70 / 0.30
    ├── pod · ResultsTableIO
    └── pod · StatusIO
```

### 4.1 ResultsTableIO

Per-check rows: name, demand, capacity, ratio, pass/fail badge.
Iterates wall node's property edges — custom fields appear automatically.
Multi-wall mode: one column per wall (Phase 6).

### 4.2 StatusIO

Overall pass/fail, run time, code reference, Export PDF trigger.

### 4.3 Engine wiring

```
Signal::RunAnalysis
  → AnalysisEngine (Function HyperNode)
  → reads WallNode props
  → writes ResultsNode to infinite-db
  → fires Signal::AnalysisComplete
  → ResultsTableIO + StatusIO re-render
```

### Deliverables

- [ ] `ResultsTableIO`: pass/fail rows, custom fields dynamic
- [ ] `StatusIO`: summary, export trigger
- [ ] Results node written to `infinite-db`
- [ ] `AnalysisComplete` signal wires engine → results page
- [ ] Export PDF (existing pdf skill, reads ResultsNode)

---

## Phase 5 — Multi-wall batch

- `Signal::RunAnalysis(scope: AllWalls)` runs every wall node
- `ResultsTableIO` switches to column-per-wall layout
- `WallListIO` shows live pass/fail badges per wall during run
- Custom fields missing on some walls show `—`

### Deliverables

- [ ] Batch run mode on `RunAnalysis`
- [ ] `ResultsTableIO` multi-wall columns
- [ ] Live badges on `WallListIO` during batch

---

## Phase 6 — Pod adaptive behavior

Rules live on the pod, not the IO. Pod passes `SizeClass` down.

| Pod width | SizeClass | Behavior |
|---|---|---|
| > 320px | `Full` | Full labels, values, units |
| 200–320px | `Compact` | Labels abbreviated |
| < 200px | `Minimal` | Labels hidden, units as tooltip |

### Deliverables

- [ ] `SizeClass` enum on `PodState`
- [ ] `InputFormIO` reads `SizeClass`, adjusts label display
- [ ] Seam drag triggers `SizeClass` recompute on both pods

---

## Data model summary

```
WallNode (HyperNode)
├── prop: wall_name        String
├── prop: wall_type        String   "special_concrete"
├── prop: height           f64      ft
├── prop: length           f64      ft
├── prop: thickness        f64      in
├── prop: clear_cover      f64      in
├── prop: fc               f64      psi
├── prop: fy               f64      psi
├── prop: es               f64      ksi  (fixed 29000)
├── prop: lambda           f64           (lightweight factor)
├── prop: vert_bar_size    u8            (bar number 3–11)
├── prop: vert_spacing     f64      in
├── prop: horiz_bar_size   u8
├── prop: horiz_spacing    f64      in
├── prop: pu               f64      kips (factored axial)
├── prop: vu               f64      kips (factored shear)
├── prop: mu               f64      kip-ft (factored moment)
└── [runtime props]        any      user-defined key/value

ResultsNode (HyperNode, written by engine)
├── prop: wall_id          NodeId
├── prop: run_timestamp    i64
├── prop: checks           Vec<CheckResult>
└── prop: governing        String

CheckResult { name, demand, capacity, ratio, pass }
```

---

## Build order

```
Phase 0a  hyper-ui · Layer A (scene renderer, camera, node/edge pipelines)
Phase 0b  hyper-ui · Layer B (particle tree, layout engine, input router)    ← parallel with 0a
Phase 0c  hyper-ui · SeamRenderer + engineer_input + acceptance tests
Phase 1   wall-analysis · workspace shell (depends on 0c passing)
Phase 2   wall-analysis · navigation page
Phase 3a  wall-analysis · InputFormIO — standard sections                    ← parallel with 3b
Phase 3b  wall-analysis · WallViewIO — cross-section render
Phase 3c  wall-analysis · FieldBuilderIO — runtime fields (depends on 3a stable)
Phase 4   wall-analysis · results page (depends on 3a for committed values)
Phase 5   wall-analysis · multi-wall batch (depends on 4)
Phase 6   wall-analysis · pod adaptive behavior (layers onto any phase)
```

Phase 0 is the critical path. Nothing else can start until 0c acceptance
criteria pass. Phases 0a and 0b can run in parallel in Cursor (separate
files, no shared mutable state until they connect in 0c).

**Running and testing:**
```sh
# run the app
cargo run

# run the app (release build)
cargo run --release

# exercise hyper-ui in isolation — opens a particle kitchen-sink window
cargo run -p hyper-ui --example demo

# exercise hypernode in isolation — builds and queries a node graph
cargo run -p hypernode --example graph

# exercise p2p-swarm in isolation — spins up two peers
cargo run -p p2p-swarm --example peer

# test a specific crate
cargo test -p hyper-ui
cargo test -p hypernode

# test the app modules
cargo test -p innovator

# test everything in the workspace
cargo test --workspace
```

`cargo run` at the workspace root runs `Innovator`. Each publishable crate's
`examples/` directory exercises that crate's public API with zero Innovator
context — if an example requires Innovator-specific types, the crate boundary
is wrong. `infinite-db` tests run against the published version on crates.io;
no local copy needed.
