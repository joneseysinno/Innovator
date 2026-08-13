# UX Pass 1 — Split Cursor, Pod Collapse, Page-Type Switcher

## Context

Three UX gaps were reported against the current page/pod interaction model:

1. Split-seam resize cursor sticks after activation (never resets).
2. Pods have no working expand/collapse despite the backend being fully
   built (`PodList::toggle`, sticky `Overrides` per `SizeClass`).
3. Pages have no Blender-style "editor type" switcher — `page_templates`
   is set once at split time and never changed by the user.

A fourth item — true 2D (horizontal *and* vertical) page splitting — was
also identified as broken, but it is **explicitly out of scope for this
pass**. `PageTree` is currently a flat `Vec<PageNode>` laid out on a single
fixed `Axis::Horizontal`, and `split_page`'s `direction` parameter is
discarded (`_direction`). Faking vertical splits inside the flat `Vec`
would be a wrong-abstraction patch that gets thrown away once container
nesting (`graph-backed-containers-plan.md`, Phases 2–3) lands, since real
nesting is what actually makes a recursive split tree possible. That work
is tracked separately and is a precondition for item 4, not a phase here.

Each phase below is independently shippable and the build stays green
after each one — no phase depends on a later phase's code existing.

---

## Phase 0 — Fix the stuck resize cursor

**Goal:** cursor reflects current hover/drag state, always, including the
"nothing is hovered" state.

**Root cause:** in `window_event.rs`:

```rust
let icon = renderer.ui.page_seams.cursor_icon()
    .or_else(|| renderer.ui.pod_dividers.cursor_icon());
if let Some(icon) = icon {
    window.set_cursor(icon);
}
```

`cursor_icon()` returns `None` when nothing is hovered/dragging, but the
`None` case never calls `window.set_cursor(CursorIcon::Default)`. Once a
resize cursor is set, nothing ever un-sets it.

**Changes:**

- `src/workspace/app_shell/window_event.rs`: replace `if let Some(icon)`
  with a full match/else that sets `CursorIcon::Default` on `None`.
- `crates/hyper-ui/examples/demo.rs`: same pattern exists there
  (`ui.pod_dividers.cursor_icon()` block) — fix identically so the demo
  doesn't silently regress this behavior later and mislead a future
  reader of the example.

**Compiles-green check:** manual — hover a seam, move away, confirm cursor
returns to default arrow. No automated test; this is a `winit` window-chrome
side effect, not covered by `hyper-ui`'s layout test harness.

**Risks / open decisions:** none. Fully isolated, no design decision.

**Parallelism:** none needed — do this first, it's a five-minute fix and
unblocks clean evaluation of everything else.

---

## Phase 1 — Wire pod collapse for Structural

**Goal:** title-bar click on a pod actually toggles collapse and persists
the sticky override, for every workspace kind, including Structural.

**Root cause:** two independent paths reach the page tree:

- `handle_page_signal.rs` (works): `shell.workspaces[idx].structural_mut()`
  → direct field access `ws.page_tree`.
- Pod-collapse handling (broken for Structural): generic accessor
  `shell.active_mut().and_then(|a| a.page_tree_mut())`, used inside the
  `pod_divider_events` block in `window_event.rs`.

The `TriggerFired` → `pod_collapse_triggers` → synthesized
`UiEvent::PodCollapse` path already fires correctly — the pod id is
correctly identified. The break is downstream: `page_tree_mut()` on the
workspace-kind enum either isn't implemented for the `Structural` arm or
doesn't resolve to the same `PageTree` that `structural_mut()` exposes.
This needs to be confirmed by opening the enum definition (likely
`src/workspace/app_shell/*.rs` or wherever the workspace-kind enum and its
`page_tree`/`page_tree_mut`/`structural`/`structural_mut` accessors live)
before writing the fix — the diagnosis above is high-confidence but not
yet a fix.

**Changes (pending the accessor audit):**

- Inspect the workspace-kind enum's `page_tree_mut()` implementation.
  Likely outcomes, in order of likelihood:
  1. Missing `Structural` match arm (falls through to `None`) → add it.
  2. Arm exists but reads a stale/wrong field → point it at the same
     `page_tree` field `structural_mut()` exposes.
- Once fixed, route pod-collapse handling through `structural_mut()`
  directly, matching the pattern `handle_page_signal.rs` already uses,
  rather than leaving two divergent access paths to the same data. This
  is the actual fix, not just patching the enum arm — divergent access
  paths to one source of truth is exactly the kind of drift that caused
  this bug, and it'll cause the next one too if left as-is.
- Confirm `toggle_pod_collapse` is called with the right `SizeClass` for
  the current viewport, since collapse overrides are scoped per
  `SizeClass` (`Overrides::set_collapse`) — a wrong size class here would
  toggle successfully but silently the override wouldn't stick on resize.

