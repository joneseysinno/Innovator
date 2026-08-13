# Hypergraph Connectivity — Phased Plan

> **Status:** in progress, 2026-08-13. Phases 0–5 and 7 landed; Phase 4c
> partial (Home launcher); Phase 6 isolated/deferred. Supersedes nothing; extends
> [`graph-backed-containers-plan.md`](./graph-backed-containers-plan.md)
> Phases 1–2, which landed partially.

## Why

An audit of the container migration found the model and the code have
drifted apart in nine places. The headline: **the composed view is nine
disconnected components**, the bottom two tiers of the container hierarchy
aren't in the graph at all, and every navigation action in the app is a
`HashMap<ParticleId, …>` standing in for an edge.

The container plan's Phases 1–2 were marked implemented. They were —
for the Workspace→Page→Pod range, on eight of nine workspaces. What was
never built: the root above workspaces, the component tier below pods, the
particle tier below that, and Home. The parity tests pass because they test
`dual_write_page_tree` directly from seed data rather than testing what
`Workspace::from_seed` actually does.

This plan closes those gaps. It does **not** touch rendering. The graph
view page currently draws nothing (camera never fits content; see Deferred),
and fixing it is explicitly out of scope — there is nothing worth looking at
until the graph underneath is connected.

**That constraint shapes the whole plan: no phase can be verified visually.
Every compiles-green check below is a test or a debug assertion.**

## Scope

**In scope:** root container node, Home's missing containers, component
tier as graph nodes, particle tier as graph nodes, per-instance valence
props, navigation intent as `Signal` edges, `PageId`/`PodId` collapsed into
`NodeId`, genuine multi-target hyperedges where the wiring is already
naturally multi-target.

**Out of scope:** physarum flux (deferred until there is a connected graph
to flow through), semantic zoom / LOD, graph view camera fixes,
`GraphScope` → focus refactor, Grant/identity.

## Key decisions locked before Phase 0

**Particles are nodes.** The smallest addressable unit is a particle, not a
component. This is affordable only because the view will eventually be
depth-limited (semantic zoom, deferred) — never rendering more than one
space's contents at a time. Until that lands, expect the graph to contain
thousands of nodes and **do not** attempt to render it whole.

**Valence is per instance, not per type.** A `SourceParticle` is not
inherently an emitter; *this* `SourceParticle` declares what it emits.
Valence lives in `Node.props`, which is already an open `BTreeMap`. There
is no `enum Polarity`. Writing one would be `IoKind` for the fourth time.

**Valence is measured, never enforced.** Nothing rejects an unbalanced
component. Imbalance is reported, not forbidden — a static label with no
input is legitimate, and a system that forces null particles to satisfy a
symmetry constraint is worse than one that tolerates asymmetry and can see
it.

**Tier is derived, then cached.** Depth-from-root is a `Binding`-walk
result, stored back as a `PropValue::I64` for cheap lookup and asserted
consistent. It is *not* a new `SpaceClass` variant and *not* a closed enum.
`SpaceClass` continues to answer "what kind of node," never "how deep."

**Visual contrast is not informational valence.** Foreground/background is
a real duality but resolves at draw time; emit/absorb resolves at wiring
time. They stay separate models, or a theme change perturbs the flux graph.

## Vocabulary additions

- **Root node** — one `SpaceClass::UIView` node, unlabeled or `"root"`,
  `Binding` to every workspace. No privileged type, no `Session`/`App`
  struct. It is an ordinary node that happens to sit at depth 0.
- `TIER_PROP: &str = "tier"` — `PropValue::I64`, depth from root.
- `VALENCE_EMIT_PROP` / `VALENCE_ABSORB_PROP` — per-instance declaration of
  what a particle offers and demands. **Open question, see Risks:** token
  encoding depends on `PropValue`'s available variants; if there is no list
  variant, multi-valued valence needs either a delimiter convention or
  indexed keys (`valence_emit.0`, `valence_emit.1`). Confirm before Phase 4.
- Edge convention unchanged: `Binding` = containment. `Signal`/`Stream`/
  `Wave` = flow. Navigation intent is a `Signal`, not a `Binding`.

---

## Phase 0 — Tier derivation + vocabulary lock

**Goal:** land the prop constants and the tier walk with zero behavior
change.

- Define `TIER_PROP`, `VALENCE_EMIT_PROP`, `VALENCE_ABSORB_PROP` in
  `src/workspace/graph_containers.rs` (sibling file, no `mod.rs`).
- Add `derive_tiers(graph: &mut Graph, root: NodeId)` — BFS over `Binding`
  edges writing depth back as `TIER_PROP`. Unused this phase.
- Add `assert_tier_consistency(graph, root)` — re-walks and compares.
- Confirm `PropValue`'s variant set and decide the multi-valued encoding.

**Compiles-green check:** everything builds; `derive_tiers` has a unit test
over a hand-built three-level graph; no call sites yet.

**Parallelism:** none — blocks everything.

---

## Phase 1 — Root node

**Goal:** the composed view becomes one connected component.

- `AppShell` construction inserts the root before any workspace, and each
  workspace's `insert_uiview` gains a `Binding` from root, ordered by
  position in `seed::ALL`.
