# hyper-ui container refactor

## Problem statement

`hyper-ui` has no coherent concept of a container hierarchy. The three
levels of the UI — workspace, page, pod — are not distinct modules with
distinct types and behaviors. Instead:

- `PodTree` lives under `seam/` as a side-effect of the seam renderer
  needing a tree to walk. A pod is structurally a leaf of a binary
  split tree, which is the wrong model entirely.
- `PageTree` / `PageNode` live under `page_tree/` but their substructure
  still delegates to `PodTree` — inheriting the split-tree shape.
- There is no `workspace` module in `hyper-ui`. The workspace concept
  only exists in `Innovator/src/workspace/`.
- `seam/` conflates two unrelated things: the seam renderer machinery
  and the pod layout data structure.

The result: a developer opening `hyper-ui` for the first time cannot
locate where workspace, page, or pod are defined or what behaviors each
level owns. Navigation is guesswork.

---

## Vocabulary — final definitions

| Level | Splits? | Collapses? | Owns |
|---|---|---|---|
| **Workspace** | — | — | `PageTree`, global tab strip, window area |
| **Page** | Yes (H or V, user-draggable) | — | `PodList`, optional header, optional icon rail |
| **Pod** | No | Yes | `PodId`, `collapsed: bool`, height, content slot |
| **IO** | — | — | Particles, signals — application-defined |

Key changes from the current model:

- **Pods do not split.** The current `PodTree` binary-split shape is
  deleted. Pod layout is a flat ordered list — a vertical stack.
- **Pods are collapsible.** A pod can be collapsed to its title bar
  height, releasing vertical space to its siblings.
- **`seam/` becomes page-seam-only.** The `SeamRenderer` and
  `SeamDrawCmd` machinery serves page seams only. Pod dividers — the
  thin drag handles between stacked pods — are a separate, simpler
  mechanism owned by `pod/`.
- **`workspace/` is a new top-level module** in `hyper-ui`, giving the
  container hierarchy a navigable home.

---

## New module structure — `crates/hyper-ui/src/`

```
crates/hyper-ui/src/
├── lib.rs

├── workspace.rs             ← declares submodules; re-exports WorkspaceShell
├── workspace/
│   └── shell.rs             ← WorkspaceShell: PageTree, area Rect, next_page_id

├── page.rs                  ← declares submodules; re-exports PageNode, PageTree, PageId, …
├── page/
│   ├── node.rs              ← PageNode: id, pods, header, icon_rail
│   ├── id.rs                ← PageId(u32)
│   ├── tree.rs              ← PageTree: binary split; split/merge/find/leaf_rects
│   ├── seam_id.rs           ← PageSeamId(u32)
│   ├── side.rs              ← PageSide enum
│   ├── header.rs            ← PageHeaderConfig, PageHeaderSlots
│   ├── icon_rail.rs         ← IconRailConfig, IconRailSide
│   ├── content_rect.rs      ← PageNode::content_rect(), header_rect()
│   └── leaf_rects.rs        ← PageTree::leaf_rects()

├── pod.rs                   ← declares submodules; re-exports Pod, PodId, PodList
├── pod/
│   ├── pod.rs               ← Pod: id, collapsed, min_height, title
│   ├── id.rs                ← PodId(u32)
│   ├── list.rs              ← PodList: Vec<Pod>, gap; layout → Vec<(PodId, Rect)>
│   ├── divider.rs           ← PodDivider: thin drag strip between pods
│   └── collapse.rs          ← collapse/expand logic, animated height (future)

├── seam.rs                  ← page seams only
├── seam/
│   ├── direction.rs         ← SeamDirection
│   ├── draw_cmd.rs          ← SeamDrawCmd (page seams only; is_page_seam field removed)
│   ├── renderer.rs          ← SeamRenderer: handles page-seam drag/right-click
│   ├── renderer/
│   │   ├── new.rs
│   │   ├── handle_event.rs
│   │   ├── rebuild_from_page_tree.rs
│   │   └── draw.rs
│   ├── rebuild_seams.rs     ← rebuild_page_seams(); pod seam rebuild removed
│   └── split_rect.rs        ← split_rect() geometry helper

├── particles.rs             ← unchanged
├── particles/ …             ← unchanged
├── renderer.rs              ← unchanged (Layer A)
├── renderer/ …              ← unchanged
├── layout.rs                ← unchanged
├── input.rs                 ← unchanged (UiEvent gains PodCollapse/PodDividerDrag)
├── input/
│   └── event.rs             ← adds: PodCollapse { id: PodId },
│                                      PodDividerDrag { above: PodId, delta: f32 }
├── text.rs                  ← unchanged
├── geom.rs                  ← unchanged
└── ui.rs                    ← UiRenderer: pod_seams field removed; pod_dividers added
    └── ui/
        └── renderer.rs      ← UiRenderer owns page_seams: SeamRenderer only
```

