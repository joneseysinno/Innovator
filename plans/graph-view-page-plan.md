# Graph View Page — Phased Plan

## Why

The container migration (`plans/graph-backed-containers-plan.md`, Phases 0–6)
made the hypergraph queryable — `binding_children()` walks `Binding` edges,
`SpaceClass`/`EdgeKind` distinguish node and edge meaning — but nothing
renders it. The plan itself flagged this as open: *"whether `Binding` edges
get rendered visually... an open question, not a blocker."*

This plan answers it: a dedicated page that renders the live composed-view
graph — nodes and directed edges, force-laid-out, not a tree — using the
`NodePipeline` / `EdgePipeline` that already exist and already draw domain
entities today. The render substrate is not new. What's new is: a layout
pass, a page to host it, and interaction on top.

Beyond dogfooding, this is a plausible real feature. AEC work cares about
provenance — "what fed into this result, and from where" — and a directed
graph view of `Signal`/`Stream`/`Wave` wiring is exactly that question,
made visible.

## Scope

**In scope:** one new page, hosted in a new lightweight workspace, showing
nodes + directed edges from a scoped subgraph, force-directed layout,
pan/zoom/select/pin interaction, basic filtering.

**Out of scope (future plan):** Grant-aware dimming of nodes outside the
current identity's access (ties to the *Deferred: Identity & Sharing*
section of the container plan — this page is where that access-boundary
rendering decision becomes testable, once it exists). Barnes-Hut
optimization for large graphs. Persisted/exportable snapshots.

## Key decision to lock before Phase 1

**Layout positions are page-local, not stored on `Node`.** `Node.world_pos`
exists today for genuinely spatial domain entities (walls, etc.) via
`infinite-db`. UIView nodes were deliberately kept out of that apparatus.
A force-directed layout computed for *this page's convenience* must not
leak into `world_pos` — that would smuggle spatial semantics onto nodes
whose position has no meaning outside this view. Instead: the page's own
pod state holds a `HashMap<NodeId, Vec2>` (plus a synthetic-junction
variant, see below), rebuilt/seeded per session, never written to the
graph itself.

**Hyperedges need a rendering strategy, not just a query.** `HyperEdge` has
`sources: Vec<NodeId>` / `targets: Vec<NodeId>` — genuinely multi-source,
multi-target. The 1-source/1-target case (the overwhelming majority today:
`Binding`, most `Stream`/`Signal` wiring) draws as one directed curve,
source → target, using the existing bezier edge pipeline. A true hyperedge
(N sources and/or M targets) gets a **synthetic junction point** — a small
render-only particle, not a graph node — with directed segments from each
source into the junction and from the junction out to each target. The
junction participates in the force simulation like any other point (low
mass, no repulsion against real nodes) so hyperedges settle naturally
instead of needing special-cased layout logic.

**Host workspace.** A new `Visibility::Hidden` workspace (`open_id:
"devtools_graph"`), seeded the same way as every other workspace in
`seed.rs` — not a bolt-on overlay outside `AppShell`. This means it gets
persistence, focus, resize, and pointer-event handling for free, the same
machinery every other page already has, instead of a second render path.
Opened via a signal (`OpenWorkspace("devtools_graph")`), bound to F11 the
same way F9/F10 already dispatch through `window_event.rs` — press F11,
get a real page, not a raw toggle flag.

**Node positions are a 2D relative embedding, not a coordinate system.**
The force simulation produces numbers that are only meaningful relative to
one another — distances and arrangement, not an absolute location in any
frame. It is a slice, not a projection of anything real: no domain
world-space, no screen-space until render time, no third dimension. This
is *why* it's safe to keep positions off `Node.world_pos` (see above) —
the values have no meaning outside this page, so they were never a graph
property to begin with, only a rendering one.