- `restore_workspaces` does the same on the persistence path — currently it
  mints workspace UIViews with no parent, exactly like the seed path.
- Delete the scope-root hack in `Workspace::build_graph_workspace_content`
  (`open_id == "structural_analysis"` string match, with an `or_else`
  fallback excluding two more hardcoded strings). The root *is* the scope
  root.

**Compiles-green check:** a test asserts BFS over `Binding` from root
reaches every workspace node, on both the seed path and a
restore-from-persisted path. Node count from root equals `graph.nodes.len()`
minus unbound domain entities (which Phase 7 fixes).

**Risk:** `add_workspace` mints workspaces at runtime — it must bind to root
too, or new tabs silently orphan.

**Parallelism:** none — Phases 2–7 all assume a root exists.

---

## Phase 2 — Home joins the model

**Goal:** the workspace you named as the hub stops being a childless orphan.

- `HomeWorkspace` gains `page_tree`, `page_overrides`, `focused_page`, and
  the trigger maps every other workspace body already has.
- `Workspace::from_seed`'s `"home"` arm calls `page_tree_from_seeds` and
  `dual_write_page_tree` like the other seven. `HOME_PAGES` already exists
  in `seed.rs` with two pages and two pods — the data was always there, the
  construction path just ignored it.
- Same fix in `restore_workspace`'s `"home"` arm.

**Compiles-green check:** extend `binding_parity_across_seeded_workspaces`
to assert against **live workspaces built by `Workspace::from_seeds`**, not
against trees built directly from seed data. Drop the `with_tree >= 5`
floor to an exact count. That floor is precisely why this bug survived —
the test exercised `dual_write_page_tree` and never noticed that Home's
construction path doesn't call it.

**Parallelism:** independent of Phase 3 onward.

---

## Phase 3 — Component tier

**Goal:** the fourth tier stops being a tuple-keyed map of strings.

- `ComponentId` already exists in `crates/hyper-ui/src/pod/component_id.rs`,
  already wraps `NodeId`, and is documented "unused until graph-backed
  containers land." This is that.
- `IoSeed` becomes a component `UIView` node at construction, with a
  `Binding` from its pod carrying `"order"`.
- Delete `StubIoMap` (`HashMap<(PageId, PodId), Vec<String>>`) and
  `stub_ios_from_pages`. Stub labels come from `Node.label` via
  `binding_children`.
- `PlaceholderWorkspace.stub_ios` and its persistence in
  `restore_stub_ios` go away with it.

**Compiles-green check:** parity test extends to three levels —
Workspace→Page→Pod→Component — across all nine workspaces. Placeholder
workspaces render identical stub labels to today, sourced from the graph.

**Risk:** this is the first phase that deletes a persisted structure.
Existing saves carry `stub_ios`; the restore path needs to tolerate its
absence and rebuild from seed, or bump a save version.

**Parallelism:** must precede Phase 4.

---

## Phase 4 — Particle tier (three steps, kept separate on purpose)

**Goal:** particles become graph citizens without a big-bang rewrite of
every `build_*` function.

This is the largest phase in the plan and the one most likely to sprawl.
Splitting it is not optional.

### 4a — Identity, dual-written

- Each built particle gains a `NodeId` alongside its `ParticleId`, and a
  `Binding` from its owning component.
- `ParticleId` stays as the render-tree handle. The six bridge maps
  (`particle_sinks`, `page_show_triggers`, `pod_collapse_triggers`,
  `icon_rail_triggers`, `filter_triggers`, `actions`) stay untouched.
- Nothing reads the new nodes yet.

**Green check:** `derive_tiers` reaches depth 5; particle node count matches
a recursive walk of the particle tree.

### 4b — Valence props

- Per-instance `VALENCE_EMIT_PROP` / `VALENCE_ABSORB_PROP` written at build
  time. Defaults follow the natural reading of each variant (a `Source`
  emits text, a `Sink` absorbs pointer, a `Field` does both, a `Signal` is
  a zero-extent carrier) but every one is overridable per instance.
- Add `valence_report(graph, space_root) -> ValenceReport` — counts
  emitters, absorbers, and unmatched declarations under a space. **Reports
  only. Rejects nothing.**

**Green check:** unit test over a hand-built component asserting a known
imbalance is detected and a balanced one isn't. No production call site
changes behavior.

### 4c — Bridge map deletion

- Replace each `HashMap<ParticleId, X>` lookup with a `Binding`/`Signal`
  edge walk from the particle node.
- Delete the maps one at a time, compiler-enforced.

**Green check:** every interaction (page show, pod collapse, icon rail,
graph filter chips, tab strip) behaves identically with the maps gone.

**Parallelism:** 4a blocks 4b and 4c. 4b and 4c are independent of each
other.

---

## Phase 5 — Navigation as edges

**Goal:** Home genuinely has edges to other workspaces, instead of a map of
strings that describes edges.

- `build_content` in `src/domains/home/build_content.rs` currently does
  `actions.insert(trigger.id, AppSignal::OpenWorkspace(seed.open_id))` — a
  closed enum over `&'static str` keys, which is the `kind_id` pattern
  surviving in the navigation layer.
