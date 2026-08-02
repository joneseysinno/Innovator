# Option B — `WorkspaceInstance` trait-object refactor

## What this replaces

`WorkspaceInstance` is currently a concrete enum with four variants
(`Analysis`, `Home`, `Pm`, `Empty`). Every new domain requires:
- A new variant
- New `match` arms in `instance.rs` (14 methods)
- A new arm in `open_workspace`
- A new arm in `build_tree`
- A new arm in every `handle_*` file that pattern-matches on the variant

This plan deletes the enum and replaces it with a trait object. After this
refactor, adding a new workspace domain touches zero existing files.

---

## Module naming convention

Sibling-file pattern throughout — no `mod.rs` files.

```
src/foo.rs          ← declares submodules
src/foo/
    bar.rs
```

---

## New type: `WorkspaceFacade` trait

Lives at `src/workspace/facade.rs`.

This is the trait every domain workspace implements. Its methods are
exactly what the shell currently extracts through the `WorkspaceInstance`
match arms — no more, no less.

```rust
// src/workspace/facade.rs

use crate::workspace::app_signal::AppSignal;
use crate::workspace::header::WorkspaceHeader;
use crate::workspace::signal::WorkspaceSignal;
use crate::workspace::tab::WorkspaceTab;
use crate::workspace::workspace_id::WorkspaceId;
use hyper_ui::{InMemoryWorldSpatial, PageTree, ParticleId, Rect};
use hyper_ui::particles::Particle;
use std::any::Any;

/// Contract every domain workspace must satisfy.
/// The shell calls these — domains never call back into the shell.
pub trait WorkspaceFacade: Any {
    // ── Identity ──────────────────────────────────────────────────────────

    fn tab(&self) -> &WorkspaceTab;

    fn id(&self) -> WorkspaceId {
        self.tab().id
    }

    fn kind_id(&self) -> &'static str;

    // ── Chrome ────────────────────────────────────────────────────────────

    /// Optional action header rendered between the tab strip and page body.
    fn header(&self) -> Option<&WorkspaceHeader> {
        None
    }

    /// Status ParticleId for live signal text, if any.
    fn status_id(&self) -> Option<ParticleId> {
        None
    }

    // ── Layout ────────────────────────────────────────────────────────────

    /// Page-split binary tree, if this workspace uses one.
    fn page_tree(&self) -> Option<&PageTree> {
        None
    }

    fn page_tree_mut(&mut self) -> Option<&mut PageTree> {
        None
    }

    // ── Content ───────────────────────────────────────────────────────────

    /// Build the workspace body particle tree. Called on every rebuild.
    fn build_content(&mut self) -> Particle;

    // ── Event dispatch ────────────────────────────────────────────────────

    /// Handle a workspace-level signal. Return true if a rebuild is needed.
    fn handle_workspace_signal(
        &mut self,
        signal: WorkspaceSignal,
        db: &mut infinite_db::InfiniteDb,
        signal_tx: &flume::Sender<String>,
    ) -> HandleResult {
        let _ = (signal, db, signal_tx);
        HandleResult::Ignored
    }

    /// Handle an app-level signal. Return true if a rebuild is needed.
    fn handle_app_signal(
        &mut self,
        signal: AppSignal,
    ) -> HandleResult {
        let _ = signal;
        HandleResult::Ignored
    }

    // ── Type erasure escape hatch ─────────────────────────────────────────

    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Result returned from event handlers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HandleResult {
    /// Event was consumed; caller must rebuild the particle tree.
    Rebuild,
    /// Event was consumed; no rebuild needed (e.g. status text update only).
    Consumed,
    /// Event was not handled by this workspace.
    Ignored,
}
```

### Why `as_any` / `as_any_mut`

The shell's domain-specific handlers (`handle_analysis_action`,
`handle_value_changed`, `handle_page_signal`, etc.) need direct access
to `StructuralWorkspace` fields. Routing all of that through the trait
would bloat the facade with structural-domain concerns. Instead, those
handlers downcast:

