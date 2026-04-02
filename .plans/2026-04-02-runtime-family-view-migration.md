# Runtime/Runner Migration Plan: Remove tree, introduce FamilyView

**Date:** 2026-04-02 (updated after test attack)

## Current state

Pipeline is leak-free up to mapper:
```
walker -> structure(tree) -> legality(structure) -> mapper(&legality)
```

But the runtime bypasses the pipeline: `RustRunContext` holds `&ProjectTree`
and all 16 runners pass it to surface-building functions. Families get raw
tree access through the surface. Code doesn't compile (intentional break).

## Target state

```
walker -> structure(tree) -> legality(structure) -> mapper(&legality)
                                                         |
                                                    per family:
                                                   route + FamilyView
                                                         |
                                                      family
```

No tree in runtime context. No tree in runners. No tree in families.

## What is FamilyView

FamilyView replaces RsProjectSurface. Same shape — scoped structure map +
content map — but built from LegalityFacts, not from the raw tree.

```rust
pub struct FamilyView {
    root: PathBuf,
    structure: BTreeMap<String, DirEntry>,
    content: BTreeMap<String, String>,
    /// Root rels this view is scoped to — for abs_path validation.
    scope_roots: Vec<String>,
}
```

Methods:
- `file_content(rel) -> Option<&str>` — read cached content within scope
- `file_exists(rel) -> bool` — check file in scoped structure
- `dir_exists(rel) -> bool` — check dir in scoped structure
- `dir_contents(rel) -> Option<&DirEntry>` — get dir children in scope
- `abs_path(rel) -> Option<PathBuf>` — ONLY for paths within scope roots.
  Returns None for out-of-scope paths. Prevents filesystem bypass.
- `matching_dir_rels(pattern) -> Vec<String>` — glob within scope
- `join_rel(parent, child) -> String` — path utility (static, no state)

### abs_path scoping (addresses filesystem bypass)

Families call `abs_path()` + `guardrail3_shared_fs::read_file_err()` to read
source files (.rs) from disk. Without scoping, families can construct paths to
ANY file and read it.

Fix: `abs_path(rel)` validates `rel` is within `scope_roots` before returning
`Some(root.join(rel))`. Out-of-scope paths return `None`. Families can only
read files within their legal roots.

## Where FamilyView lives

New crate: `app-rs-family-view`

Contains:
- `FamilyView` type and its impl
- Re-exports `DirEntry` from domain-project-tree (transitive dep — harmless,
  DirEntry is just a data struct, families can't construct ProjectTree from it)

Route types stay in `family_mapper` for now. Families import routes from
family_mapper and FamilyView from family-view. Eventually routes move to
family-view too.

Families do NOT depend on:
- domain-project-tree (no ProjectTree access)
- app-rs-structure (no StructureFacts)
- app-rs-legality (no LegalityFacts)

## How FamilyView is built

Builder function in runtime (same logic as old `from_route_scope`):

```rust
fn build_family_view(
    legality: &LegalityFacts,
    root_rels: &[String],
    extra_file_rels: &[String],
    extra_dir_rels: &[String],         // for hooks
    scoped_files: Option<&BTreeSet<String>>,  // for test/code
) -> FamilyView
```

Reads from `legality.structure().dir_structure()` and
`legality.structure().content()`. Filters to root_rels. Applies extra files,
extra dirs, and scoped_files filtering. Same algorithm as old from_route_scope.

## RustRunContext changes

```rust
pub(crate) struct RustRunContext<'a> {
    pub(crate) legality: &'a RustLegalityFacts,  // NEW: replaces tree
    pub(crate) mapper: &'a FamilyMapper<'a>,
    // fs, path, tc, thorough stay for families that need them
}
```

`tree` field removed. Runtime creates tree, passes to structure::collect (which
consumes it). After that, only legality survives. Context holds &legality.

## Pre-pipeline tree usage

`load_config(&tree)` (line 78) and `family_selection::resolve(&tree, ...)` 
(line 84) happen BEFORE `structure::collect(tree)` (line 87). These borrow
the tree temporarily. The borrows end before the move. This is fine — Rust
allows borrows that end before a move.

Runtime MUST keep its `domain-project-tree` Cargo dependency because it
creates the tree from the walker. It just stops storing/passing it after
structure consumes it.

## Special cases

### Code family (global, needs all .rs files)
FamilyView is scoped to all legal roots (global family gets all roots).
Code iterates `view.dir_contents()` recursively within those roots to find
.rs files. Same as current behavior but scoped to legal roots only.

### Hooks families (no route, hardcoded paths)
Build FamilyView with extra_file_rels (hardcoded hook paths) and
extra_dir_rels (hardcoded hook directories). No root_rels needed.
The view includes only hook-related files/dirs from the legal structure.
hooks-shared::check(fs, path, &FamilyView, tc) — signature keeps fs/path/tc.

### Test family (needs toolchecker + scoped files)
build_family_view passes scoped_files from route.
check(&FamilyView, &route, tc)

### Release family (needs thorough flag)
check(&FamilyView, &route, tc, thorough)

## Migration steps

1. Create `app-rs-family-view` crate with FamilyView type
2. Implement `build_family_view` in runtime (port from_route_scope logic)
3. Remove `tree` from RustRunContext, add `legality`
4. Rewrite each runner: route from mapper, view from build_family_view
5. Change each family check() to take &FamilyView instead of &RsProjectSurface
6. Remove RsProjectSurface references everywhere
7. Verify: no family crate depends on domain-project-tree

## LegalityFacts scoping note

LegalityFacts carries the FULL structure data (all classified roots, including
fixture classifications). The scoping to legal-only happens in
`build_family_view`, not in LegalityFacts itself. The builder uses the mapper's
route (which comes from legal roots) to scope the view. LegalityFacts remains
the full truth; FamilyView is the scoped slice.