**`physarum` is its own crate from day one, not page-local logic.** The
conductivity-reinforcement model (see Phase 5) is explicitly scoped to
grow far beyond this page — a generic network-optimization engine, with
graph-view rendering as merely its first consumer. Building it generic
and dependency-free now (no `hypernode`, no `hyper-ui` in its public API)
avoids the retrofit `hypernode` itself already had to prove out: identity
and dispatch stay open, not shape-matched to whatever consumed it first.

## Vocabulary additions

- `TemplateId("graph_view")` — new `PageTemplate` impl.
- `GraphViewState` — page-local (not `AppShell`-global, not graph-stored):
  positions `HashMap<NodeId, Vec2>`, junction positions
  `HashMap<EdgeId, Vec2>`, pinned set `HashSet<NodeId>`, selected
  `Option<NodeId>`, active filters (`SpaceClass` set, `EdgeKind` set, scope
  mode).
- `GraphScope` enum — `ActiveWorkspace`, `Reachable` (BFS from active
  workspace's node through all edge kinds), `Composed` (everything the
  current access surface returns — same surface `AppShell` already uses,
  never bypassed).

## New crate: `physarum`

`Innovator/crates/physarum` — library only, no `main.rs`, own `Cargo.toml`,
workspace member, Rust 2024 edition. No dependency on `hypernode` or
`hyper-ui`; generic over any `N: Eq + Hash + Copy` node id. This is the
same library/app separation rule `hypernode` itself follows — a crate this
consequential earns the discipline from the start rather than after it's
already load-bearing for something. See Phase 5 for the initial API
surface and Deferred for why the boundary is being drawn this early.

---

## Phase 0 — Vocabulary & scope lock

**Goal:** land types with zero rendering, zero physics.

- Define `TemplateId("graph_view")`, `GraphViewState`, `GraphScope` (empty
  fields where needed, no behavior).
- Add `WorkspaceSeed` for `devtools_graph`: one page, one pod, full-bleed
  extent (mirrors `PAGE_VIEWER`/`POD_VIEW` sizing already used for
  Drafting/Engineering viewers).
- Wire `F11` in `window_event.rs` to an `OpenWorkspace("devtools_graph")`
  signal, same shape as the existing `F9`/`F10` arms.

**Compiles-green check:** app builds identically; F11 opens an empty page.

**Parallelism:** none — blocks everything else.

---

## Phase 1 — Static snapshot render (prove the render path, isolate risk)

**Goal:** get *something* from `Graph` onto screen via the existing
pipelines, before physics is in the mix. Mirrors the container plan's own
pattern of isolating the highest-risk piece (there: cascade migration;
here: physics) from the parts that are just wiring.