```rust
let Some(ws) = shell.active_mut()
    .and_then(|a| a.as_any_mut().downcast_mut::<StructuralWorkspace>())
else { return; };
```

This is the standard Rust pattern for "I need the concrete type in a few
specific places, but trait object everywhere else." It is explicit and
deliberate — not a workaround. If a handler only touches `StructuralWorkspace`,
that is correctly expressed by `downcast_mut::<StructuralWorkspace>()`.

---

## `WorkspaceInstance` — the new wrapper

`src/workspace/instance.rs` becomes a thin newtype around the trait object.
The enum is deleted entirely.

```rust
// src/workspace/instance.rs

use crate::workspace::facade::WorkspaceFacade;

/// A live workspace instance — owns a boxed domain workspace.
pub struct WorkspaceInstance(pub Box<dyn WorkspaceFacade>);

impl WorkspaceInstance {
    pub fn new<W: WorkspaceFacade + 'static>(ws: W) -> Self {
        Self(Box::new(ws))
    }
}

impl std::ops::Deref for WorkspaceInstance {
    type Target = dyn WorkspaceFacade;
    fn deref(&self) -> &Self::Target { &*self.0 }
}

impl std::ops::DerefMut for WorkspaceInstance {
    fn deref_mut(&mut self) -> &mut Self::Target { &mut *self.0 }
}
```

`Deref` means existing call sites like `shell.active().and_then(|a| a.header())`
continue to compile unchanged — they just call through the trait object.

---

## `WorkspaceKind` enum — deleted

`src/workspace/kind.rs` is deleted. The `kind()` method on `WorkspaceInstance`
becomes `kind_id() -> &'static str` (already on the trait). Every callsite
that matched on `WorkspaceKind` is updated:

| Old | New |
|---|---|
| `w.kind() == WorkspaceKind::Analysis` | `w.kind_id() == "structural_analysis"` |
| `WorkspaceKind::Analysis` in open_workspace | `StructuralDescriptor::KIND_ID` constant |

`WorkspaceTab` currently stores `kind: WorkspaceKind`. It becomes:

```rust
pub struct WorkspaceTab {
    pub id: WorkspaceId,
    pub kind_id: &'static str,   // replaces kind: WorkspaceKind
    pub title: String,
    pub icon: &'static str,
}
```

---

## `AppShell.workspaces` field — unchanged in shape

```rust
pub workspaces: Vec<WorkspaceInstance>,
```

Still a `Vec`. `active()` and `active_mut()` in `app_shell/active.rs`
remain identical — they return `Option<&WorkspaceInstance>` /
`Option<&mut WorkspaceInstance>`, which now derefs to `&dyn WorkspaceFacade`.

---

## `build_tree` — the biggest simplification

Before (5 match arms, imports from every domain):

```rust
let kind = shell.active().map(|a| a.kind());
let body = match kind {
    Some(WorkspaceKind::Analysis) => { ... build_pages(ws) ... }
    Some(WorkspaceKind::Home)     => { ... build_home(ws) ... }
    Some(WorkspaceKind::PM)       => build_pm(),
    Some(WorkspaceKind::Empty)    => build_empty(),
    None                          => build_empty(),
};
```

After (zero match arms, no domain imports):

```rust
let body = shell
    .active_mut()
    .map(|a| a.build_content())
    .unwrap_or_else(build_empty_particle);
```

`build_tree.rs` imports nothing from `domains/`. It calls one method.

---

## `open_workspace` — before and after

Before:

```rust
let instance = match kind {
    WorkspaceKind::Analysis => WorkspaceInstance::Analysis(AnalysisWorkspace::new(id, db)),
    WorkspaceKind::PM       => WorkspaceInstance::Pm(PmWorkspace::new(id)),
    WorkspaceKind::Home     => WorkspaceInstance::Home(HomeWorkspace::new(id)),
    WorkspaceKind::Empty    => WorkspaceInstance::Empty(EmptyWorkspace::new(id)),
};
```

