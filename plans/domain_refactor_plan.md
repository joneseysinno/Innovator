# Domain refactor plan — Innovator

## Problem statement

`src/workspace/` currently mixes two concerns that will grow in opposite
directions:

1. **Shell infrastructure** — `AppShell`, `WorkspaceInstance`, `WorkspaceKind`,
   `WorkspaceTab`, `TabStrip`, `WorkspaceSignal`, `WorkspaceHeader`, etc.
   These are generic, stable, and exist once.

2. **Domain content** — `analysis/`, `pm/`, `home/`, `empty/` and all their
   sub-modules. These are AEC-specific and will multiply.

As the app grows to serve engineers, PMs, estimators, BIM coordinators,
inspectors, and owners, every new role adds a new workspace and forces
changes to `WorkspaceKind`, `WorkspaceInstance`, `open_workspace`, and
`build_tree`. That is a closed/open principle violation — the shell has
to be modified every time content is added.

The fix: separate the shell from the domains, and give domains a
registration contract so the shell never needs to know them at compile
time.

---

## Module naming convention

This project uses the **sibling-file** module pattern throughout.
A module `foo` with submodules is always:

```
src/
├── foo.rs          ← declares submodules with `pub mod bar;`
└── foo/
    └── bar.rs
```

Never `src/foo/mod.rs`. This applies to every new file in this plan.

---

## Target directory layout

```
src/
├── main.rs                         ← unchanged · entry point · stays thin

├── workspace.rs                    ← TRIMMED · re-exports shell types only
├── workspace/
│   ├── app_shell.rs                ← unchanged
│   ├── app_shell/                  ← unchanged
│   ├── app_signal.rs               ← unchanged
│   ├── descriptor.rs               ← NEW · WorkspaceDescriptor trait
│   ├── instance.rs                 ← CHANGED · now type-erased (see below)
│   ├── registry.rs                 ← NEW · WorkspaceRegistry
│   ├── screen_class.rs             ← unchanged
│   ├── signal.rs                   ← unchanged
│   ├── size_class.rs               ← unchanged
│   ├── tab.rs                      ← unchanged
│   ├── tab_strip.rs                ← unchanged
│   └── workspace_id.rs             ← unchanged

├── auth.rs                         ← NEW · Role + Capability + Session
├── auth/
│   ├── role.rs                     ← NEW · Role enum
│   ├── capability.rs               ← NEW · Capability enum + CapabilitySet
│   └── session.rs                  ← NEW · Session struct

└── domains.rs                      ← NEW · register_all() entry point
    domains/
    ├── structural.rs               ← MOVED from workspace/analysis.rs
    ├── structural/
    │   ├── workspace.rs            ← MOVED from workspace/analysis/ (renamed)
    │   ├── action.rs               ← MOVED from workspace/analysis_action.rs
    │   ├── field_builder_draft.rs  ← MOVED from workspace/field_builder_draft.rs
    │   ├── build_icon_rail.rs      ← MOVED
    │   ├── build_page_header.rs    ← MOVED
    │   ├── build_pages.rs          ← MOVED
    │   ├── io_kind.rs              ← MOVED
    │   ├── kind.rs                 ← MOVED
    │   ├── new.rs                  ← MOVED
    │   ├── page_signal.rs          ← MOVED
    │   └── templates.rs            ← MOVED
    │
    ├── pm.rs                       ← MOVED from workspace/pm.rs
    ├── pm/
    │   ├── workspace.rs            ← MOVED
    │   └── build_content.rs        ← MOVED
    │
    ├── home.rs                     ← MOVED from workspace/home.rs
    ├── home/
    │   ├── workspace.rs            ← MOVED
    │   └── build_content.rs        ← MOVED
    │
    └── empty.rs                    ← MOVED from workspace/empty.rs
        empty/
        └── build_content.rs        ← MOVED

```

`src/pages/` and `src/engine.rs` and `src/components.rs` do not move —
they are already correctly separated from the workspace shell.

---

## New types

