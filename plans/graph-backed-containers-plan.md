# Graph-Backed Containers — Phased Plan

> **Execution status (2026-08-10):** Phases 0–6 implemented. Compiles green;
> binding/cascade parity, hyper-ui (40), and persist tests pass. Ownership spike:
> [`composed_view_graph_spike.md`](./composed_view_graph_spike.md).

## Why

Today the container hierarchy (workspace → page → pod → component) lives in
plain Rust structures — `PageTree` (binary split tree) and `PodList` (flat
vec) — completely disconnected from `hypernode::Graph`, which only ever holds
domain data (walls, results, analysis engines). `SpaceClass::UIView` has
existed in `hypernode` since the beginning, unused — the design already
pointed here.

The consequence of the split: linking Drafting → Analysis → Construction is
hard, because containers have no graph identity to attach a `Signal` /
`Stream` / `Wave` edge to. `IoKind` exists as a patch over this gap — a
closed Rust enum standing in for what should be graph-native, extensible
identity.

This plan moves containers into the graph as `SpaceClass::UIView` nodes,
connected by `Binding` edges (structure) and reachable by `Signal` /
`Stream` / `Wave` edges (meaning) — without pulling spatial coordinates,
`infinite-db` `Space`s, or bbox queries into the container hierarchy. That
apparatus stays reserved for genuinely spatial domain entities. Visibility
cascade stays exactly what it is today conceptually — a pure `resolve()`
function per `SizeClass` — it just walks edges instead of struct pointers.

## Scope

**In scope:** container identity migration, containment as `Binding` edges,
cascade resolution rewrite, pod-collapse override model, `TemplateId` +
`PageTemplate` registry (replaces `IoKind` entirely), first real
`Signal`/`Stream`/`Wave` cross-container wiring for the Analysis page.

**Out of scope (future plan):** `persist_container()` / P2P sync scope
rules for container nodes. Container *identity* becomes graph-native in this
plan; container *persistence/sync policy* is deliberately deferred so this
plan doesn't grow a second unresolved fork.

## Key decision to lock before Phase 1

Domain data currently lives in a `Graph` owned per `StructuralWorkspace`
(`ws.graph`). Cross-workspace `Signal`/`Stream` wiring (Drafting pod →
Analysis pod → Construction pod, which live in *different* workspaces)
requires a graph shared across the whole app, not one graph per workspace.
This is the biggest structural change in the plan — bigger than the
container migration itself — and touches every existing call site that does
`ws.graph.*` today (`new_wall`, `run_analysis`, `handle_promote_prop`,
`build_navigation`, etc.).

**Corrected framing (superseding the original "single owned Graph"
proposal):** `AppShell` does not own *the* graph as a monolithic, tenant-wide
structure. It owns and renders a **composed, permission-scoped view** —
the union of whatever scopes the current identity has been granted and
currently has loaded. Today, with no auth/sharing model built yet, that
composed view happens to be "everything, unfiltered" — but that must be an
*implementation detail of Phase 1-6*, not an architectural assumption baked
into type or method names. Concretely:

- No type in this plan is named or shaped after `Company` or `Project`.
  Neither gets privileged status — that would recreate the exact `kind_id`
  mistake `WorkspaceInstance` already taught us to avoid. A future "project"
  is just an ordinary node that a `Grant` edge happens to root a scope at
  (see **Deferred: Identity & Sharing** below). Nothing in Phases 1-6 should
  assume containers or domain nodes belong to a single privileged owner.
- `Signal`/`Stream`/`Wave`/`Binding` edges must stay free to cross *any*
  future scope boundary exactly as freely as they already cross workspace
  boundaries — a hyperedge doesn't know or care what "boundary" its
  endpoints sit in. Don't introduce any check, filter, or assumption in
  Phases 1-6 that a node's edges must stay within one graph, one workspace,
  or one owner.
- `AppShell` should hold its `Graph` behind a narrow enough access surface
  (a method that returns "the current composed view," even if today that
  method trivially returns everything) that inserting real filtering later
  is additive — a new `Grant`-aware layer underneath the same call sites —
  rather than a rewrite of how every phase below reads the graph.

`SpaceClass` already distinguishes `UIView` / `Entity` / `Function` /
`Carrier` within one graph, so nothing about the type system blocks this —
it's an ownership and access-surface decision, not a data-model one.

**This must be explicitly re-confirmed with a short spike before Phase 1
starts** — it's large enough that it deserves its own compiles-green
checkpoint before container work begins on top of it.

## New crate dependency

`hyper-ui` gains a workspace-path dependency on `hypernode`. This is fine
under the library/app separation rule — `hypernode` is itself a generic,
independently-publishable crate with "no knowledge of Innovator or UI." The
constraint to hold: **hyper-ui's use of `hypernode` types must stay fully
generic** — `TemplateId` string values like `"analysis"` are Innovator/
structural-domain concepts and must never appear inside `hyper-ui` itself,
only in `Innovator/src/domains/structural/`.

## Vocabulary additions