---

## Data structures

### `Pod`

```rust
// pod/pod.rs
#[derive(Debug, Clone)]
pub struct Pod {
    pub id:         PodId,
    pub collapsed:  bool,
    pub min_height: f32,    // minimum height when expanded; default 80.0
    pub title:      String, // shown in title bar when collapsed
}
```

A pod has no knowledge of its content. Content (particles) lives in the
application layer and is mapped to a `PodId` the same way IO is currently
mapped to a pod-leaf id.

### `PodList`

```rust
// pod/list.rs
#[derive(Debug, Clone)]
pub struct PodList {
    pub pods: Vec<Pod>,
    pub gap:  f32,          // pixel gap between pod dividers; default 1.0
}

impl PodList {
    /// Compute (PodId, Rect) for every pod given the available content rect.
    /// Collapsed pods receive only their title bar height (24.0 px).
    pub fn layout(&self, area: Rect) -> Vec<(PodId, Rect)> { … }

    pub fn collapse(&mut self, id: PodId) { … }
    pub fn expand(&mut self, id: PodId) { … }
    pub fn toggle(&mut self, id: PodId) { … }

    /// Apply a divider drag: redistribute height between the pod above and below.
    pub fn apply_divider_drag(&mut self, above: PodId, delta: f32, area_height: f32) { … }
}
```

`layout()` replaces all of `PodTree::leaf_rects()`. It is simpler: one
linear pass, top to bottom, no recursion.

### `PageNode` — updated

```rust
// page/node.rs
pub struct PageNode {
    pub id:        PageId,
    pub pods:      PodList,                   // replaces pod_tree: PodTree
    pub header:    Option<PageHeaderConfig>,
    pub icon_rail: Option<IconRailConfig>,
}
```

### `WorkspaceShell`

```rust
// workspace/shell.rs
pub struct WorkspaceShell {
    pub pages:        PageTree,
    pub area:         Rect,
    pub next_page_id: u32,
}

impl WorkspaceShell {
    pub fn new(area: Rect) -> Self { … }
    pub fn next_id(&mut self) -> PageId { … }
}
```

`WorkspaceShell` in `hyper-ui` is generic — it knows nothing about
`Innovator`. The Innovator `AnalysisWorkspace` embeds or wraps it.

---

## `seam/` — page seams only

`SeamDrawCmd` drops `is_page_seam` and `page_seam_id` — all seams in
this renderer are now page seams by definition.

```rust
pub struct SeamDrawCmd {
    pub start:      Vec2,
    pub end:        Vec2,
    pub direction:  SeamDirection,
    pub hovered:    bool,
    pub dragging:   bool,
    pub seam_id:    PageSeamId,   // was page_seam_id: Option<PageSeamId>
    pub split_area: Rect,
}
```

`rebuild_seams.rs` keeps only `rebuild_page_seams()`. The
`rebuild_pod_seams()` function is deleted.

