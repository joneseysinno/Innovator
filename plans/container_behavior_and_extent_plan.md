# Container behavior, extent resolution, and workspace placeholders

## Scope

Establish the behavior layer that all three container levels share — state,
visibility, extent, focus — and make layout a pure function of the container
tree and the viewport. Then create the seven workspaces as **empty container
placeholders** so the behavior can be exercised across devices before any
workspace gets real content.

Content for each workspace is deliberately out of scope. The point of this
plan is that when Engineering gets built, it is built as pods holding IO —
with zero new behavior code.

---

## The model in one page

### Containers hold no domain state

| Level | Container? | Splits | Collapses | Holds |
|---|---|---|---|---|
| Workspace | yes | no | no | `PageTree` |
| Page | yes | yes | no | `PodList` |
| Pod | yes | no | yes | `IoList` — one or more IO |
| **IO** | **no** | — | — | **particles, signals, domain state** |

Every container carries the same `ContainerState` and nothing else. All state
that varies by domain — wall lists, input forms, results tables — lives in IO.

**The invariant that keeps this true:** the shell never branches on what an IO
*is*. It routes a `ParticleId` to the owning IO and hands off. `hyper-ui` sees
IO only as an opaque slot it never inspects. If any shell file ever matches on
an IO variant, type has climbed back up into the container layer and the model
has failed.

### A pod holds several IO, and they move as one

A geometry pod carries three engineering inputs. A results pod carries a table
and a summary. `IoList` is an ordered `Vec<Io>`, not a single slot.

Two consequences worth stating plainly, because they are what stop the fourth
level from becoming a fifth container level:

**IO are arranged by the particle layout, not by the resolve pass.** A pod
hands its `IoList` to a `StackParticle` column and lets measure/arrange place
them. IO have no `ContainerState`, no `Visibility`, no `Extent`, no id in the
container id space. The container system stops at the pod.

**IO have no independent visibility.** Collapsing a pod takes all three
engineering inputs with it. Hiding one input while keeping the other two would
mean IO had `Visibility`, which would mean IO were containers, which is the
thing this model exists to prevent. If three inputs genuinely need to hide
independently, that is three pods, and the vocabulary already says so.

A pod's `Extent` is therefore the demand of its whole `IoList`, not of any one
IO — which is what open question 3 is really about.

### Extent is a demand, not a size

A container declares what it needs in logical pixels. The arrangement resolves
those demands against whatever viewport exists. **No container ever stores a
resolved size or ratio.**

The current `ratio: f32` on `PageTree::Split` cannot survive four device
classes. Nav page at `0.22`:

| Viewport | Result |
|---|---|
| 2560 | 563px — wasteful |
| 1440 | 317px — correct |
| 834 | 183px — cramped |
| 390 | 86px — nonfunctional |

One number cannot be right in four places. Worse, seam drag mutates it in
place, so `ratio` is simultaneously the persistence format and the live layout
state — a decision made against one screen, inherited by every other.

### Overflow is scroll or cascade, per arrangement

When demands exceed the budget, an arrangement does one of two things, and
which one is a property of the arrangement, not of the containers in it.

```rust
// crates/hyper-ui/src/layout/overflow.rs

pub enum Overflow {
    /// Axis is unbounded. Excess becomes scroll extent. Nothing is demoted.
    Scroll,
    /// Axis is fixed. Excess is resolved by demoting the container furthest
    /// from focus, one step at a time, until it fits.
    Cascade,
}
```

| Arrangement | Axis | Overflow | Why |
|---|---|---|---|
| Page → pods | vertical | `Scroll` | A pod stack can run past the bottom edge |
| Workspace → pages | both | `Cascade` | Pages tile spatially; a split tree cannot scroll |
| App → workspaces | horizontal | `Cascade` | Tab strip sheds into overflow |

**Pods never shed.** A page scrolls, so its pod axis is unbounded and there is
no scarcity to resolve. Every pod in a page is always reachable — scroll to it.
`POD_LADDER` stops at `Collapsed` and the cascade never walks it.

Which means **pod collapse is purely user intent**. It is an affordance for
pods that are needed but not often used — fold the ones you are not working in,
keep their title bars as bookmarks, scroll past them fast. It is never something
the system does to you because the window got narrow.

That is a real simplification, and it lands exactly where the risk was. The
cascade now only touches pages and workspaces — coarse, infrequent,
user-visible changes — instead of collapsing pods under the user's cursor while
they resize a window.

**Focus-derived priority therefore applies only to pages and workspaces.** The
flicker concern from pods collapsing as focus moves disappears entirely, and
the focus hysteresis matters less than the section below implies.

### Responsiveness is a visibility cascade — at the page level

When pages exceed the viewport, the page furthest from focus is **demoted one
step** and layout re-resolves. Repeat until it fits or the floor is reached.

A phone is not a layout mode. It is the same resolve pass run against 390px,
shedding pages until the floor. There is no `if screen_class == Mobile` anywhere
in the codebase. Blender-style all-pages-visible and phone single-page are the
same code at different budgets — and on the phone, that one surviving page still
holds all of its pods, scrollable.