After (uses the registry):

```rust
pub fn open_workspace(shell: &mut AppShell, kind_id: &'static str) {
    if let Some(id) = shell.workspaces.iter()
        .find(|w| w.kind_id() == kind_id)
        .map(|w| w.id())
    {
        select_workspace(shell, id);
        return;
    }

    let id = WorkspaceId(shell.next_workspace_id);
    shell.next_workspace_id += 1;

    let Some(descriptor) = shell.registry.find(kind_id) else { return; };
    let instance = WorkspaceInstance::new(descriptor.spawn(id, &mut shell.db));

    shell.workspaces.push(instance);
    // ... rebuild as before
}
```

`open_workspace.rs` imports nothing from any domain.

---

## Domain-specific handlers — downcast pattern

The handlers that need `StructuralWorkspace` fields directly use downcast.
This is clean, explicit, and only appears in structural-domain handler files.

### `handle_analysis_action.rs`

```rust
use crate::domains::structural::StructuralWorkspace;

pub fn handle_analysis_action(shell: &mut AppShell, action: AnalysisAction) {
    let active_id = shell.active_id;
    let Some(idx) = shell.workspaces.iter().position(|w| w.id() == active_id) else {
        return;
    };
    let Some(ws) = shell.workspaces[idx]
        .as_any_mut()
        .downcast_mut::<StructuralWorkspace>()
    else { return; };

    // ... rest unchanged, ws is &mut StructuralWorkspace
}
```

### `handle_value_changed.rs`, `handle_workspace_signal.rs`, `handle_page_signal.rs`

Same pattern. Each file's downcast makes it explicit: "this handler is
structural-domain only." If a future domain needs its own value-changed
handling, it gets its own handler file.

### `window_event.rs` — `.analysis()` method

Currently `instance.analysis()` returns `Option<&AnalysisWorkspace>`.
This is replaced with:

```rust
// In window_event.rs, where StructuralWorkspace fields are needed:
let ws = shell.active()
    .and_then(|a| a.as_any().downcast_ref::<StructuralWorkspace>());
```

The `WorkspaceFacade` trait provides everything that is generic
(header, status_id, page_tree, build_content). The downcast is used only
where structural-specific data is needed (wall_sinks, icon_rail_triggers,
wall_view_sink, io_rect, etc.).

---

## `WorkspaceFacade` implementations

### `StructuralWorkspace` (renamed from `AnalysisWorkspace`)

```rust
// src/domains/structural/workspace.rs

impl WorkspaceFacade for StructuralWorkspace {
    fn tab(&self) -> &WorkspaceTab { &self.tab }
    fn kind_id(&self) -> &'static str { StructuralDescriptor::KIND_ID }
    fn header(&self) -> Option<&WorkspaceHeader> { self.header.as_ref() }
    fn status_id(&self) -> Option<ParticleId> { self.header.as_ref().map(|h| h.status_id) }
    fn page_tree(&self) -> Option<&PageTree> { Some(&self.page_tree) }
    fn page_tree_mut(&mut self) -> Option<&mut PageTree> { Some(&mut self.page_tree) }
    fn build_content(&mut self) -> Particle { build_pages(self) }

    fn handle_workspace_signal(
        &mut self,
        signal: WorkspaceSignal,
        db: &mut InfiniteDb,
        signal_tx: &flume::Sender<String>,
    ) -> HandleResult {
        structural_handle_workspace_signal(self, signal, db, signal_tx)
    }

    fn as_any(&self) -> &dyn Any { self }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}
```

### `HomeWorkspace`, `PmWorkspace`, `EmptyWorkspace`

Each implements `build_content()` returning their existing content particle.
All other trait methods use the default implementations (return `None`).