`SeamRenderer` drops `rebuild_from_pods()` and `append_from_pods()`.
These had no callers outside the pod-seam path.

---

## `pod/divider.rs` — replaces pod seam machinery

A pod divider is not a seam. It is a thin horizontal strip between
vertically stacked pods. It supports:
- Hover highlight
- Left-drag to redistribute height between the pod above and below
- Double-click to equalize heights

```rust
// pod/divider.rs
pub struct PodDivider {
    pub above:    PodId,
    pub rect:     Rect,    // thin hit rect, rebuilt each frame from PodList::layout()
    pub hovered:  bool,
    pub dragging: bool,
}

pub struct PodDividerRenderer {
    dividers: Vec<PodDivider>,
}

impl PodDividerRenderer {
    pub fn rebuild(&mut self, layout: &[(PodId, Rect)], gap: f32) { … }
    pub fn handle_event(
        &mut self,
        event: &WindowEvent,
        cursor: Vec2,
    ) -> Vec<UiEvent> { … }
    pub fn draw(&self, …) { … }
}
```

`UiEvent` gains two new variants:

```rust
PodCollapse   { id: PodId },
PodDividerDrag { above: PodId, delta: f32 },
```

The application handles these by calling `PodList::toggle()` or
`PodList::apply_divider_drag()` and rebuilding the layout.

---

## `UiRenderer` — updated

```rust
pub struct UiRenderer {
    pub rects:      NodePipeline,
    pub focus_ring: NodePipeline,
    pub tree:       ParticleTree,
    pub input:      InputRouter,
    pub page_seams: SeamRenderer,        // page boundaries only
    // pod_seams: SeamRenderer  ← DELETED
    // Pod dividers are owned per-page by the application layer or a
    // future PodDividerRenderer stored alongside AnalysisWorkspace.
}
```

Pod dividers are intentionally not stored on `UiRenderer` because
divider count varies per page and per frame as pods collapse/expand.
The application builds and draws them as part of its per-page pass.

---

## `lib.rs` — updated public API

```rust
// workspace
pub mod workspace;
pub use workspace::WorkspaceShell;

// page
pub mod page;
pub use page::{
    IconRailConfig, IconRailSide,
    PageHeaderConfig, PageHeaderSlots,
    PageId, PageNode, PageSeamId, PageSide, PageTree,
};

// pod
pub mod pod;
pub use pod::{Pod, PodDivider, PodDividerRenderer, PodId, PodList};

// seam (page seams only)
pub mod seam;
pub use seam::{SeamDirection, SeamDrawCmd, SeamRatioAction, SeamRenderer};

// unchanged
pub mod particles;   pub use particles::{…};
pub mod renderer;    pub use renderer::{…};
pub mod layout;      pub use layout::{LayoutBox, LayoutEngine};
pub mod input;       pub use input::{InputRouter, UiEvent};
pub mod text;        pub use text::TextRenderer;
pub mod geom;        pub use geom::{Rect, UVec2, Vec2, WorldRect};
pub mod engineer;    pub use engineer::{…};
pub mod ui;          pub use ui::{apply_signal_text, UiRenderer};

// REMOVED from public API:
// PodTree         — replaced by PodList
// SeamRatioAction (pod variant) — pod dividers use PodDividerDrag instead
```

---

## Innovator changes

### `src/workspace/analysis/templates/`

Each template replaces `pod_tree: PodTree` with `pods: PodList`:

```rust
// navigation_page.rs — before
let pod_tree = PodTree::Split {
    direction: SeamDirection::Horizontal,
    ratio: 0.35,
    first:  Box::new(PodTree::Leaf { id: 0 }),
    second: Box::new(PodTree::Leaf { id: 1 }),
};
let node = PageNode { id, pod_tree, … };

// navigation_page.rs — after
let pods = PodList {
    pods: vec![
        Pod { id: PodId(0), collapsed: false, min_height: 80.0, title: "Wall List".into() },
        Pod { id: PodId(1), collapsed: false, min_height: 80.0, title: "Summary".into() },
    ],
    gap: 1.0,
};
let node = PageNode { id, pods, … };
```