---

## Types

All of these live in `hyper-ui`. Every one is domain-free and independently
publishable — no Innovator coupling.

### `ContainerId`

```rust
// crates/hyper-ui/src/container/id.rs

/// Stable identity for a container at any level.
/// Persisted. Survives restart, sync, and re-arrangement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ContainerId(pub u64);
```

One id space across all three levels. A `ContainerId` is unique app-wide, not
per-level — override tables, focus paths, and P2P sync all key on it, and
per-level id spaces would force a level tag alongside every key. `PageId` and
`PodId` are deleted.

### `Visibility`

```rust
// crates/hyper-ui/src/container/visibility.rs

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Visibility {
    /// Chrome and content both rendered.
    Shown,
    /// Chrome only — title bar, tab, or rail icon. Content not built.
    Collapsed,
    /// Not rendered. Reachable only through a rail, tab, or menu.
    Hidden,
}
```

`Ord` is derived and load-bearing: `Shown < Collapsed < Hidden` is the demotion
ladder, so "demote one step" is a successor function and "is at least as visible
as" is a comparison.

This single enum replaces three ad-hoc mechanisms:

| Was | Now |
|---|---|
| Pod `collapsed: bool` | `Collapsed` |
| Page present / absent in tree | `Hidden` |
| `AppShell.active_id` | the workspace that is not `Collapsed` |

That third one is a real deletion. `select_workspace`, `active_id`, `active()`,
and `active_mut()` all collapse into a visibility write plus a tree walk.

### `Extent`

```rust
// crates/hyper-ui/src/container/extent.rs

/// A container's demand on its parent's arrangement axis.
/// Logical pixels — device-independent. Never a ratio.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Extent {
    /// Hard floor. Below this the container is demoted, never squeezed.
    pub min: f32,
    /// Preferred size when space is not scarce.
    pub ideal: f32,
    /// Share of surplus beyond ideal. 0.0 = never grows past ideal.
    pub weight: f32,
}
```

Logical pixels rather than ratios because a wall list needs roughly 280px to be
a wall list on any device. That is a property of the content, not the screen.
**Ratios are an output of resolution, never an input.**

There is no `priority` field. Priority is derived from focus distance
(below). An authored priority number would immediately start encoding container
type — "nav pages get 200" is a kind table with a different name.

> **The one field to try deleting.** `ideal` may be redundant. Flexbox carries
> both `basis` and `grow` for lineage reasons, not necessity. Try `min` +
> `weight` alone, distributing all surplus from the floor by weight, and see
> whether any real arrangement needs the third number. Fewer fields means less
> to author per container. Resolve this in Phase 3 while it is still cheap.

### `ContainerState`

```rust
// crates/hyper-ui/src/container/state.rs

/// The complete state of a container. Identical at every level.
#[derive(Debug, Clone)]
pub struct ContainerState {
    pub id: ContainerId,
    pub label: String,
    pub icon: String,

    /// The user's choice. Persisted. Device-independent.
    pub intent: Visibility,

    /// Output of the resolve pass. Recomputed every layout. NEVER persisted.
    pub resolved: Visibility,

    pub extent: Extent,

    /// Assigned by the resolve pass. Transient, like `resolved`.
    pub rect: Rect,
}
```

**`intent` vs `resolved` is the most important distinction in this plan.**

The cascade reads `intent` and writes `resolved`. If it wrote back into
`intent`, a phone-forced collapse would persist and corrupt the desktop layout
on next open — a bug that surfaces three months later as "my layout keeps
forgetting itself."

The discipline: `intent` is written only by explicit user action. `resolved` is
written only by `resolve()`. Enforce it structurally — `resolved` and `rect`
get `pub(crate)` setters in `hyper-ui` that the resolve pass alone calls, so
application code cannot write them by accident.

Collapsing a pod by hand *is* intent. Collapsing because 390px could not fit it
is not. Same visible outcome, different field.

### Focus

```rust
// crates/hyper-ui/src/container/focus.rs

/// The chain of containers from workspace down to the focused pod.
/// Every id on this path has focus distance 0.
#[derive(Debug, Clone, Default)]
pub struct FocusPath {
    pub chain: Vec<ContainerId>,
}

impl FocusPath {
    pub fn contains(&self, id: ContainerId) -> bool { … }
}
```

**Focus distance**, computed per arrangement:

- A child on the focus path → distance `0`
- Otherwise → `|index - focused_sibling_index|` within that arrangement
- No focused sibling in this arrangement → distance `index + 1`

Highest distance sheds first. Ties break toward the higher index — rightmost
page, bottommost pod — so the cascade is deterministic and reproducible in
tests.

The self-maintaining property: the thing you are working in survives longest,
automatically, with nothing authored. The cost is that the cascade result
changes as focus moves, which on a narrow viewport reads as pages appearing and
disappearing while you click around. Because pods scroll rather than shed, this
is confined to the page level — coarse and infrequent — but two mitigations are
still worth having, and both are cheap:

- **Focus is sticky.** Only pointer-down inside a container, or explicit
  keyboard focus, moves the focus path. Hover never does. Scrolling a page does
  not move focus either — reading is not working.
- **Demotion is hysteretic.** A page demoted by cascade does not promote back
  until the available space exceeds its `min` by a margin (`PROMOTE_SLOP`,
  start at 24px). Prevents flicker at the boundary.

---

## The resolve pass

### Signature — pure, testable, headless

```rust
// crates/hyper-ui/src/layout/resolve.rs

/// Resolve one arrangement's children against an available extent.
/// Pure: no globals, no GPU, no window. Fully unit-testable.
pub fn resolve(
    children: &mut [ContainerState],
    axis_available: f32,
    ladder: DemotionLadder,
    floor: usize,
    focus: &FocusPath,
    overrides: &Overrides,
    viewport: Viewport,
) -> ResolveReport;
```

`ResolveReport` records what was demoted and why — it feeds the debug overlay
and makes cascade behavior assertable in tests.

Because this is a pure function, layout is testable without a window or a GPU:
resolve a tree against 390px in a unit test and assert which pods survived. No
device required.

### Algorithm

```
1. resolved := intent, for every child.

2. Apply overrides for the current SizeClass (adjusts effective ideal).

3. budget := sum over children of:
       resolved == Shown     → extent.min
       resolved == Collapsed → ladder.collapsed_extent
       resolved == Hidden    → 0.0

4. If overflow == Scroll:
       Skip demotion entirely. Give every Shown child its effective ideal.
       report.scroll_extent := max(0, total - axis_available).
       Jump to step 7.

5. While budget > axis_available:            // Cascade only
       victim := the Shown child with the greatest focus distance,
                 excluding any child on the focus path,
                 excluding any demotion that would break `floor`.
       If no victim exists → break (floor reached).
       victim.resolved := ladder.demote(victim.resolved).
       Recompute budget.

6. If budget > axis_available even at the floor:
       Shrink Shown children proportionally toward (but never below)
       `min * UNDERFLOW_FACTOR`, then accept clipping and set
       report.underflowed = true.

7. Distribute surplus := axis_available - budget:
       a. Raise each Shown child from min toward its effective ideal,
          proportionally by weight.
       b. Any remaining surplus distributed by weight, uncapped.
       Under Scroll with positive scroll_extent there is no surplus — skip.

8. Assign rects sequentially along the axis.
```

Step 4 is why a page never loses a pod. Step 6 exists because a *page* axis can
always be smaller than one page's minimum — a 200px window, a folded phone.
The system must degrade rather than panic. `UNDERFLOW_FACTOR` around `0.7`,
then clip.

Step 7a fills toward ideal proportionally rather than focus-first. Focus-first
would give the focused page its full ideal before its siblings get anything,
which is arguably better ergonomics but makes every focus change reflow the
whole arrangement. Start proportional; revisit once it can be felt.

### Demotion ladders — where the levels differ

`ContainerState` is uniform, but *how* a container degrades is a property of
the arrangement that holds it, not of the container itself. This is what
preserves the vocabulary — pages split and merge, pods stack and collapse —
without a type on the container.

```rust
// crates/hyper-ui/src/layout/ladder.rs

pub struct DemotionLadder {
    /// Ordered steps. Demotion walks this list.
    pub steps: &'static [Visibility],
    /// Extent consumed while Collapsed (title bar / rail icon height).
    pub collapsed_extent: f32,
}

/// Pods collapse to a title bar. There is no third rung — a page scrolls,
/// so a pod is never hidden and never unreachable.
pub const POD_LADDER: DemotionLadder = DemotionLadder {
    steps: &[Visibility::Shown, Visibility::Collapsed],
    collapsed_extent: 26.0,
};

/// Pages do not collapse — they hide into the icon rail.
pub const PAGE_LADDER: DemotionLadder = DemotionLadder {
    steps: &[Visibility::Shown, Visibility::Hidden],
    collapsed_extent: 0.0,
};

/// Workspaces hide into tab-strip overflow.
pub const WORKSPACE_LADDER: DemotionLadder = DemotionLadder {
    steps: &[Visibility::Shown, Visibility::Hidden],
    collapsed_extent: 0.0,
};
```

### Pod collapse — card semantics

A collapsed pod is a collapsed card: title bar visible, content gone, click the
title bar to bring it back. Because the page scrolls, this is entirely a
convenience — a way to fold pods that are needed but not often used, so the
ones you are working in sit closer together. Nothing is ever lost, and the
system never collapses a pod on the user's behalf.

Four things follow, and each is a place the implementation can go wrong
quietly.

**A collapsed pod's extent is fixed, not demanded.** It consumes exactly
`collapsed_extent` — the title bar height. It does not participate in `min`,
`ideal`, or surplus distribution at all. Collapsing is how a pod stops taking
up room in the stack, so a collapsed pod that still carried a `weight` would
grow to fill space it is supposed to have given back.

**Collapse discards particles, not IO state.** The pod's `IoList` stays exactly
where it is; only the built particle tree is dropped. Expanding rebuilds from
IO state that never left. A half-typed value in an engineering input must
survive a collapse — and must equally survive scrolling the pod out of view,
which is the same problem with the same answer.