---

## `AppSignal::OpenWorkspace` — kind arg change

Before:
```rust
OpenWorkspace(WorkspaceKind)
```

After:
```rust
OpenWorkspace(&'static str)   // kind_id string
```

Callers (tab strip launcher, home page actions) pass the `KIND_ID` constant
from the descriptor:

```rust
AppSignal::OpenWorkspace(StructuralDescriptor::KIND_ID)
```

---

## Files deleted

| File | Reason |
|---|---|
| `src/workspace/kind.rs` | `WorkspaceKind` enum removed |

## Files created

| File | Contents |
|---|---|
| `src/workspace/facade.rs` | `WorkspaceFacade` trait + `HandleResult` |

## Files changed (signature of change)

| File | Change |
|---|---|
| `src/workspace/instance.rs` | Enum → newtype wrapper around `Box<dyn WorkspaceFacade>` |
| `src/workspace.rs` | Remove `kind` re-export; add `facade` module |
| `src/workspace/tab.rs` | `kind: WorkspaceKind` → `kind_id: &'static str` |
| `src/workspace/app_signal.rs` | `OpenWorkspace(WorkspaceKind)` → `OpenWorkspace(&'static str)` |
| `src/workspace/app_shell/build_tree.rs` | Replace match with `a.build_content()` |
| `src/workspace/app_shell/open_workspace.rs` | Replace match with registry dispatch |
| `src/workspace/app_shell/handle_analysis_action.rs` | `WorkspaceInstance::Analysis(ws)` → downcast |
| `src/workspace/app_shell/handle_value_changed.rs` | Same |
| `src/workspace/app_shell/handle_workspace_signal.rs` | Same |
| `src/workspace/app_shell/handle_page_signal.rs` | Same |
| `src/workspace/app_shell/handle_builder_field.rs` | Same |
| `src/workspace/app_shell/window_event.rs` | `.analysis()` → downcast; remove `WorkspaceKind` import |
| `src/workspace/app_shell/sync_from_page_tree.rs` | `.analysis()` → downcast |
| `src/domains/structural/workspace.rs` | Impl `WorkspaceFacade` |
| `src/domains/home/workspace.rs` | Impl `WorkspaceFacade` |
| `src/domains/pm/workspace.rs` | Impl `WorkspaceFacade` |
| `src/domains/empty/workspace.rs` | Impl `WorkspaceFacade` |

---

## Build phases — sequenced for a green compiler at each step

### Phase 1 — Create the trait (no existing code touched)

- [ ] Create `src/workspace/facade.rs` with `WorkspaceFacade` and `HandleResult`
- [ ] Add `pub mod facade;` to `src/workspace.rs`

Acceptance: `cargo check` passes. Nothing uses the trait yet.

---

### Phase 2 — Implement the trait on all four domain workspaces

Implement `WorkspaceFacade` for:
- [ ] `AnalysisWorkspace` (in its current location — rename comes later)
- [ ] `HomeWorkspace`
- [ ] `PmWorkspace`
- [ ] `EmptyWorkspace`

All `as_any` / `as_any_mut` impls are identical boilerplate — one line each.

Acceptance: `cargo check` passes. Impls exist but are not used yet.

---

### Phase 3 — Replace `WorkspaceInstance` enum with the newtype

- [ ] Rewrite `src/workspace/instance.rs` to the newtype definition above
- [ ] `WorkspaceInstance::Analysis(ws)` → `WorkspaceInstance::new(ws)` at every
      construction site (only in `open_workspace.rs` — one file)
- [ ] All the `match` arms in the old `instance.rs` methods are gone.
      The `Deref` impl means `shell.active().and_then(|a| a.header())` still
      compiles — the trait method is called through the deref.

Compiler will flag every place that pattern-matches on the enum variant
(`WorkspaceInstance::Analysis(ws)`). Work through them systematically:

**`open_workspace.rs`** — replace the match with the registry call (see above).

