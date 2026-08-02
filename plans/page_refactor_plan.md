# Page container refactor — Innovator

## Problem statement

The current codebase conflates two distinct layout levels into a single
`PodTree`. `pages_pod_tree()` builds one monolithic tree that encodes
both the workspace-level page divisions (Navigation | Analysis | Results)
and the page-internal pod divisions (WallList / WallSummary, etc.) as
peers. `sync_page_layouts` then reconstructs page boundaries by unioning
leaf rects back together — a clear sign the data model is doing something
the structure was not designed for.

Consequences of the current model:
- Pages cannot be split or merged at runtime — the tree shape is fixed.
- The workspace header is a single optional construct, not per-page.
- There is no concept of an icon rail attached to a page.
- Right-click on a seam does nothing; split is not user-accessible.
- Page and pod are the same thing in the tree, making the vocabulary
  inconsistent with the design intent.

---

## Vocabulary — final definitions

| Level | Generic? | Splits? | Owns |
|---|---|---|---|
| **Workspace** | Yes | — | `PageTree`, tab strip |
| **Page** | Yes — pure container | **Yes** | `PodTree`, optional header, optional icon rail |
| **Pod** | Yes — pure slot | No | Position and size within a page |
| **IO** | No — application content | — | Particles, signals, business logic |
| **Template** | No — application | — | Returns a configured `PageNode` + IO assignments |

A page has no type. It is a container. What fills its pods is the
application's responsibility, assembled from templates. The library
(`hyper-ui`) knows nothing about walls, inputs, or results.

---

## New data structures — `hyper-ui` crate

### `PageId`

```rust
/// Stable identity for a page across tree mutations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PageId(pub u32);
```

### `PageNode`

```rust
/// A single page — a generic spatial container. No content knowledge.
pub struct PageNode {
    pub id:        PageId,
    pub pod_tree:  PodTree,          // internal pod layout, fixed per template
    pub header:    Option<PageHeaderConfig>,
    pub icon_rail: Option<IconRailConfig>,
}
```

### `PageHeaderConfig`

```rust
/// Declarative config for a page's optional header bar.
pub struct PageHeaderConfig {
    pub height:  f32,                // typically 32–64 px; variable per page
    pub slots:   PageHeaderSlots,    // what the application populates
}

pub enum PageHeaderSlots {
    /// Application builds the header particle subtree freely.
    Custom,
    /// No pre-built slots — application injects its own particle.
    None,
}
```

The header height subtracts from the page rect before it reaches the
`PodTree`. The application assembles the header particle subtree
separately and stitches it in. The library enforces the geometry only.

### `IconRailConfig`

```rust
pub enum IconRailSide { Left, Right }

pub struct IconRailConfig {
    pub side:  IconRailSide,
    pub width: f32,   // typically 32–36 px
}
```

The icon rail is a narrow column pinned to the left or right edge of the
page's content area (below the header if present). It is subtracted from
the rect passed to the `PodTree` layout. The application populates the
rail with one icon per pod leaf, in vertical order matching the pod tree.

### `PageTree`

Structurally mirrors `PodTree` — a binary split tree — but its leaves
hold `PageNode` instead of a bare `u32`.

```rust
/// Binary split tree for page-level workspace layout.
pub enum PageTree {
    Leaf(PageNode),
    Split {
        direction: SeamDirection,
        ratio:     f32,
        first:     Box<PageTree>,
        second:    Box<PageTree>,
    },
}
```

`PageTree` lives in `crates/hyper-ui/src/page_tree/`. It is a parallel
module to `crates/hyper-ui/src/seam/pod_tree/` and reuses
`SeamDirection`, `split_rect`, and `SeamDrawCmd` unchanged.

---

## Seam changes — two seam populations

The `SeamRenderer` currently manages one flat `Vec<SeamDrawCmd>` derived
from a single `PodTree`. After the refactor there are two independent
populations:

1. **Page seams** — derived from the `PageTree`. These are the boundaries
   between pages. They support split, merge, and ratio-drag.
2. **Pod seams** — derived from each `PageNode`'s `PodTree`. These are
   the boundaries between pods within a single page. They support
   ratio-drag only (no split/merge).