- `ComponentId` — new fourth tier below `PodId`, minted the same way.
- `TemplateId(&'static str)` — replaces all `IoKind`-based dispatch.
- `PageTemplate` trait — `id()`, `header_slots()`, `build_header()`,
  `build_body()`. Implemented once per template (Navigation, Analysis,
  Results, Generic fallback).
- Edge convention: `EdgeKind::Binding` = containment only. `Signal` /
  `Stream` / `Wave` = cross-container or container↔domain data flow. Never
  mixed.

---

## Phase 0 — Dependency & vocabulary lock

**Goal:** land the scaffolding with zero behavior change.

- Add `hyper-ui → hypernode` path dependency to workspace `Cargo.toml`.
- Define `ComponentId`, `TemplateId` types (empty/unused).
- Write the one-page ownership spike for the composed-view `Graph` decision
  above and get it signed off before Phase 1 — confirming no `Company` or
  `Project` type gets introduced, and `AppShell`'s graph access goes through
  a surface narrow enough to filter later without a rewrite.

**Compiles-green check:** everything builds identically to today; no runtime
behavior differs.

**Parallelism:** none — this blocks everything else.

---

## Phase 1 — Container identity migration (coexistence)

**Goal:** every container gets a real graph identity, but nothing renders
differently yet.

- Unify `Graph` ownership at `AppShell` per the Phase 0 spike — behind the
  composed-view access surface, not a raw public field.
- `WorkspaceId` / `PageId` / `PodId` / `ComponentId` become (or wrap)
  `NodeId`.
- At construction from seed, insert a `Node { space_class: UIView, .. }`
  for every container, **in addition to** building `PageTree`/`PodList` as
  today (dual-write).
- `PageTree`/`PodList` remain the sole source of truth for rendering.

**Compiles-green check:** old rendering path untouched; new graph nodes
exist but are inert. Add a debug assertion that graph node count matches
`PageTree`/`PodList` node count after construction, as an early canary.

**Parallelism:** none — foundational.

---

## Phase 2 — Containment as `Binding` edges

**Goal:** structure becomes edge-queryable.

- Insert `Binding` edges: Workspace→Page, Page→Pod, Pod→Component, at the
  same construction point as Phase 1.
- Add an `"order"` prop (`PropValue::I64`) on each `Binding` edge so
  traversal order is deterministic without relying on struct position.
- Still dual-write — `PageTree`/`PodList` still drive rendering. Add a test
  that walks `Binding` edges and asserts the resulting order/shape matches
  `PageTree`/`PodList` exactly, across all five seeded workspaces.

**Compiles-green check:** parity test passes for all current seed data.

**Parallelism:** none — needed before Phase 3.

---

## Phase 3 — Cascade resolution migration (highest risk, isolate it)

**Goal:** the visibility cascade walks edges instead of struct pointers.
The cascade *algorithm* (focus-derived priority, guaranteed floor,
`SizeClass` ladder) does not change — only its traversal substrate does.

- Rewrite cascade resolution to query `Binding` edges ordered by `"order"`
  instead of recursing `PageTree::Split`/`Leaf`.
- Run both old and new cascade side by side behind a flag; assert identical
  `Shown`/`Hidden`/`Collapsed` output across the full `SizeClass` ladder for
  every seeded page, at a range of viewport widths, before deleting
  anything.
- Once parity holds: delete `PageTree`. Decide whether `PodList` is deleted
  outright or kept as a cheap cached projection rebuilt from `Binding`
  queries on structural change (recommended — avoids an edge walk every
  frame for something that rarely restructures).

**Compiles-green check:** parity harness passes, then old path deleted with
compiler enforcing no dangling references.

**Parallelism:** none internally, but Phases 4 and 5 can start once Phase 2
lands — they don't need Phase 3 finished.

---

## Phase 4 — Pod collapse: resolved default + sticky override

**Goal:** reconcile "pods collapse only by user intent" with "responsive
across screen sizes," using the same two-tier pattern `Overrides` already
uses for page split fractions.

- `default_collapse(pod: PodId, size_class: SizeClass) -> bool` — pure
  function, no persisted state.
- Extend `Overrides` to also record pod collapse state keyed by
  `(PodId, SizeClass)`, written only on explicit user toggle.
- Resolution rule: override present → override wins, always. No override →
  resolved default. Screen size may suggest a starting state for anything
  untouched; it may never revert a decision the user already made.
- Component-level reflow (stack vs row, icon-only vs labeled) gets the same
  pure-`resolve()` treatment, **no override tier** for now — nothing to
  "undo" at that granularity yet.

**Compiles-green check:** existing pod-collapse tests still pass at Large;
new tests cover default-vs-override precedence at Compact/Medium.

**Parallelism:** independent of Phase 5; can run concurrently.

---

## Phase 5 — `TemplateId` + `PageTemplate` registry (kills `IoKind`)

**Goal:** delete `IoKind` and every shape-matching dispatch point.

- Add `TemplateId` as a prop on Page/Pod `UIView` nodes at seed-construction
  time (explicit, not inferred).
- Define `PageTemplate` trait; implement `Navigation`, `Analysis`,
  `Results`, and a `Generic` fallback (introspects props/edges instead of
  matching an enum) — one file each under `src/pages/<name>/template.rs`.