- `GraphView` `PageTemplate::build_body()` returns a single canvas-style
  pod (comparable to the `WallView` pod's spatial surface).
- One-time **naive placement** — evenly-spaced grid or circle — explicitly
  *not* the final layout. This step only proves: nodes queried via scope →
  converted to `NodeInstance` → drawn; edges converted to `WorldEdge`
  (direct for 1:1, junction-routed for hyperedges) → drawn with arrowheads
  at the target end.
- Color nodes by `SpaceClass` (UIView / Entity / Function / Carrier — four
  fixed colors). Style edges by `EdgeKind` (`Binding` thin/muted, `Signal`
  solid, `Stream` dashed, `Wave` distinct dash pattern).
- If arrowheads aren't already supported by the edge pipeline: smallest
  addition is a small triangle instance drawn near the target end, oriented
  along the curve's tangent at that point — reuse `NodePipeline`'s SDF
  triangle capability if present, otherwise a minimal dedicated draw call.

**Compiles-green check:** opening the page renders the current graph's
nodes and edges with correct colors/arrow direction, on an empty graph
renders nothing without panicking, no interactivity yet.

**Parallelism:** none — needed before Phase 2.

---

## Phase 2 — Force-directed layout

**Goal:** replace naive placement with a real spring-embedder, and make
direction legible without hierarchy.

- Standard force model: all-pairs repulsion (fine at current graph sizes;
  Barnes-Hut deferred), attractive spring per edge (endpoint-to-endpoint
  for 1:1 edges, source-to-junction and junction-to-target for hyperedges),
  spring constant tunable per `EdgeKind` — `Binding` edges shorter/stiffer
  so containment clusters stay visually tight, `Signal`/`Stream`/`Wave`
  looser so cross-cluster connections have room to read clearly.
- Alpha-decay cooling: high iteration budget per frame while unstable,
  tapering toward near-zero at steady state, so newly-added nodes (e.g. a
  new wall) nudge the layout instead of re-exploding it.
- Positions live in `GraphViewState.positions`, seeded from Phase 1's
  placement for first-run, carried frame-to-frame after.
- `Stream` edges (the "live-updating" kind, per the container plan's
  Results-node wiring) get animated dash-offset — a moving-dashes effect —
  so live data flow is visually distinct from static structure at a glance.

**Compiles-green check:** physics is nondeterministic, so the checkpoint is
a headless stability test — run N simulated frames, assert total positional
delta trends toward zero and no two non-junction nodes end up closer than a
minimum-distance threshold at steady state.

**Parallelism:** none internally; Phase 3 can start once Phase 1 lands, it
doesn't need Phase 2 finished.

---

## Phase 3 — Interaction (pan / zoom / select / pin)

**Goal:** make the page usable, not just a screensaver.

- Pan/zoom: reuse the *transform* half of `StructuralWorkspace`'s
  `wall_view_last_pos` / `wall_view_panning` / `SinkPointer` handling —
  explicitly **not** `InMemoryWorldSpatial` or any `infinite-db` bbox
  query. A force graph is a few hundred points at most; spatial indexing
  is the exact apparatus your own notes say to reserve for genuinely
  spatial domain entities, and this isn't one.
- Click node → select; highlight incident edges (width/opacity bump); show
  `label` / `space_class` / props in a second "Inspector" pod on the page
  (same two-pod pattern as `STRUCTURAL_RESULTS_PODS`'s Results + Status).
- Drag node → pin. This is the same two-tier **resolved-default vs. sticky
  override** pattern already established for pod collapse
  (`crates/hyper-ui/src/pod/collapse.rs`) — a pinned node is exactly an
  override that always wins over the simulated default position. Reusing
  the pattern instead of inventing a new one keeps the codebase's "pin"
  concept singular.
- New `ParticleId` sink registered in the workspace's own trigger/sink
  maps (mirrors `wall_sinks`, `icon_rail_triggers`) — must not collide with
  Structural's existing sink routing in `window_event.rs`, since this is a
  different workspace's dispatch.

**Compiles-green check:** select/pin toggle tests; existing pointer-event
tests for Structural remain unaffected (new sink is workspace-scoped).

**Parallelism:** independent of Phase 4.

---

## Phase 4 — Filtering & scope (polish)

**Goal:** keep the view legible as the graph grows.

- Toggle chips: filter by `SpaceClass` and `EdgeKind`.
- `GraphScope` selector: `ActiveWorkspace` / `Reachable` / `Composed`.
  `Composed` must go through the same composed-view access surface
  `AppShell` already uses for everything else — never a raw/unfiltered
  graph read, even for a debug tool, per the container plan's Phase 0/1
  discipline.
- Filtering must handle dangling references cleanly: an edge whose
  endpoint got filtered out disappears (and its junction point, if any)
  rather than pointing at nothing.

**Compiles-green check:** toggling any filter combination doesn't panic on
partially-filtered hyperedges.

**Parallelism:** independent of Phase 3; both need Phase 1 done.

---

## Phase 5 — `physarum` crate + conductivity-weighted rendering

**Goal:** stand up `physarum` as a real, independent crate — implementing
the Physarum/slime-mold reinforcement model — and consume it from the
graph view as its first user, purely for visual worn-path rendering. Real
usage-driven flux and structural application are explicitly **not** in
this phase (see Deferred) — this phase is: does the crate work, and does
feeding it into edge width/opacity make the graph more legible.

- Core type: `PhysarumNetwork<N: Eq + Hash + Copy>`.
  - `add_edge(a: N, b: N, length: f64)`
  - `inject(sources: &[(N, f64)], sinks: &[(N, f64)])` — one flow
    injection pass, generalizes past a single source/sink pair since
    real usage will have many simultaneous "food sources."
  - `step(dt: f64)` — solve nodal pressure from current conductivities
    (`Q_ij = D_ij / L_ij × (p_i − p_j)`), update each edge's conductivity
    via a pluggable reinforcement function, decay the rest.
  - `conductivity(a: N, b: N) -> f64`
- Reinforcement function is a parameter (`fn(flux: f64) -> f64`, default
  `|Q|^1.0`), not hardcoded — gamma tuning (sparse-tree-like vs.
  redundant-mesh-like convergence) is a call-site decision. Same "don't
  bake a closed shape into the type" discipline that replaced `IoKind`
  with `TemplateId`.
- Graph view page (Phase 1's canvas) becomes the first consumer: each
  tick, feed synthetic flux (or simple edge-touch-count as a stand-in for
  real telemetry), read `conductivity()` per edge, map to edge width and
  opacity in the existing `EdgePipeline` draw. High-conductivity edges
  render as worn paths; low-conductivity edges fade — independent of and
  layered on top of the Phase 2 spring-embedder positions.

**Compiles-green check:** `physarum` has its own unit tests — a known
small mesh (e.g. the two-food-source example) converges to the expected
sparse result — that pass with zero knowledge of `hypernode`, `hyper-ui`,
or Innovator. Graph view renders visibly different edge weights without
the crate needing to know what a `UIView` or `Binding` is.

**Parallelism:** independent of Phases 3/4; needs Phase 1's render path
to have something to consume.

---

## Deferred

- Barnes-Hut spatial partitioning for repulsion, once graph size (roughly
  500+ nodes) makes all-pairs O(n²) repulsion visibly slow.
- Persisting or exporting a layout snapshot (useful later for incident
  review / documentation — AEC liability conversations specifically
  benefit from "here's what the system knew and when").
- Grant-aware rendering: when the identity/sharing layer lands, this page
  is the natural place to resolve the container plan's open question —
  hide nodes outside the current identity's access entirely, or render
  them as opaque stubs. Not decided here; deliberately deferred alongside
  it.

### Deferred: `physarum` as an optimizer, not just a renderer

Not started here — called out explicitly because it's the actual reason
`physarum` is being drawn as its own crate rather than left as page-local
code in this plan's Phase 5. Two future consumers, intertwined, sharing
one core:

- **Usage-driven architecture health.** Replace Phase 5's synthetic flux
  with real telemetry — `Signal` fire counts, `Stream` throughput — so
  conductivity decay surfaces genuinely dead wiring automatically. The
  Path A deletion happening right now is this process done by hand, once;
  this would make it continuous.
- **Structural load-path optimization.** Run the same reinforcement loop
  over `Entity` nodes (Wall, Support, Load) instead of `UIView` nodes —
  load sources in place of food sources, supports in place of sinks. This
  is genuine structural topology-optimization territory (a real, active
  research area), not a metaphor stretched to fit.
- Both consumers want the identical pressure-solve + reinforcement core;
  they differ only in what supplies flux and what the nodes represent.
  This is precisely why `physarum`'s public API must stay free of any
  Innovator-, UI-, or structural-specific type even in Phase 5 — a future
  end user building and executing their own graph against `physarum`
  shouldn't need to know any of those domains exist. If Phase 5's flux
  source (edge-touch-count) turns out to be hardwired anywhere `physarum`
  itself could instead accept an arbitrary flux input, that's the seam to
  watch for.
- Not designed here. This is the optimizer these two items were always
  going to become; deliberately left undesigned until Phase 5 proves the
  crate boundary holds.
