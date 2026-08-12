# Spike: Composed-view Graph ownership (Phase 0 sign-off)

**Status:** signed off for Phases 1–6  
**Date:** 2026-08-10  
**Locks:** no `Company` / `Project` types; Grant-ready access surface

## Decision

`AppShell` owns a single `Graph` behind a narrow composed-view API. It does
**not** own “the company graph” or “the project graph.” Today the composed
view is the unfiltered union of everything loaded; that is an implementation
detail of Phases 1–6, not an architectural assumption.

```rust
// AppShell — private field, public accessors only
graph: Graph,

pub fn composed_view(&self) -> &Graph { &self.graph }
pub fn composed_view_mut(&mut self) -> &mut Graph { &mut self.graph }
```

Call sites that today do `ws.graph.*` thread `&Graph` / `&mut Graph` from
`shell.composed_view[_mut]()` (or receive the graph as a parameter from a
caller that already holds the shell). `StructuralWorkspace` no longer owns a
`Graph`.

## Why not per-workspace Graph

Cross-workspace `Signal` / `Stream` / `Wave` edges (Drafting pod → Analysis
pod → Construction pod) require endpoints in one traversable structure.
`SpaceClass` already distinguishes `UIView` / `Entity` / `Function` /
`Carrier` inside one graph — ownership is the only blocker.

## Explicit non-goals (do not introduce in Phases 1–6)

| Forbidden | Why |
|-----------|-----|
| `Company`, `Project`, or any privileged owner type | Recreates the `kind_id` / `WorkspaceInstance` mistake; a future “project” is just a node a `Grant` roots at |
| Filtering edges to “stay within one workspace” | Hyperedges must cross any future scope boundary freely |
| Naming the accessor `company_graph` / `project_graph` / `tenant_graph` | Bakes a tenancy model into the call surface |

## Later: Grant-aware filtering (additive)

When identity & sharing land, `composed_view` gains a `Grant`-walk under the
same signature:

1. Resolve current identity from `Session`
2. Walk `Grant` edges → union reachable scope roots
3. Return that filtered view

Existing call sites keep calling `composed_view()` — no rewrite.

## Migration sketch

1. Add private `graph` + accessors on `AppShell`
2. `load_walls` / seed dual-write insert into that graph
3. Remove `StructuralWorkspace.graph`; update handlers and page builders to
   take `&Graph` / `&mut Graph` from the shell
4. Debug-assert UIView container counts match `PageTree` / `PodList` after seed

## Checkpoint

Compiles green with identical runtime behavior except graph ownership moved.
Container dual-write and Binding edges are Phase 1–2, not this spike.