### `src/auth/role.rs`

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Role {
    StructuralEngineer,
    ProjectManager,
    Estimator,
    BimCoordinator,
    FieldInspector,
    Owner,
    Admin,
}
```

### `src/auth/capability.rs`

```rust
use std::collections::HashSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Capability {
    RunStructuralAnalysis,
    ViewCostEstimates,
    EditSchedule,
    ExportReports,
    ManageUsers,
    ViewOwnerDashboard,
}

#[derive(Debug, Clone, Default)]
pub struct CapabilitySet(pub HashSet<Capability>);

impl CapabilitySet {
    pub fn has(&self, cap: Capability) -> bool {
        self.0.contains(&cap)
    }

    pub fn from_role(role: Role) -> Self {
        use Capability::*;
        let caps: &[Capability] = match role {
            Role::StructuralEngineer => &[RunStructuralAnalysis, ExportReports],
            Role::ProjectManager     => &[EditSchedule, ExportReports],
            Role::Estimator          => &[ViewCostEstimates, ExportReports],
            Role::Admin              => &[
                RunStructuralAnalysis, ViewCostEstimates,
                EditSchedule, ExportReports, ManageUsers, ViewOwnerDashboard,
            ],
            // etc.
            _ => &[],
        };
        Self(caps.iter().copied().collect())
    }
}
```

### `src/auth/session.rs`

```rust
use super::capability::CapabilitySet;
use super::role::Role;

pub struct Session {
    pub display_name: String,
    pub role: Role,
    pub capabilities: CapabilitySet,
}

impl Session {
    pub fn new(display_name: impl Into<String>, role: Role) -> Self {
        let capabilities = CapabilitySet::from_role(role);
        Self { display_name: display_name.into(), role, capabilities }
    }

    /// Placeholder for single-user mode (no auth yet).
    pub fn guest() -> Self {
        Self::new("Guest", Role::StructuralEngineer)
    }
}
```

### `src/workspace/descriptor.rs`

```rust
use crate::auth::capability::Capability;
use crate::workspace::instance::AnyWorkspaceInstance;
use crate::workspace::workspace_id::WorkspaceId;
use infinite_db::InfiniteDb;

/// Contract every domain workspace must satisfy.
/// The shell calls these methods — domains never call back into the shell.
pub trait WorkspaceDescriptor: Send + Sync {
    /// Stable ASCII key — stored in DB, config files, URLs.
    /// Never rename after shipping.  Example: "structural_analysis"
    fn kind_id(&self) -> &'static str;

    /// Label shown in the launcher / tab strip.
    fn label(&self) -> &'static str;

    /// Glyph shown in the launcher grid.
    fn icon(&self) -> &'static str;

    /// Capabilities required to open this workspace.
    /// Empty slice = available to all users.
    fn required_capabilities(&self) -> &[Capability];

    /// Spawn a live instance.
    fn spawn(&self, id: WorkspaceId, db: &mut InfiniteDb) -> Box<dyn AnyWorkspaceInstance>;
}
```

### `src/workspace/registry.rs`

```rust
use crate::auth::capability::CapabilitySet;
use crate::workspace::descriptor::WorkspaceDescriptor;

pub struct WorkspaceRegistry {
    descriptors: Vec<Box<dyn WorkspaceDescriptor>>,
}

impl WorkspaceRegistry {
    pub fn new() -> Self {
        Self { descriptors: Vec::new() }
    }

    pub fn register(&mut self, d: Box<dyn WorkspaceDescriptor>) {
        self.descriptors.push(d);
    }

    /// Descriptors the current user may open.
    pub fn available_for<'a>(&'a self, caps: &CapabilitySet)
        -> Vec<&'a dyn WorkspaceDescriptor>
    {
        self.descriptors
            .iter()
            .filter(|d| {
                d.required_capabilities().is_empty()
                    || d.required_capabilities().iter().all(|c| caps.has(*c))
            })
            .map(|d| d.as_ref())
            .collect()
    }

    pub fn find(&self, kind_id: &str) -> Option<&dyn WorkspaceDescriptor> {
        self.descriptors.iter().find(|d| d.kind_id() == kind_id).map(|d| d.as_ref())
    }
}
```

### `src/domains.rs`

```rust
//! Registers every AEC domain workspace with the shell registry.