**Compiles-green check:**

- Existing unit tests in `crates/hyper-ui/src/pod/layout_tests.rs` already
  cover `PodList::toggle` / `resolved_collapse` / override persistence —
  those don't change and should stay green throughout.
- Add one integration-level check (or manual pass) confirming a title-bar
  click on a Structural-workspace pod collapses it, persists across a
  simulated resize within the same `SizeClass`, and expands correctly.

**Risks / open decisions:**

- If `page_tree_mut()` is used elsewhere for Structural in ways that
  currently *appear* to work, unifying the access path could surface a
  second latent bug there. Worth a quick grep for all `page_tree_mut()`
  call sites before touching it, not just the pod-collapse one.

**Parallelism:** independent of Phase 0 and Phase 2. Can run in parallel
with Phase 2 if convenient — no shared files.

---

## Phase 2 — Page-type switcher menu (Blender-style editor switch)

**Goal:** click a trigger in a page's header, get a dropdown of available
page templates (Navigation / Analysis / Results / Generic), pick one,
that page's `TemplateId` changes and it rebuilds with the new template's
body — same idea as Blender's area-type dropdown in the top-left corner
of an editor.

**What already exists (no new data model needed):**

- `page_templates()` in `src/pages/registry.rs` returns
  `HashMap<TemplateId, Box<dyn PageTemplate>>` for all four templates.
- `StructuralWorkspace.page_templates: HashMap<PageId, TemplateId>`
  already tracks the per-page assignment and is read every rebuild via
  `template_for()`.
- The menu-building pattern already exists and can be copied almost
  directly: `page_context_menu.rs` (`PageContextMenu` →
  `build_page_context_menu` → `HashMap<ParticleId, PageSignal>` trigger
  map) is structurally identical to what a template-switch menu needs.

**Changes:**

- New `PageSignal::SwitchTemplate { page_id: PageId, template_id: TemplateId }`
  variant in `src/domains/structural/page_signal.rs`.
- New menu builder, e.g. `src/workspace/app_shell/page_template_menu.rs`,
  modeled directly on `page_context_menu.rs`: one trigger per registered
  template (label = template display name), each mapped to
  `PageSignal::SwitchTemplate`.
- A trigger in the page header to open this menu. Candidate placement:
  next to the existing "⧉" split trigger in `build_split_only_header` /
  `build_analysis_page_header`, or a dedicated icon per-template (closer
  to Blender's editor-type icon, which doubles as the menu opener and a
  visual indicator of current type). Recommend the icon-as-opener version
  — it gives the user an at-a-glance answer to "what kind of page is
  this" without opening anything, which the current bare "⧉" button
  doesn't provide.
- Handler in `handle_page_signal.rs`: on `SwitchTemplate`, write
  `ws.page_templates.insert(page_id, template_id)`, clear/reset anything
  template-specific to the old template that shouldn't leak into the new
  one (check `pod_templates` entries scoped to that page — likely want to
  reset those to the new template's default pod layout rather than
  leaving stale pod assignments from the previous template type), then
  rebuild.

**Compiles-green check:** manual — open menu on each page, switch through
all four templates, confirm body rebuilds correctly and no stale
template-specific state (pod triggers, field maps) leaks across the
switch. Worth a quick look at whether `rebuild_active` already clears
per-page trigger maps on any rebuild, or whether template-switch needs to
explicitly clear them (stale `ParticleId` keys pointing at particles that
no longer exist would be a silent-no-op bug in the same family as the
Phase 1 one).

**Risks / open decisions:**

- **Header space:** the icon rail, split trigger, and now a template
  switcher all compete for the same header row. Worth a quick pass on
  header layout once this lands, not blocking the initial
  implementation.
- **Stale per-page state on switch:** flagged above — needs an explicit
  decision on what gets cleared vs. preserved when a page's template
  changes (e.g. does switching Analysis → Generic → back to Analysis lose
  the wall selection? Probably should, but worth deciding explicitly
  rather than by default of whatever code path happens to run).
- **Whether "Generic" should be selectable by the user at all**, or only
  reachable as the auto-assigned default on split — Blender doesn't let
  you switch *to* "Empty," it's just the initial state. Might want the
  same rule here: menu excludes `GENERIC`, split still defaults new pages
  to it.

**Parallelism:** independent of Phase 0 and Phase 1.

---

## Sequencing summary

| Phase | Effort | Depends on | Design decisions needed |
|---|---|---|---|
| 0 — cursor reset | trivial | none | none |
| 1 — pod collapse wiring | small | enum accessor audit | none (bug fix) |
| 2 — page-type switcher | medium | none | header placement, stale-state-on-switch policy, Generic selectability |

Deferred, not part of this pass: real 2D page splitting, which requires
`PageTree` to become graph-nested (container migration Phases 2–3) before
`SeamDirection` can mean anything at layout time.