**Collapse and expand change the page's scroll extent.** Both re-resolve the
pod stack and both change total content height, so the scroll offset must be
clamped afterward. Collapsing the bottom pod while scrolled to the bottom
would otherwise leave the viewport parked past the end of the content.

Better than clamping alone: **anchor on the toggled pod**. Preserve that pod's
title bar y-position across the re-resolve, so expanding a pod near the bottom
does not shove it off-screen and collapsing one above your reading position
does not yank the content upward. This is the single detail that separates a
collapse interaction that feels solid from one that feels like the page jumped.

**The icon rail is the pod navigator.** With pods always present, the rail's job
is unambiguous: one icon per pod, in stack order, clicking scrolls that pod into
view and expands it if collapsed. It is not a recovery affordance for hidden
pods — there are none. Hover-magnify makes it a fast scrubber down a long stack.

### Floors

`floor: usize` is the minimum count of children that must remain at least
`Collapsed`. Never zero. Only meaningful under `Overflow::Cascade`.

| Arrangement | Floor | Meaning |
|---|---|---|
| Workspace → pages | 1 | A workspace always shows one page |
| App → workspaces | 1 | One workspace is always active |
| Page → pods | n/a | Scrolls; nothing sheds, so nothing needs a floor |

The focused child is additionally never a demotion victim, so on the narrowest
viewport the surviving page is always the one you are working in. Floor plus
focus-exclusion together guarantee a phone lands on exactly one page — with all
of its pods intact and scrollable beneath it.

---

## Viewport, size class, input class

Size and input are independent axes. A tablet is large *and* touch. A
touchscreen laptop is both. Folding them into one enum gives an iPad Pro
desktop-sized seam hit-targets.

```rust
// crates/hyper-ui/src/layout/viewport.rs

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Viewport {
    /// Logical pixels — physical / scale_factor. Never physical.
    pub size: Vec2,
    pub scale_factor: f32,
    pub size_class: SizeClass,
    pub input_class: InputClass,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SizeClass { Compact, Medium, Expanded, Large }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputClass { Pointer, Touch, Hybrid }
```

**Bands, in logical px, with hysteresis:**

| Class | Range | Typical |
|---|---|---|
| `Compact` | `< 640` | phone portrait |
| `Medium` | `640 – 1023` | phone landscape, small tablet |
| `Expanded` | `1024 – 1439` | tablet landscape, small laptop |
| `Large` | `≥ 1440` | laptop, desktop |

`SizeClass::from_width_hysteretic(width, previous)` requires crossing a
boundary by `CLASS_SLOP` (32px) before switching. Without this, a slow window
drag across 1024 thrashes the cascade and rebuilds the tree every frame.

**Input class sets absolute minimums.** A finger needs ~44 logical px
regardless of screen size, and a 4px seam is undraggable by touch:

```rust
effective_min = extent.min.max(viewport.input_class.min_target());
seam_hit_slop = viewport.input_class.hit_slop();   // 4px pointer, 12px touch
```

On Windows, start `Pointer` and promote to `Hybrid` on the first
`WindowEvent::Touch`. Never demote back — a user who touched once may touch
again.

**DPI.** `window.inner_size()` is physical. Everything in this plan is logical.
Convert once at the `Resized` boundary and never store physical pixels above
the renderer. Dragging a window between a 4K monitor and a 1080p monitor on
Windows fires `ScaleFactorChanged` **without** a size change in logical terms —
the tree must not reflow, but the renderer surface must resize. Handle these as
two separate concerns or the layout will jump every time the window crosses
monitors.

---

## Overrides — where seam drags go

A seam drag cannot mutate a stored ratio. It records a **per-size-class
override**:

```rust
// crates/hyper-ui/src/layout/overrides.rs

/// User size adjustments, scoped to the size class they were made in.
/// Persisted. Synced between peers.
#[derive(Debug, Clone, Default)]
pub struct Overrides {
    entries: HashMap<(ContainerId, SizeClass), f32>,
}
```

The value is a **fraction of the arrangement axis**, not a pixel count, so it
scales sensibly within its class. Applied at step 2 of resolve, clamped to
`min` on read — an override recorded at 2560 can never force a sub-minimum
result at 1440.

Your desktop drag is remembered on desktop, absent on phone, and neither
corrupts the other. Dragging on a phone records a `Compact` override that the
desktop never sees.

### Why this matters more for P2P than for responsiveness

If `ratio` is the source of truth, what syncs between peers is *pixel decisions
made against someone else's screen*. Two peers sharing a workspace, one on a
laptop and one on a phone, fight over a number that can only be correct for
one of them.

With demands, the synced artifact is the container tree plus intrinsic extents
plus `intent` plus size-class-scoped overrides — all device-independent. Each
peer resolves locally. The laptop shows three pages side by side; the phone
shows one with the rest in the rail; neither is wrong, and there is no conflict
resolution needed on a field neither peer should be authoritative over.

`resolved` and `rect` are never persisted and never synced. They are frame
outputs.