The `IoKind` assignment map `page_ios: HashMap<PageId, Vec<(u32, IoKind)>>`
changes its key from pod-leaf `u32` to `PodId`:

```rust
// before
pub page_ios: HashMap<PageId, Vec<(u32, IoKind)>>

// after
pub page_ios: HashMap<PageId, Vec<(PodId, IoKind)>>
```

### `src/workspace/app_shell/sync_from_page_tree.rs`

Replace the `page.pod_tree.leaf_rects(content_rect)` call with
`page.pods.layout(content_rect)`. Return type is the same shape:
`Vec<(PodId, Rect)>` vs the old `Vec<(u32, Rect)>`.

### `src/workspace/app_shell/rebuild_seams.rs`

Remove the pod-seam pass entirely:

```rust
// before
pub fn rebuild_seams(ws: &AnalysisWorkspace, pages_area: Rect, renderer: &mut HyperRenderer) {
    renderer.ui.page_seams.rebuild_from_page_tree(&ws.page_tree, pages_area);
    renderer.ui.pod_seams.clear();
    for (page_id, page_rect) in ws.page_tree.leaf_rects(pages_area) {
        let page = ws.page_tree.find(page_id).unwrap();
        let content_rect = page.content_rect(page_rect);
        renderer.ui.pod_seams.rebuild_from_pods(&page.pod_tree, content_rect);
    }
}

// after
pub fn rebuild_seams(ws: &AnalysisWorkspace, pages_area: Rect, renderer: &mut HyperRenderer) {
    renderer.ui.page_seams.rebuild_from_page_tree(&ws.page_tree, pages_area);
    // Pod dividers are rebuilt per-frame by each page's draw pass. No seam pass needed.
}
```

### `src/workspace/app_shell/window_event.rs`

Add handling for the two new pod events:

```rust
UiEvent::PodCollapse { id } => {
    // find which page owns this pod, toggle it
    for page in ws.page_tree.leaves_mut() {
        if page.pods.pods.iter().any(|p| p.id == id) {
            page.pods.toggle(id);
            rebuild_needed = true;
        }
    }
}
UiEvent::PodDividerDrag { above, delta } => {
    // find owning page, apply drag, rebuild
    …
}
```

Remove: `pod_ratio_action` handling and the `pod_seams.handle_event_with`
call. Pod divider drag goes through the new `PodDividerRenderer`.

### `src/workspace/analysis/build_pages.rs`

Replace `page.pod_tree.leaf_rects(…).len()` used for icon rail icon count
with `page.pods.pods.len()`.

---

## Files deleted

| File | Reason |
|---|---|
| `crates/hyper-ui/src/seam/pod_tree.rs` | replaced by `pod/list.rs` |
| `crates/hyper-ui/src/seam/pod_tree/` (all submodules) | replaced |
| `crates/hyper-ui/src/seam/renderer/rebuild_from_pods.rs` | pods no longer use SeamRenderer |
| `crates/hyper-ui/src/page_tree/` (whole directory) | moved to `page/` |
| `crates/hyper-ui/src/page_tree.rs` | replaced by `page.rs` |

---

## Files moved / renamed