pub mod structural;
pub mod pm;
pub mod home;
pub mod empty;

use crate::workspace::registry::WorkspaceRegistry;

pub fn register_all(registry: &mut WorkspaceRegistry) {
    registry.register(Box::new(structural::StructuralDescriptor));
    registry.register(Box::new(pm::PmDescriptor));
    registry.register(Box::new(home::HomeDescriptor));
    registry.register(Box::new(empty::EmptyDescriptor));
}
```

---

## What changes in `WorkspaceInstance`

`WorkspaceInstance` currently is a concrete enum — every arm is a known
type. After the refactor it becomes a thin wrapper around a trait object,
or the enum survives but its variants are domain types imported from
`domains/` rather than `workspace/`:

**Option A — keep the enum, change the import paths** (lower risk, faster)

```rust
// src/workspace/instance.rs

use crate::domains::structural::StructuralWorkspace;
use crate::domains::pm::PmWorkspace;
use crate::domains::home::HomeWorkspace;
use crate::domains::empty::EmptyWorkspace;

pub enum WorkspaceInstance {
    Structural(StructuralWorkspace),   // renamed from Analysis
    Pm(PmWorkspace),
    Home(HomeWorkspace),
    Empty(EmptyWorkspace),
}
```

The enum still lives in the shell; domain types moved, not the enum.
`build_tree` and `open_workspace` match arms stay, but dispatch to
`domains::structural::build_content(ws)` etc.

**Option B — trait object** (higher abstraction, more work)

```rust
pub trait AnyWorkspaceInstance {
    fn tab(&self) -> &WorkspaceTab;
    fn header(&self) -> Option<&WorkspaceHeader>;
    fn page_tree(&self) -> Option<&PageTree>;
    fn page_tree_mut(&mut self) -> Option<&mut PageTree>;
    fn build_content(&mut self) -> Particle;
    fn handle_signal(&mut self, signal: AppSignal) -> bool;
}

pub struct WorkspaceInstance(pub Box<dyn AnyWorkspaceInstance>);
```

`build_tree` and `open_workspace` become two lines each — no match.
Adding a new domain requires zero changes to the shell.

**Recommendation:** do Option A now (pure file moves, low risk, compiles
cleanly). Do Option B when you add the third real domain workspace —
that's the natural forcing function.

---

## `WorkspaceKind` — what to do with it

`WorkspaceKind` is an enum used for:
1. Default tab titles
2. `open_workspace` dispatch
3. `build_tree` dispatch
4. Stored on `WorkspaceTab`

After Option A, WorkspaceKind keeps living but gets a new variant for
each domain and its `kind_id()` string gets wired to `descriptor.kind_id()`.
After Option B, `WorkspaceKind` can be deleted entirely — each workspace
identifies itself by its `kind_id: &'static str` stored on `WorkspaceTab`.

For now: keep `WorkspaceKind`, add `kind_id() -> &'static str` to it:

```rust
impl WorkspaceKind {
    pub fn kind_id(self) -> &'static str {
        match self {
            Self::Analysis => "structural_analysis",
            Self::PM       => "project_management",
            Self::Home     => "home",
            Self::Empty    => "empty",
        }
    }
}
```

---

## What moves to `domains/structural/`

These files move with **no logic changes** — only `use` paths update:

| From | To |
|---|---|
| `src/workspace/analysis.rs` | `src/domains/structural.rs` |
| `src/workspace/analysis/` (all files) | `src/domains/structural/` |
| `src/workspace/analysis_action.rs` | `src/domains/structural/action.rs` |
| `src/workspace/field_builder_draft.rs` | `src/domains/structural/field_builder_draft.rs` |

`AnalysisWorkspace` is renamed `StructuralWorkspace` in this move.

The `StructuralDescriptor` implementation lives in `src/domains/structural.rs`:

```rust
// src/domains/structural.rs

pub mod workspace;
pub mod action;
pub mod field_builder_draft;
pub mod build_icon_rail;
pub mod build_page_header;
pub mod build_pages;
pub mod io_kind;
pub mod kind;
pub mod new;
pub mod page_signal;
pub mod templates;

pub use workspace::StructuralWorkspace;

use crate::auth::capability::Capability;
use crate::workspace::descriptor::WorkspaceDescriptor;
use crate::workspace::workspace_id::WorkspaceId;
use infinite_db::InfiniteDb;

pub struct StructuralDescriptor;

impl WorkspaceDescriptor for StructuralDescriptor {
    fn kind_id(&self)    -> &'static str { "structural_analysis" }
    fn label(&self)      -> &'static str { "Structural Analysis" }
    fn icon(&self)       -> &'static str { "⬡" }
    fn required_capabilities(&self) -> &[Capability] {
        &[Capability::RunStructuralAnalysis]
    }
    fn spawn(&self, id: WorkspaceId, db: &mut InfiniteDb) -> Box<dyn AnyWorkspaceInstance> {
        Box::new(StructuralWorkspace::new(id, db))
    }
}
```

---

## What moves to `domains/pm/`, `domains/home/`, `domains/empty/`

Same pattern — move the files, add a trivial `*Descriptor` impl,
no logic changes required.

| From | To |
|---|---|
| `src/workspace/pm.rs` | `src/domains/pm.rs` |
| `src/workspace/pm/` | `src/domains/pm/` |
| `src/workspace/home.rs` | `src/domains/home.rs` |
| `src/workspace/home/` | `src/domains/home/` |
| `src/workspace/empty.rs` | `src/domains/empty.rs` |
| `src/workspace/empty/` | `src/domains/empty/` |

---

## `src/workspace.rs` after the refactor

Only shell types remain — domain imports are gone:

```rust
//! App shell infrastructure — tabs, signals, layout chrome.

pub mod app_shell;
pub mod app_signal;
pub mod descriptor;
pub mod instance;
pub mod registry;
pub mod screen_class;
pub mod signal;
pub mod size_class;
pub mod tab;
pub mod tab_strip;
pub mod workspace_id;

// REMOVED: analysis, analysis_action, empty, field_builder_draft,
//          header, home, kind, page, pm

pub use app_shell::AppShell;
pub use app_signal::AppSignal;
pub use descriptor::WorkspaceDescriptor;
pub use registry::WorkspaceRegistry;
pub use screen_class::ScreenClass;
pub use signal::WorkspaceSignal;
pub use tab::WorkspaceTab;
pub use tab_strip::{TabStripIO, TAB_STRIP_HEIGHT};
pub use workspace_id::WorkspaceId;
```

`WorkspaceHeader` moves to `domains/` or to a shared `chrome/` module
(see note below).

---

## Header — shared concern

`WorkspaceHeader` / `HEADER_HEIGHT` is used by both `structural` and `pm`
domains. Two options:

- Keep it in `src/workspace/header.rs` as shared shell chrome.
- Move it to `src/chrome/header.rs` when a `chrome/` module is warranted.

For now: keep at `src/workspace/header.rs`, re-export from `workspace.rs`.
It is generic enough (just a particle + trigger map) to live there.

---

## `main.rs` wiring

```rust
// src/main.rs

fn main() {
    let event_loop = EventLoop::new().expect("event loop");
    event_loop.set_control_flow(ControlFlow::Poll);

    let db = InfiniteDb::open("innovator.db").expect("open db");

    let mut registry = WorkspaceRegistry::new();
    domains::register_all(&mut registry);          // ← one new line

    let session = Session::guest();                // ← one new line (single-user)

    let mut app = AppShell::new(db, registry, session);
    event_loop.run_app(&mut app).expect("run app");
}
```

---

## Build phases

Phases are sequenced so the project compiles at the end of each phase.
Phases within a group can run in parallel in Cursor (separate files,
no shared mutable state until they connect).

### Phase 1 — Add auth types (no existing code changes)

Create new files only; nothing imports them yet.