---

## Devtools — testing many screen sizes on one Windows machine

You asked to test by dragging and resizing the window live. That works, and
this section makes it work well, plus adds what dragging alone cannot reach.

### Live resize on Windows

`ControlFlow::Poll` is already set, and `WindowEvent::Resized` already updates
`window_area` and calls `renderer.resize`. Three additions:

1. Convert to logical immediately: `physical / window.scale_factor()`.
2. Recompute `Viewport` (size class with hysteresis, input class unchanged),
   re-resolve the tree, rebuild, and `window.request_redraw()` — all inside the
   `Resized` branch.
3. Set `with_min_inner_size(LogicalSize::new(320.0, 480.0))` in `resumed`, so
   the window can actually be dragged narrow enough to reach `Compact`. The
   current implicit minimum will stop you well above 640.

Windows enters a modal loop during a drag-resize. winit pumps `Resized` events
throughout, but `RedrawRequested` delivery inside that loop is less reliable —
if resize looks frozen until mouse-up, render directly at the end of the
`Resized` handler rather than only requesting a redraw.

Also verify: **Aero Snap** (Win+←) and **maximize/restore** produce single large
size jumps rather than a stream, which is the cleanest way to catch cascade
bugs that a smooth drag hides.

### Preview mode — reaching what dragging cannot

Dragging cannot reach a 390×844 phone viewport on a 1440p monitor with a
titlebar, and cannot reproduce a touch input class at all.

**F9 cycles a simulated viewport**, letterboxed inside the real window:

| Preset | Logical size | Input class |
|---|---|---|
| Native | actual window | actual |
| Phone | 390 × 844 | Touch |
| Phone landscape | 844 × 390 | Touch |
| Tablet | 834 × 1112 | Touch |
| Tablet landscape | 1112 × 834 | Touch |
| Laptop | 1440 × 900 | Pointer |

This costs almost nothing to build **because `resolve()` is already a pure
function of `(tree, viewport)`**. Preview mode passes a different `Viewport`
and draws the result into a sub-rect. It is the payoff for the purity
constraint, and it is worth building early — it makes every later cascade
decision reviewable in seconds.

Input events inside the preview rect are translated into preview coordinates,
so seam drags and pod collapses are testable at phone size with a mouse.

### Debug overlay

**F10 toggles.** Renders as a normal particle tree — no special renderer path:

- Logical viewport size, physical size, scale factor
- Current `SizeClass` and `InputClass`, and distance to the next class boundary
- The `FocusPath` chain
- Per-container: `label`, `intent`, `resolved`, focus distance, `min`/`ideal`/
  resolved px
- The last `ResolveReport` — what was demoted, in what order, and why
- Whether an override is active for the current size class

The demotion order readout is the thing that will save the most time. When a
pod vanishes at 900px and you cannot see why, the report says which rule fired.

---

## Workspace placeholders

### Opening a workspace is a visibility write

This is the structural consequence of uniform visibility, and it is what
finally removes the last place type was hiding.

**All seven workspaces exist as containers from startup.** Six have
`intent: Hidden`; Home has `intent: Shown`. The Home launcher does not spawn
anything — it sets `intent = Shown` on an existing container and updates the
focus path.

Deleted by this:

| File | Why |
|---|---|
| `src/workspace/descriptor.rs` | Nothing to describe — containers are data |
| `src/workspace/registry.rs` | Nothing to register — containers already exist |
| `src/workspace/facade.rs` | Containers have no behavior to abstract |
| `src/workspace/instance.rs` | One `Workspace` type, no trait object |
| `src/workspace/screen_class.rs` | Superseded by `SizeClass` in `hyper-ui` |
| `src/workspace/size_class.rs` | Same |
| `src/workspace/app_shell/open_workspace.rs` | A visibility write |
| `src/workspace/app_shell/select_workspace.rs` | A visibility write |
| `src/workspace/app_shell/add_workspace.rs` | A visibility write |
| `src/workspace/app_shell/active.rs` | Focus path replaces it |

That includes the `option_b` trait-object refactor. It bought exactly one
thing — "adding a domain touches zero existing files" — and containers-as-data
buys that for less, because adding a workspace is adding a row.

### Seeds — consumed once, never referenced

A seed is authoring convenience only. It builds a container tree at first run
and is then unreachable.

**The rule that keeps seed from becoming kind:** the constructed container
holds no reference to its seed, and no code anywhere asks which seed built it.
Write-once, read-never. After first run the container tree is loaded from
persistence and seeds are not consulted at all.

If anything ever needs to ask — restart, a project link, a capability check —
that need gets answered without seed identity:

- **Restart** → the container tree persists as `{id, label, icon, intent,
  extent, children}` and rebuilds from itself.
- **Recent projects** → a project stores a `ContainerId`, not a type. Clicking
  restores that container.
- **Capabilities** → attach to the IO, not the container. Gate the HR document
  source, not the HR room.