| Old path | New path |
|---|---|
| `crates/hyper-ui/src/page_tree.rs` | `crates/hyper-ui/src/page.rs` |
| `crates/hyper-ui/src/page_tree/tree.rs` | `crates/hyper-ui/src/page/tree.rs` |
| `crates/hyper-ui/src/page_tree/page_node.rs` | `crates/hyper-ui/src/page/node.rs` |
| `crates/hyper-ui/src/page_tree/page_id.rs` | `crates/hyper-ui/src/page/id.rs` |
| `crates/hyper-ui/src/page_tree/page_seam_id.rs` | `crates/hyper-ui/src/page/seam_id.rs` |
| `crates/hyper-ui/src/page_tree/page_side.rs` | `crates/hyper-ui/src/page/side.rs` |
| `crates/hyper-ui/src/page_tree/page_header.rs` | `crates/hyper-ui/src/page/header.rs` |
| `crates/hyper-ui/src/page_tree/icon_rail.rs` | `crates/hyper-ui/src/page/icon_rail.rs` |
| `crates/hyper-ui/src/page_tree/content_rect.rs` | `crates/hyper-ui/src/page/content_rect.rs` |
| `crates/hyper-ui/src/page_tree/leaf_rects.rs` | `crates/hyper-ui/src/page/leaf_rects.rs` |
| `crates/hyper-ui/src/page_tree/find.rs` | `crates/hyper-ui/src/page/find.rs` |
| `crates/hyper-ui/src/page_tree/merge.rs` | `crates/hyper-ui/src/page/merge.rs` |
| `crates/hyper-ui/src/page_tree/split.rs` | `crates/hyper-ui/src/page/split.rs` |
| `crates/hyper-ui/src/page_tree/set_ratio.rs` | `crates/hyper-ui/src/page/set_ratio.rs` |

---

## Build order

Phases are sequenced so that each compiles and passes tests before the
next begins. Phases within a group can run in parallel in Cursor (separate
files, no shared mutable state until they connect).

```
Phase 1  — hyper-ui · pod module (new, no dependents yet)
Phase 2  — hyper-ui · page module (rename/move from page_tree; swap PodTree → PodList)
Phase 3  — hyper-ui · workspace module (new thin shell)
Phase 4  — hyper-ui · seam cleanup (delete pod seam code; update SeamDrawCmd)
Phase 5  — hyper-ui · UiRenderer (remove pod_seams field; add hook for dividers)
Phase 6  — hyper-ui · input (add PodCollapse, PodDividerDrag events)
Phase 7  — hyper-ui · lib.rs (update re-exports; delete old page_tree / seam/pod_tree)
Phase 8  — Innovator · templates (PodTree → PodList in all three page templates)
Phase 9  — Innovator · sync_from_page_tree (leaf_rects → pods.layout)
Phase 10 — Innovator · rebuild_seams (remove pod-seam pass)
Phase 11 — Innovator · window_event (swap pod seam events for PodCollapse/PodDividerDrag)
Phase 12 — Innovator · build_pages (pod count via pods.pods.len())
Phase 13 — cargo test --workspace (all acceptance criteria)
```

Phases 1–3 are the critical path. Nothing in Phases 8–12 can start until
Phase 7 passes `cargo check`.

---

## Acceptance criteria

- [x] `cargo check -p hyper-ui` clean after Phase 7
- [x] `cargo check --workspace` clean after Phase 12
- [x] `cargo test --workspace` passes
- [x] `cargo run -p hyper-ui --example demo` opens the demo window without
      pod-seam rects; pod dividers are visible and draggable
- [x] `cargo run` opens Innovator with three pages visually identical to
      pre-refactor
- [x] Dragging a page seam resizes pages live — unchanged
- [x] Right-clicking a page seam shows the context menu — unchanged
- [x] Pod collapse: clicking a pod title bar collapses/expands it; the
      sibling pod fills the freed space
- [x] Pod divider drag redistributes height between adjacent pods
- [x] `PodTree` is not referenced anywhere in `hyper-ui` or `Innovator`
- [x] `page_tree` module is not referenced anywhere — all imports use `page`
- [x] `pod_seams` field is not referenced anywhere in `UiRenderer`
- [x] Opening `crates/hyper-ui/src/` in a file tree shows: `workspace/`,
      `page/`, `pod/`, `seam/`, `particles/`, `renderer/` as the primary
      top-level modules — the hierarchy is immediately legible