Both use the same `SeamDrawCmd` and `SeamRenderer` machinery. The
renderer runs twice per frame — once for page seams (fed the `PageTree`
and `pages_area`), once for all pod seams (fed each page's `PodTree` and
that page's content rect).

### `SeamDrawCmd` additions

Two boolean fields are added:

```rust
pub struct SeamDrawCmd {
    pub start:       Vec2,
    pub end:         Vec2,
    pub direction:   SeamDirection,
    pub hovered:     bool,
    pub dragging:    bool,
    // NEW
    pub is_page_seam: bool,   // true = page boundary; supports split/merge context menu
    pub page_seam_id: Option<PageSeamId>, // which split node in PageTree
}
```

### Right-click on a page seam

`SeamRenderer::handle_event` is extended to detect
`MouseButton::Right` pressed while a page seam is hovered. When detected
it emits a new `UiEvent`:

```rust
UiEvent::PageSeamRightClick {
    seam_id:   PageSeamId,
    cursor:    Vec2,
    direction: SeamDirection,
}
```

The application shell handles this event by recording the pending context
menu state. On the next frame it renders a small context menu particle
popup at the cursor position containing:

- **Split vertical** → `PageSignal::Split { seam_id, direction: Vertical }`
- **Split horizontal** → `PageSignal::Split { seam_id, direction: Horizontal }`
- **Merge ←** or **Merge ↑** (first page survives) → `PageSignal::Merge { seam_id, keep: First }`
- **Merge →** or **Merge ↓** (second page survives) → `PageSignal::Merge { seam_id, keep: Second }`
- **Reset 50/50** → `PageSignal::ResetRatio { seam_id }`

The directional merge labels and arrows adapt to the seam direction:
a vertical seam shows left/right arrows; a horizontal seam shows up/down
arrows. The user clicks the arrow pointing toward the page they want to
keep.

### Split trigger in page header

Each page header (when present) carries a split icon trigger on its right
side. It fires `PageSignal::Split` with a default direction based on the
page's current aspect ratio (wider → split vertical, taller → split
horizontal). This is an alternative entry point to the same signal —
both paths produce identical tree mutations.

---

## `PageSignal` — new signal enum

```rust
/// Signals that mutate the PageTree. Handled by the workspace, not the library.
pub enum PageSignal {
    Split {
        /// The Split node in the PageTree whose first or second child is being split.
        /// If splitting a Leaf, this is the leaf's parent seam.
        seam_id:   PageSeamId,
        direction: SeamDirection,
    },
    Merge {
        seam_id: PageSeamId,
        keep:    PageSide,        // First | Second
    },
    ResetRatio {
        seam_id: PageSeamId,
    },
}

pub enum PageSide { First, Second }
```

---

## `PageTree` mutations

### Split

Splitting a leaf replaces it with a `Split` node containing the original
leaf at 0.5 ratio and a new empty `PageNode` sibling. The new page gets a
fresh `PageId`, a default `PodTree::Leaf { id: 0 }`, and no header or
icon rail. The application then assigns IOs to its pod slots.

```
Before:
    Leaf(page_A)

After:
    Split {
        direction: <chosen>,
        ratio: 0.5,
        first:  Leaf(page_A),
        second: Leaf(page_new),    // empty — application fills it
    }
```

### Merge

Merging removes the `Split` node and promotes the surviving child in its
place. The eliminated page's `PageId` is retired; any IOs mapped to it
are dropped.

```
Before:
    Split {
        first:  Leaf(page_A),
        second: Leaf(page_B),    // keep: Second → page_B survives
    }

After:
    Leaf(page_B)
```

### Ratio drag

Unchanged from current pod seam drag — mutates the `ratio` field on the
`Split` node. The seam index → node mapping is maintained by a flat index
table rebuilt whenever the `PageTree` changes shape.

---

## Application-layer changes — `src/`

### Remove `pages_pod_tree()`

`src/workspace/analysis/pages_pod_tree.rs` is deleted. It mixed levels.

### `AnalysisWorkspace` replaces `pod_tree` with `page_tree`

```rust
pub struct AnalysisWorkspace {
    // REMOVE
    // pub pod_tree: PodTree,

    // ADD
    pub page_tree: PageTree,

    // IO assignment: PageId → Vec<(pod_leaf_id, IoKind)>
    pub page_ios: HashMap<PageId, Vec<(u32, IoKind)>>,

    // ... rest unchanged
}
```

`IoKind` is an application enum:

```rust
pub enum IoKind {
    WallList,
    WallSummary,
    InputForm,
    WallView,
    ResultsTable,
    Status,
    Empty,
}
```

### Templates — replace hardcoded page construction

Templates are functions in `src/workspace/analysis/templates/` that return
`(PageNode, Vec<(u32, IoKind)>)`. The initial three-page layout is
constructed by calling three templates:

```rust
fn navigation_page(id: PageId) -> (PageNode, Vec<(u32, IoKind)>) {
    let pod_tree = PodTree::two_column(0.35);  // existing helper
    let ios = vec![(0, IoKind::WallList), (1, IoKind::WallSummary)];
    let node = PageNode {
        id,
        pod_tree,
        header: None,
        icon_rail: Some(IconRailConfig { side: IconRailSide::Left, width: 34.0 }),
    };
    (node, ios)
}

fn analysis_page(id: PageId) -> (PageNode, Vec<(u32, IoKind)>) {
    // header carries live result ratios — configured by the application
    let header = Some(PageHeaderConfig { height: 44.0, slots: PageHeaderSlots::Custom });
    let pod_tree = PodTree::two_column(0.30);
    let ios = vec![(0, IoKind::InputForm), (1, IoKind::WallView)];
    let node = PageNode { id, pod_tree, header, icon_rail: None };
    (node, ios)
}

fn results_page(id: PageId) -> (PageNode, Vec<(u32, IoKind)>) {
    let pod_tree = PodTree::Split {
        direction: SeamDirection::Horizontal,
        ratio: 0.70,
        first:  Box::new(PodTree::Leaf { id: 0 }),
        second: Box::new(PodTree::Leaf { id: 1 }),
    };
    let ios = vec![(0, IoKind::ResultsTable), (1, IoKind::Status)];
    let node = PageNode { id, pod_tree, header: None, icon_rail: None };
    (node, ios)
}
```

### `sync_page_layouts` — replaced by `PageTree` walker

`src/workspace/app_shell/sync_page_layouts.rs` is deleted and replaced by
`src/workspace/app_shell/sync_from_page_tree.rs`, which:

1. Walks the `PageTree` to collect `(PageId, Rect)` pairs — each leaf's
   rect is computed directly by `split_rect` traversal, with no unioning.
2. For each `PageId`, subtracts the header height and icon rail width to
   get the page's content rect.
3. Walks that page's `PodTree` using the existing `leaf_rects()` method
   to get `(pod_leaf_id, Rect)` pairs.
4. Looks up the page's `IoKind` assignments and stitches the IO particles
   into the correct rects.

The unioning hack in the old `sync_two_pod_page` is gone entirely.

### `rebuild_seams` — two passes

`src/workspace/app_shell/rebuild_seams.rs` is extended to run two passes:

```rust
pub fn rebuild_seams(ws: &AnalysisWorkspace, pages_area: Rect, renderer: &mut HyperRenderer) {
    // Pass 1 — page seams
    renderer.ui.page_seams.rebuild_from_page_tree(&ws.page_tree, pages_area);

    // Pass 2 — pod seams, one per page
    renderer.ui.pod_seams.clear();
    for (page_id, page_rect) in ws.page_tree.leaf_rects(pages_area) {
        let page = ws.page_tree.find(page_id).unwrap();
        let content_rect = page.content_rect(page_rect);  // subtracts header + rail
        renderer.ui.pod_seams.rebuild_from_pods(&page.pod_tree, content_rect);
    }
}
```

`UiRenderer` gains two seam collections:

```rust
pub struct UiRenderer {
    pub rects:      NodePipeline,
    pub focus_ring: NodePipeline,
    pub tree:       ParticleTree,
    pub input:      InputRouter,
    // REPLACE: pub seams: SeamRenderer, pub pods: PodTree,
    // WITH:
    pub page_seams: SeamRenderer,   // page boundaries — support split/merge
    pub pod_seams:  SeamRenderer,   // pod boundaries within pages — ratio drag only
}
```

### `window_event.rs` — handle `PageSeamRightClick`

The event dispatch in `window_event.rs` is extended:

```rust
UiEvent::PageSeamRightClick { seam_id, cursor, direction } => {
    shell.pending_context_menu = Some(PageContextMenu { seam_id, cursor, direction });
    window.request_redraw();
}
```

`AppShell` gains:

```rust
pub pending_context_menu: Option<PageContextMenu>,
```

On redraw, if `pending_context_menu` is `Some`, the shell builds and
renders a context menu particle popup positioned at `cursor`. Clicking
any item fires the corresponding `PageSignal` and clears
`pending_context_menu`.

---

## Icon rail — application assembly

The icon rail is a `StackParticle(column)` of `TriggerParticle` items,
one per pod leaf in the page's `PodTree`. It is built by a shared
function in the application layer:

```rust
pub fn build_icon_rail(
    page: &PageNode,
    pod_icons: &[(u32, &'static str)],  // (leaf_id, glyph)
) -> Particle {
    // ... returns Surface → Stack(column) → [Trigger(icon), ...]
}
```

Each `TriggerParticle` fires a `UiEvent::TriggerFired` mapped to
`PageSignal::ScrollToPod { page_id, pod_leaf_id }`. On the next frame the
workspace scrolls (or jumps) the page content to bring that pod's rect
into view.

Icons enlarge on hover through the existing `TriggerState::Hover`
mechanism — the trigger's layout size is set to the hover size in the
particle tree when hover state changes. Neighboring icons at ~80% of the
hovered size are achieved by rebuilding the rail particle subtree on hover
state change, which the dirty flag system handles without a full rebuild.

---

## Build order

```
Step 1 — hyper-ui: PageId, PageNode, PageHeaderConfig, IconRailConfig
Step 2 — hyper-ui: PageTree enum + leaf_rects(), split_rect traversal
Step 3 — hyper-ui: PageTree mutations — split(), merge(), set_ratio()
Step 4 — hyper-ui: SeamDrawCmd — add is_page_seam, page_seam_id fields
Step 5 — hyper-ui: SeamRenderer::handle_event — right-click → PageSeamRightClick UiEvent
Step 6 — hyper-ui: UiRenderer — replace seams/pods with page_seams/pod_seams
Step 7 — src: delete pages_pod_tree.rs, delete sync_page_layouts.rs
Step 8 — src: add templates/ — navigation_page, analysis_page, results_page
Step 9 — src: AnalysisWorkspace — replace pod_tree with page_tree + page_ios
Step 10 — src: sync_from_page_tree.rs — walk PageTree, assign IO rects
Step 11 — src: rebuild_seams.rs — two-pass (page seams + pod seams)
Step 12 — src: window_event.rs — handle PageSeamRightClick, PageSignal dispatch
Step 13 — src: context menu particle — build, render, dismiss
Step 14 — src: build_icon_rail() — per-page, left/right configurable
Step 15 — src: analysis page header — live ratio display wired to AnalysisComplete
```

Steps 1–3 and 7–9 can proceed in parallel in separate Cursor sessions
(no shared mutable state until Step 10 connects them).

---

## What does not change

- `PodTree` struct — unchanged, just demoted to the pod level.
- `SeamDirection`, `split_rect`, `SeamDrawCmd` (base fields) — unchanged.
- `rebuild_seams` internal logic — unchanged, called twice now.
- `SeamRenderer` drag mechanics — unchanged for both seam populations.
- The particle system, layout engine, dirty flags — untouched.
- All IO builders (`build_navigation`, `build_analysis`, `build_results`) — 
  their internals are unchanged; only how they are slotted into rects changes.
- `WorkspaceSignal` — unchanged.
- The existing double-click → reset 50/50 on pod seams — unchanged.

---

## Acceptance criteria

- [x] Three pages tile the workspace as before — visually identical to current.
- [x] Dragging a page seam resizes pages live.
- [x] Right-clicking a page seam shows the context menu (split V, split H,
      merge ←/→ or ↑/↓, reset).
- [x] Split via context menu inserts a new empty page at 50/50.
- [x] Split via the header icon also works.
- [x] Merge removes one page cleanly; surviving page fills the freed space.
- [x] Pod seams within each page still drag to resize pods — unchanged.
- [x] Icon rail renders on the configured side (left or right) per page.
- [x] Clicking an icon in the rail jumps to that pod.
- [x] Icon hover-magnify works via existing TriggerState::Hover.
- [x] Analysis page header shows live result ratios after AnalysisComplete.
- [x] Window resize recomputes all rects correctly without unioning hacks.
- [x] `pages_pod_tree()` and `sync_page_layouts()` are fully deleted.