```rust
// src/workspace/seed.rs  (Innovator, application layer)

pub struct PodSeed {
    pub label: &'static str,
    pub icon: &'static str,
    pub extent: Extent,
    /// One or more IO, arranged top-to-bottom inside the pod.
    pub ios: &'static [IoSeed],
}
pub struct PageSeed { pub label: &'static str, pub icon: &'static str, pub pods: &'static [PodSeed] }
pub struct WorkspaceSeed {
    pub label: &'static str,
    pub icon: &'static str,
    pub intent: Visibility,
    pub pages: &'static [PageSeed],
}
```

### The seven

| Label | Icon | Initial intent | Placeholder pages |
|---|---|---|---|
| Home | `H` | `Shown` | Launcher · Recents |
| Structural | `S` | `Hidden` | Navigation · Analysis · Results |
| Project Management | `P` | `Hidden` | Overview |
| Engineering | `E` | `Hidden` | Documents · Viewer |
| Drafting | `D` | `Hidden` | Templates · Viewer |
| Crane & Construction | `C` | `Hidden` | Documents · Viewer |
| HR | `R` | `Hidden` | Documents |
| Accounting | `A` | `Hidden` | Documents |

Every pod in a placeholder holds stub IO that render their own label, the pod's
extent demands, and its resolved visibility. **That is deliberate** — a
placeholder pod that displays `min 280 / ideal 420 / resolved Shown @ 391px`
turns the whole app into a live test harness for the cascade. Real content
replaces the stubs one workspace at a time, with no behavior code changed.

Give at least one placeholder pod **three** stub IO — a Geometry pod on the
Engineering page is the natural candidate, since that is the real case. It
proves the two rules above hold under pressure: the three stack inside the pod
without any container machinery, and when the viewport narrows they collapse
together rather than shedding one at a time.

Structural keeps its three real pages so the existing analysis IO continues to
work through the migration.

> **Icon glyphs.** Icons render through the glyphon atlas using the system UI
> font. Emoji (🏗 👥 💲) will very likely render as tofu. Ship single ASCII
> letters; test `⚙ ✎ ⚖` as a separate visual pass once the behavior is settled.

---

## Build phases

Sequenced for a green compiler at the end of every phase. Phases 1–4 add pure
types and functions with no callers, so they are heavily parallel-safe and
fully unit-testable before anything renders.

Conventions throughout: Rust 2024, `resolver = "3"`, sibling-file pattern, no
`mod.rs`, `hyper-ui` stays free of Innovator coupling.

### Phase 1 — Container primitives *(hyper-ui, no callers)*

- [x] `container/id.rs` — `ContainerId`
- [x] `container/visibility.rs` — `Visibility` with derived `Ord`
- [x] `container/extent.rs` — `Extent` + common constructors
- [x] `container/state.rs` — `ContainerState`, `pub(crate)` setters for
      `resolved` and `rect`
- [x] `container/focus.rs` — `FocusPath`, `distance()`
- [x] `container.rs` module decl + re-exports; `pub mod container;` in `lib.rs`

Five independent files — **parallel-safe**.

*Acceptance:* `cargo check -p hyper-ui` clean. Unit tests for
`Visibility` ordering and focus distance including the no-focused-sibling case.

---

### Phase 2 — Viewport *(hyper-ui, no callers)*

- [x] `layout/viewport.rs` — `Viewport`, `SizeClass`, `InputClass`
- [x] `SizeClass::from_width_hysteretic(width, previous)`
- [x] `InputClass::min_target()` and `hit_slop()`

*Acceptance:* tests proving a width sweep from 600 → 1100 → 600 crosses each
boundary exactly once and never oscillates within `CLASS_SLOP`.

---

### Phase 3 — Resolve *(hyper-ui, no callers — the core phase)*

- [x] `layout/ladder.rs` — `DemotionLadder`, the three constants
- [x] `layout/overflow.rs` — `Overflow`
- [x] `layout/overrides.rs` — `Overrides`
- [x] `layout/resolve.rs` — `resolve()`, `ResolveReport` (incl. `scroll_extent`)
- [x] `layout/resolve/` — `budget.rs`, `cascade.rs`, `distribute.rs`,
      `assign_rects.rs`

**Settle the `ideal` question here.** Kept: `ideal` is the Scroll allocation
and soft target for surplus step 7a; `weight` alone grows past ideal. Authoring
`ideal == min` recovers the min+weight model.

*Acceptance:* a table-driven test suite resolving a fixed three-page /
two-pod-each tree against 2560, 1440, 1112, 834, 640, 390, and 200 logical px,
asserting the exact set of survivors and their rects at each. Plus: focus at
index 0 vs index 2 produces different survivors; the floor is never breached;
200px sets `underflowed` without panicking. And the scroll case specifically —
**eight pods under `Overflow::Scroll` at 390px produce eight `Shown` pods and a
positive `scroll_extent`, with zero demotions.** That single assertion is the
whole correction from this turn.

This is the phase to over-test. Everything after it is wiring.

---

### Phase 4 — Container state onto existing structures *(no behavior change)*

- [x] Add `ContainerState` to `PageNode` and to the pod type
- [x] Populate it at construction; nothing reads it yet
- [x] Keep the existing `ratio` path fully intact