- Single registry: `src/pages/registry.rs`, `HashMap<TemplateId, Box<dyn
  PageTemplate>>`.
- `build_one_page` becomes: look up `TemplateId` → call
  `template.build_header()` / `template.build_body()`. No more shape
  matches.
- **Delete:** `src/domains/structural/io_kind.rs`,
  `io_kind_from_label()` in `templates.rs`, the slice-match in
  `build_pages.rs`, the `is_analysis` check in `build_page_header.rs`, the
  manual registry in `pages.rs`.

**Compiles-green check:** deletion of `IoKind` and its match sites is itself
the checkpoint — compiler will refuse to build until every call site is
migrated, which is the point.

**Parallelism:** independent of Phase 4; can run concurrently. Both need
Phase 2 done.

---

## Phase 6 — `Signal`/`Stream`/`Wave` cross-container wiring

**Goal:** deliver the actual payoff — Drafting → Analysis → Construction,
for real, starting with the worked Analysis-page example already speced:

- `Wall Selector` pod `--Stream-->` Wall nodes (`Entity`).
- `Wall Properties` pod `--Binding-->` active Wall node.
- `[Wall] --Signal(RunAnalysis)--> [ACI 318 Engine] --Signal(AnalysisComplete)--> [Results]`
  (already exists at the domain layer today — this phase makes it reachable
  *from* a `UIView` pod, not just between domain nodes).
- `Results` node `--Stream--> ` Results pod, keeping it live-updating.
- Extend `Analysis` template's `build_header()` to read the Results pod's
  status via its own `Binding` children rather than any `IoKind`-derived
  check.

**Compiles-green check:** the worked example (Analysis page, Special
Concrete Wall) renders identically to pre-migration behavior, but is now
driven entirely by graph queries.

**Parallelism:** depends on Phase 1 (unified graph) and Phase 5
(`TemplateId`) — the last phase to start.

---

## Deferred to a future plan

- `persist_container()`, mirroring `persist_wall()` / `persist_results()`.
- Local-vs-shared prop scoping for P2P sync (scroll offset, in-progress
  drag state must never sync; `Binding` structure and `TemplateId` probably
  should).
- Whether `Binding` edges get rendered visually (the WGSL edge pipeline
  already supports it — an open question, not a blocker).

### Deferred: Identity & Sharing

Not started in this plan — noted here so the container work doesn't
foreclose it. If Phases 0-6 hold the discipline above (no privileged
`Project`/`Company` type, edges free to cross any boundary, graph access
behind a composed-view surface), this becomes a genuinely additive layer
on top rather than a refactor of the container work:

- **`Grant` edge** — a new `EdgeKind`, `sources: [Identity...]`,
  `targets: [scope-root node...]`, carrying a role/capability prop. Any
  node can be a scope root — a whole workspace, a single page, a single
  pod. No privileged "Project" node type; a project is just whatever node
  someone's `Grant` happens to root at.
- This is exactly why the *hyper*graph property (multi-source,
  multi-target per edge) matters, not just "a graph": one `Grant` can name
  several identities and several scope-roots in a single edge — "these
  three companies' structural engineers get Viewer on Drafting and
  Analysis" — instead of enumerating pairwise grants.
- `AppShell`'s composed view (Phase 0/1) becomes: walk `Grant` edges for
  the current identity, union the reachable scopes, render that. Existing
  `src/auth/` (`Role`, `Capability`, `CapabilitySet`, `Session`) is the
  right foundation to extend — today it's flat/local (one `Session`, no
  project scoping); it needs to become scope-aware rather than replaced.
- **Open question, not answered here:** when a `Signal`/`Stream`/`Wave`
  edge connects a node the current identity can see to one it can't, is
  the far end hidden entirely or shown as an opaque stub ("connected to
  something outside your access")? For AEC — contracts, liability, who
  knew what when — this is a real decision, not a cosmetic one, and
  deserves its own pass when this plan starts.
- **Explicitly out of scope even then:** cryptographic enforcement over
  P2P (encrypting scoped data so an unauthorized peer holds ciphertext,
  not just an honest client that chooses not to render it). App-level
  `Grant` edges solve *authorization*; they do not by themselves solve
  *confidentiality* once data has physically synced to another peer's
  machine. That's a key-management design, sequenced after this.

## Risks

- **Unified-`Graph` ownership change (pre-Phase-1 spike)** is the largest
  unknown in this plan and touches existing wall code paths. Do not start
  Phase 1 until this has its own signed-off design.
- **Accidentally privileging a "Project" or "Company" type** anywhere in
  Phases 1-6 (a struct, an enum variant, a field name) would quietly
  foreclose the `Grant`-edge model above. Watch for this in review the
  same way `kind_id` reintroduction gets watched for elsewhere.
- **Phase 3 parity harness** is load-bearing — do not delete `PageTree`
  until it's green across the full `SizeClass` ladder for all five seeded
  workspaces, not just the Analysis page.
- **`PodList` fate** (deleted vs. cached projection) should be decided with
  a quick perf sanity check, not assumed.