**`build_tree.rs`** — replace the match with `a.build_content()`.

**`handle_analysis_action.rs`**, **`handle_value_changed.rs`**,
**`handle_workspace_signal.rs`**, **`handle_page_signal.rs`**,
**`handle_builder_field.rs`** — replace `WorkspaceInstance::Analysis(ws)`
pattern with `downcast_mut::<StructuralWorkspace>()`.

**`window_event.rs`** — replace `.analysis()` calls with downcast.
The two call sites are:
1. `sync_from_page_tree` guard
2. `icon_rail_triggers`, `pod_collapse_triggers`, `page_split_triggers` extraction

**`sync_from_page_tree.rs`** — takes `&AnalysisWorkspace` directly;
update the call site in `window_event.rs` to pass the downcasted ref.

Acceptance: `cargo check` passes. `cargo run` opens the app with identical
behavior.

---

### Phase 4 — Delete `WorkspaceKind`

- [ ] Delete `src/workspace/kind.rs`
- [ ] Update `WorkspaceTab`: `kind: WorkspaceKind` → `kind_id: &'static str`
- [ ] Update `AppSignal::OpenWorkspace(WorkspaceKind)` → `OpenWorkspace(&'static str)`
- [ ] Update all `OpenWorkspace(...)` call sites to pass `KIND_ID` strings
- [ ] Update `tab_strip` builder (currently reads `tab.kind` for display logic)
- [ ] Remove `pub mod kind;` from `src/workspace.rs`
- [ ] Remove `WorkspaceKind` re-export

Acceptance: `cargo check --all` passes with zero warnings. `WorkspaceKind`
no longer appears anywhere in the codebase.

---

### Phase 5 — Move domains (can follow or interleave with Phase 4)

This is the file-move step from the domain refactor plan.
Phase 3 and 4 are cleaner to do first because the compiler guides you
through every callsite that needs updating.

- [ ] Move `src/workspace/analysis/` → `src/domains/structural/`
- [ ] Rename `AnalysisWorkspace` → `StructuralWorkspace`
- [ ] Move `src/workspace/pm/` → `src/domains/pm/`
- [ ] Move `src/workspace/home/` → `src/domains/home/`
- [ ] Move `src/workspace/empty/` → `src/domains/empty/`
- [ ] Move `src/workspace/analysis_action.rs` → `src/domains/structural/action.rs`
- [ ] Move `src/workspace/field_builder_draft.rs` → `src/domains/structural/field_builder_draft.rs`
- [ ] Create `src/domains.rs` with `register_all()`
- [ ] Update all `use` paths

Acceptance: `cargo check --all` clean. `cargo run` opens app unchanged.

---

## What it looks like to add a new domain after this refactor

**Total files to create or edit:**

1. `src/domains/estimating.rs` — new file
2. `src/domains/estimating/workspace.rs` — new file (impl `WorkspaceFacade`)
3. `src/domains.rs` — one new `registry.register(...)` line

**Files that do NOT change:**
- `AppShell`
- `WorkspaceInstance`
- `build_tree`
- `open_workspace`
- `window_event`
- Every existing domain

---

## Quick reference — the downcast pattern

```rust
// Immutable access to StructuralWorkspace from a shell handler:
let Some(ws) = shell.active()
    .and_then(|a| a.as_any().downcast_ref::<StructuralWorkspace>())
else { return; };

// Mutable access (most handlers):
let active_id = shell.active_id;
let Some(idx) = shell.workspaces.iter().position(|w| w.id() == active_id) else {
    return;
};
let Some(ws) = shell.workspaces[idx]
    .as_any_mut()
    .downcast_mut::<StructuralWorkspace>()
else { return; };
```

The `idx`-then-downcast pattern is necessary (rather than `active_mut()`)
when the handler also needs `shell.db` or other shell fields, since
`active_mut()` borrows all of `shell`.