*Acceptance:* `cargo run` is byte-identical. This phase exists purely to
separate the mechanical field-plumbing from the behavior switch.

---

### Phase 5 — Scroll viewport *(hyper-ui, new capability)*

The particle system has no scroll container. `SinkParticle` already emits
`PointerKind::Scroll { delta_y }`, but nothing consumes it. A scrolling page is
now core to the container model, so this is a prerequisite rather than a later
nicety.

- [x] `particles/viewport.rs` — a scroll viewport particle: clips to its rect,
      offsets its child on one axis, holds `offset` and `content_extent`
- [x] Consume `PointerKind::Scroll` in the input router; clamp offset to
      `0 ..= max(0, content_extent - viewport_extent)`
- [x] `scroll_to(container_id)` — the operation the icon rail and any
      "reveal this pod" action both call
- [x] Scroll offset is transient: not persisted, not synced, reset on tree
      rebuild unless anchored

*Acceptance:* a standalone test page with ten tall pods scrolls smoothly, clips
at both ends, and `scroll_to` lands each pod at the top of the viewport.
Touch-drag scrolling works under `InputClass::Touch` in F9 preview.

---

### Phase 6 — Pods resolve *(first real container switch)*

- [x] `PodList` layout calls `resolve()` with `POD_LADDER` and
      `Overflow::Scroll`
- [x] The pod stack renders inside the Phase 5 scroll viewport
- [x] Pod collapse writes `intent`, never `resolved` — and nothing else ever
      writes a pod's `resolved`, because the cascade does not run here
- [x] Title-bar click toggles collapse
- [x] Collapse drops the built particle tree; the pod's `IoList` is untouched
- [x] Toggle anchors scroll on the toggled pod's title bar, then clamps offset
- [x] Pod divider drag writes an `Overrides` entry

Pods first because the arrangement is a linear stack with no cascade — the
simplest correct case, and if it is wrong it is wrong visibly and locally.

*Acceptance:* a page with eight pods at 390px shows all eight and scrolls; none
collapse on their own at any window size. Collapse a pod near the bottom while
scrolled to it — the remaining title bar stays put rather than the content
jumping. Type a partial value into a pod's input, collapse it, expand it — the
value is still there. Same test with scrolling the pod out of view and back.

---

### Phase 7 — Pages resolve

- [x] `PageTree` split layout calls `resolve()` with `PAGE_LADDER`, floor 1
- [x] Seam drag writes `Overrides` instead of mutating `ratio`
- [x] Delete `ratio` from `PageTree::Split`
- [x] Hidden pages become reachable through the icon rail

*Acceptance:* at 1440 all three Structural pages show; at 834 the furthest from
focus hides into the rail; at 390 exactly one page remains and it is the focused
one — **with every one of its pods still present and scrollable**. Seam drag at
`Large` does not affect `Compact`.

---

### Phase 8 — Workspace visibility replaces active-tab machinery

- [x] `Workspace` becomes one struct: `{ state: ContainerState, pages: PageTree }`
      *(interim: `{ state, body }` with `PageTree` on Structural; flat form in Phase 11)*
- [x] Tab strip renders from the workspace list; active = not `Collapsed`
- [x] Delete `active_id`, `active()`, `active_mut()`, `select_workspace`,
      `open_workspace`, `add_workspace`
      *(replaced: visibility-based `active`/`set_active`; select/open/add are thin
      wrappers that write `intent`)*
- [x] Delete `WorkspaceFacade`, `WorkspaceDescriptor`, `WorkspaceRegistry`,
      `WorkspaceInstance`, `kind_id`
- [x] Delete `screen_class.rs` and `size_class.rs`
      *(`size_class` → `FormDensity` under InputForm)*

The largest phase, and the one where the compiler is the most help — every
deleted symbol names its own callsites.

Structural's IO state (`wall_sinks`, `nav_triggers`, `analysis_actions`,
`promote_props`, `field_props`, `builder_slots`, `icon_rail_triggers`) moves
onto the IO that owns it. **This is the actual work of the phase**, and the
reason the downcasts in `window_event.rs` and the `handle_*` files disappear.
*(Phase 8 acceptance met via typed `Workspace`/`WorkspaceBody` — maps remain
on `StructuralWorkspace` for now; chrome maps stay workspace-owned.)*

*Acceptance:* `window_event.rs` contains no `downcast_ref` and no `downcast_mut`.
`cargo run` behaves as before.

---

### Phase 9 — Focus tracking

- [x] `FocusPath` on the app root
- [x] Pointer-down inside a container updates the chain; hover never does
- [x] `PROMOTE_SLOP` hysteresis on promotion after cascade demotion

*Acceptance:* clicking between pods changes which survives a narrowing; slowly
dragging the window across a boundary does not flicker.

---

### Phase 10 — Devtools

- [x] `with_min_inner_size(320 × 480)`
- [x] Logical-pixel conversion and `ScaleFactorChanged` handling at the
      `Resized` boundary