- [ ] `src/auth.rs` — declares `pub mod role; pub mod capability; pub mod session;`
- [ ] `src/auth/role.rs` — `Role` enum
- [ ] `src/auth/capability.rs` — `Capability`, `CapabilitySet`
- [ ] `src/auth/session.rs` — `Session`

Acceptance: `cargo check` passes, no warnings in new files.

---

### Phase 2 — Shell contracts (no existing code changes)

- [ ] `src/workspace/descriptor.rs` — `WorkspaceDescriptor` trait
- [ ] `src/workspace/registry.rs` — `WorkspaceRegistry`

Add to `src/workspace.rs`:
```rust
pub mod descriptor;
pub mod registry;
```

Acceptance: `cargo check` passes.

---

### Phase 3 — Move domains (file moves + import path fixes only)

Move files in this order to keep the compiler happy at each step.

**3a — structural domain**

1. Create `src/domains/structural.rs` and `src/domains/structural/`
2. Copy (not cut) all files from `src/workspace/analysis/` → `src/domains/structural/`
3. Copy `src/workspace/analysis.rs` content → `src/domains/structural.rs`
   (rename `AnalysisWorkspace` → `StructuralWorkspace` throughout)
4. Copy `src/workspace/analysis_action.rs` → `src/domains/structural/action.rs`
5. Copy `src/workspace/field_builder_draft.rs` → `src/domains/structural/field_builder_draft.rs`
6. Fix `use` paths in every moved file (`crate::workspace::analysis_action` →
   `crate::domains::structural::action`, etc.)
7. Add `StructuralDescriptor` impl to `src/domains/structural.rs`
8. Update `src/workspace/instance.rs` to import from `domains::structural`
9. Update `src/workspace/app_shell/open_workspace.rs`
10. Update `src/workspace/app_shell/build_tree.rs`
11. Delete old files from `src/workspace/analysis*`

**3b — pm, home, empty domains** (can run in parallel with 3a)

Same pattern for each; these have fewer internal dependencies.

Acceptance: `cargo check` passes; `cargo run` opens the app unchanged.

---

### Phase 4 — `domains.rs` + `register_all`

- [ ] Create `src/domains.rs` with `pub mod structural; pub mod pm; pub mod home; pub mod empty;`
      and `register_all()`
- [ ] Add `pub mod domains;` to `src/lib.rs` (or `main.rs` if no lib.rs)
- [ ] Wire `WorkspaceRegistry` into `AppShell::new()`
- [ ] Wire `Session::guest()` into `AppShell::new()`

Acceptance: `cargo run` opens the app. Adding a new descriptor is verified
by adding a no-op `TestDescriptor` and confirming it appears in the launcher,
then deleting it.

---

### Phase 5 — Trim `src/workspace.rs`

Remove domain re-exports that no longer live there. Update any remaining
`use crate::workspace::analysis` references caught by the compiler.

Acceptance: `cargo check --all` with zero warnings on unused imports.

---

## Adding a new workspace after this refactor

The complete checklist for adding, say, `Estimating`:

1. Create `src/domains/estimating.rs`
2. Create `src/domains/estimating/` with `workspace.rs` and content files
3. Implement `WorkspaceDescriptor` for `EstimatingDescriptor` in `estimating.rs`
4. Add `pub mod estimating;` to `src/domains.rs`
5. Add `registry.register(Box::new(estimating::EstimatingDescriptor));` to `register_all()`
6. Add `Capability::ViewCostEstimates` to the roles that should see it

That is the entire change. No edits to `WorkspaceInstance`, `WorkspaceKind`,
`open_workspace`, `build_tree`, or any existing domain.

---

## Files that do NOT move

| File | Why |
|---|---|
| `src/pages/` | Already separated from the shell; `pages/` is IO content not tied to a single domain |
| `src/engine.rs` | Analysis engine is structural-domain but already in the right place as a sibling to `src/` |
| `src/components.rs` | Shared UI components; correctly lives at the root |
| `src/walls.rs` | Structural domain data; could move to `domains/structural/` in a later pass |
| `crates/hyper-ui/` | Unchanged |
| `crates/hypernode/` | Unchanged |