- Replace with a `Signal` edge from the Launcher trigger's particle node to
  the target workspace node. Pressing the trigger resolves its target by
  walking that edge.
- Same treatment for `AppSignal::SelectWorkspace(WorkspaceId)` in the tab
  strip.
- `AppSignal` shrinks to genuinely app-global actions (`AddWorkspace` and
  similar) or disappears.

**Green check:** a test asserts every `LAUNCHABLE` seed has exactly one
inbound `Signal` from Home, and that opening a workspace from Home is a
visibility write reached through an edge walk.

**Parallelism:** needs 4a for particle node identity.

---

## Phase 6 — Identity collapse (highest risk, isolate it)

**Goal:** one identity per container, not two.

`PodId` is still `pub struct PodId(pub u32)`. `PageId` likewise. Every
container therefore carries both an ordinal id and a `node_id`, kept in
sync by `dual_write_page_tree` doing `edges.retain(…)` and reinserting on
every write. `sync_page_order_from_bindings` exists solely to repair drift
between the two. That function is evidence of the problem, not a feature.

- `PageId` and `PodId` become `NodeId` (or newtypes over it, matching
  `ComponentId`).
- Delete dual-write. `PageTree`/`PodList` become caches rebuilt from
  `binding_children` on structural change, per the container plan's own
  Phase 3 recommendation.
- Delete `sync_page_order_from_bindings` and `assert_binding_parity` —
  with one identity there is nothing left to keep in parity.

**Green check:** the full cascade parity harness from the container plan's
Phase 3 runs green across the `SizeClass` ladder for every seeded page at a
range of viewport widths, before any deletion. Then delete with the
compiler enforcing no dangling references.

**Risk:** highest in the plan. `PageId(0)` / `PodId(u32)` appear in
persistence types, override keys, focus paths, and seam ids. Save-format
migration is unavoidable. Consider a save version bump rather than
tolerating both shapes.

**Parallelism:** last structural phase. Do not start before 3 and 4a land.

---

## Phase 7 — Real hyperedges

**Goal:** use the property that makes this a hypergraph rather than a graph.

Today nothing in the app uses multi-source or multi-target.
`wire_wall_list_streams` inserts N separate 1:1 `Stream` edges — one per
wall — where the natural form is one edge with N targets. As a result,
`build_spatial`'s `is_hyper` branch and the entire synthetic-junction
rendering path are dead code that has never executed.

- Collapse `wire_wall_list_streams` to a single `Stream` with
  `targets: vec![all_walls]`.
- Bind domain entities into the container tree so walls stop floating.
  `first_entity_id(graph)` currently picks an arbitrary unbound node.
- Audit `graph_wires.rs` for other naturally-multi wiring.

**Green check:** a test asserts at least one edge in the seeded graph has
`sources.len() > 1 || targets.len() > 1`, and that `binding_children` and
the cascade are unaffected by the collapse.

**Parallelism:** independent of 4–6; can run any time after Phase 1.

---

## Deferred

- **Physarum flux.** Explicitly deferred at your direction. Once the graph
  is connected and valence is declared, `inject()`'s source/sink lists
  derive from emitters and absorbers instead of the synthetic flux
  `sync_physarum` uses today. Per-space networks
  (`HashMap<NodeId, PhysarumNetwork<u64>>` keyed by space root), not one
  global network.
- **Semantic zoom / LOD.** Depth becomes a function of camera zoom rather
  than a click. Requires Phase 0's tier prop and hierarchical subtree
  culling (Barnes-Hut-shaped, needed earlier than the 500-node threshold
  the graph-view plan assumed).
- **Graph view camera.** Two bugs, both real, neither worth fixing before
  LOD decides what the camera should frame: no fit-to-content (the layout
  expands under all-pairs repulsion far past the ±10 world-unit visible
  rect at `zoom: 40`, so everything is culled), and `camera.set_screen_size`
  receives the canvas pod's *size* while `world_to_screen` draws in window
  coordinates, centering the scene behind the tab strip. Also delete
  `let _ = rebuild_spatial;` in `window_event.rs`.
- **`GraphScope` → focus.** The three-variant enum plus the string-matched
  scope root should become `focus: NodeId` + depth. Phase 1 removes the
  string match; the enum itself waits for the view work.
- **Grant / identity.** Unchanged from the container plan. Nothing here
  introduces a privileged owner type, and the root node is deliberately an
  ordinary node so a future `Grant` can root a scope anywhere.

## Open questions to resolve during the plan

1. `PropValue` variant set, and therefore how multi-valued valence is
   encoded. Blocks Phase 4b.
2. Save-format versioning strategy. Phases 3 and 6 both break persisted
   shapes; decide once, up front, rather than twice.
3. Whether `PageTree`/`PodList` survive Phase 6 as caches or are deleted
   outright. Recommendation: keep as caches — an edge walk per frame for
   something that rarely restructures is the wrong trade.
4. Whether the root node gets a stable well-known id or is discovered by
   "the node with no inbound `Binding`." The latter is more honest but
   breaks the moment a second unbound node appears.