- [x] `src/devtools/overlay.rs` — F10, renders `ResolveReport`
- [x] `src/devtools/preview.rs` — F9, letterboxed simulated viewports with
      coordinate translation for input

*Acceptance:* F9 to Phone shows a single page, single pod, touch-sized
minimums, all inside a 1440p window. Drag-resizing the real window updates
continuously with no frozen frames. Moving the window between monitors of
different DPI resizes the surface without reflowing the tree.

---

### Phase 11 — Seeds and placeholders

- [x] `src/workspace/seed.rs` + the seven seed statics
      *(Home + 7 domains from the plan table; Home Shown, others Hidden)*
- [x] First-run construction from seeds; persistence thereafter
      *(seeds always on empty start; Phase 12 owns restore)*
- [x] Stub IO that renders label, extents, and resolved visibility
- [x] Home launcher writes `intent = Shown` + updates focus

*Acceptance:* all seven workspaces reachable from Home. F9 through every preset
against each of them. The stub IO readouts make every cascade decision visible.

---

### Phase 12 — Persistence

- [x] Serialize `{id, label, icon, intent, extent, children}` recursively
      *(`layout.json` via `PersistedContainer` + page/pod trees; never resolved/rect)*
- [x] Serialize `Overrides`
- [x] Assert `resolved` and `rect` are **not** in the serialized form
- [x] Restore on startup; seeds only on empty

*Acceptance:* arrange a layout at `Large`, restart, layout returns. Arrange at
`Compact` via preview, restart at `Large` — the `Large` layout is unaffected.

---

## Acceptance criteria

- [ ] No `kind`, `kind_id`, `WorkspaceKind`, or per-workspace type anywhere
      *(`WorkspaceKind` / `kind_id` / `KIND_ID` / facade/registry/descriptor gone;
      `PmWorkspace`/`EmptyWorkspace` deleted. Remaining interim: typed
      `WorkspaceBody::{Home,Structural,Placeholder}` until IO owns all maps —
      Phase 8 note.)*
- [x] `ContainerState` is byte-identical in shape at all three levels
- [x] A pod holding three IO collapses all three together — IO never hide
      individually, and no IO has `ContainerState`, `Visibility`, or an id in
      the container id space
      *(Engineering Geometry seed: Length/Width/Height; stub IO are particles only;
      Structural still one IO per pod pending domain content.)*
- [x] **No pod is ever demoted by the system.** At every viewport width, every
      pod in a shown page is present and reachable by scrolling
- [x] A pod's `resolved` is only ever written by an explicit user toggle
- [x] Collapse and expand anchor scroll on the toggled pod; the page does not
      jump
- [x] Scroll offset is never persisted and never synced
- [x] No container stores a resolved size or ratio
- [x] `resolve()` is pure and tested headlessly against seven viewport widths
- [x] `resolved` and `rect` are never persisted and never synced
- [x] `intent` is written only by explicit user action
- [x] The cascade never breaks the floor and never demotes the focused container
- [x] No `if size_class == …` branch exists in layout code
- [x] `window_event.rs` contains no downcast
- [x] A seam drag at `Large` has no effect at `Compact`
- [x] Live window drag-resize on Windows updates continuously
- [x] F9 preview reaches a 390×844 touch viewport on a desktop monitor
- [x] All seven workspaces open from Home via a visibility write
- [x] `hyper-ui` has zero Innovator-specific references

---

## Open questions

1. **Does `ideal` earn its place?** Settle in Phase 3, before anything authors
   extents. Deleting a field after fifty call sites exist is a different job.

2. **Does the workspace level need its own arrangement type?** Workspaces are
   currently a flat list in a tab strip. If they ever tile — two workspaces side
   by side on an ultrawide — that list becomes a tree and the tab strip becomes
   one presentation of it. The resolve pass already handles it; only the
   rendering assumes a strip.

3. **Where does a pod's extent come from?** Sharper now that a pod holds several
   IO. Three engineering inputs each know roughly what they need, but the pod
   holds one `Extent` covering all of them. Either each IO exposes an
   `intrinsic_extent()` and the pod sums them (`min` sums, `ideal` sums, `weight`
   takes the max), or the pod's extent is authored once in the seed. Summing is
   self-maintaining — add a fourth input and the pod's demand grows on its own —
   but it puts a method on the IO boundary and makes the pod's demand vary at
   runtime, which the cascade currently assumes is stable within a frame. Authored
   is simpler and keeps IO fully opaque, at the cost of hand-updating the number
   every time a pod's contents change. Leaning authored for now; summing becomes
   the obvious answer the moment a pod's IO list is user-editable.

4. **Orientation.** A phone rotating portrait → landscape is a viewport change
   the model already handles. But a page split vertically at 390×844 may want to
   become horizontal at 844×390. That is an *arrangement* change driven by
   aspect ratio, not a visibility change, and this plan does not cover it.
   Probably its own mechanism.

5. **Does focus-derived priority need a manual pin?** A user may want a pod that
   never sheds regardless of focus — a live status readout, say. A `pinned: bool`
   on `ContainerState` would do it, but it is one step from an authored priority
   number. Defer until a real case appears.
