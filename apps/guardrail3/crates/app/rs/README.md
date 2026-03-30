# Rust Validation Scope Plan

This directory needs one shared Rust-topology layer, one shared owned-surface layer, and one external typed family-mapper layer.

The problem:

- `arch` and `test` currently make their own decisions about which `Cargo.toml` roots are live
- families still have too much freedom to decide which discovered roots they validate
- exclusions and ownership rules can drift between families
- root-discovery bugs get reimplemented family by family

That boundary is wrong.

## Target Architecture

There should be exactly one shared answer to:

- which `Cargo.toml` roots are in scope for Rust validation
- which roots are excluded
- how overlapping roots are classified structurally
- which root-level input failures happened during root discovery

There should also be exactly one shared answer to:

- which non-excluded files belong to each Rust family's owned surface
- which workspace, if any, owns each such file
- which files are outside every legal workspace
- which files sit in illegal nested locations beneath a workspace

There should also be exactly one external answer to:

- which topology and owned-surface facts are routed to which family
- which global families receive repo-global owned surfaces
- which workspace-local families receive all legal workspaces plus all files relevant to them

Those answers should come from shared layers under:

- [placement](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/placement)
- a new shared owned-surface / file-ownership layer under `apps/guardrail3/crates/app/rs/`

Families should then consume shared topology and owned-surface facts and only do family-specific work.

The shared layers must not hand families their own mini filesystem snapshots.

Instead:

- `ProjectTree` remains the one full repository snapshot
- shared layers derive typed owned surfaces from that snapshot
- families receive the full `ProjectTree` plus a narrow typed route describing exactly what they may validate

## Responsibility Split

Shared Rust scope must own:

- live `Cargo.toml` root discovery
- exclusions like `target/`, fixtures, and worktrees
- structural root classification and overlap facts
- root-discovery input failures
- any shared root metadata needed by multiple families

Shared owned-surface discovery must own:

- non-excluded family-relevant file discovery
- file-to-workspace attachment facts
- file placement facts such as:
  - owned by workspace root
  - nested beneath workspace
  - outside every workspace
  - nested beneath an illegal workspace
- any shared file metadata needed by multiple families

External typed family mapping must own:

- per-family topology routing from the shared layers
- mapper-resolved file scoping for any family that needs it
- mapping shared topology and owned-surface facts into typed family orchestrator inputs

Families must not own:

- deciding which `Cargo.toml` roots are live
- deciding which in-scope roots they are allowed to validate
- deciding from raw paths which workspace, if any, owns a family-relevant file

Families must own only:

- family-specific parsing and normalization inside already-routed owned surfaces
- family-specific component discovery inside already-routed owned surfaces
- per-rule input fan-out

## Governance Classes

Rust families fall into two scope classes only:

### Global families

These govern the entire non-excluded Rust repo surface.

- `RS-ARCH`
- `RS-FMT`
- `RS-CODE`
- `RS-TEST`

Global does not mean "family-local full-tree crawl."
It means the shared layers must expose a repo-global owned surface for that family.

Examples:

- `arch` owns repo-global Rust root/topology facts over all live non-excluded `Cargo.toml` roots
- `fmt` owns repo-global formatting config placement over all non-excluded formatting config candidates
- `code` owns all non-excluded Rust source files in the repo
- `test` owns all non-excluded Rust test/runtime test surfaces in the repo

### Workspace-local families

These govern legal workspace roots rather than the whole repo at once.

- `RS-TOOLCHAIN`
- `RS-CLIPPY`
- `RS-DENY`
- `RS-CARGO`
- `RS-GARDE`
- `RS-DEPS`
- `RS-RELEASE`
- `RS-HEXARCH`
- `RS-LIBARCH`

Additional zoning still applies where relevant:

- `hexarch` is local only to app workspaces under `apps/*`
- `libarch` is local only to package workspaces under `packages/*`

Workspace-local does not mean families may ignore misplaced family-owned files outside legal workspaces.
It means:

- shared topology determines which workspace roots are legal
- the route supplies those legal workspace roots
- the route also supplies candidate family-owned files needed to catch illegal placement outside or beneath those roots

Example:

- `clippy` should receive legal workspace roots plus `clippy.toml` and relevant Cargo override candidates
- `toolchain` should receive legal workspace roots plus all `rust-toolchain*` candidates
- `deny` should receive legal workspace roots plus all `deny.toml` candidates

That keeps illegal family-owned files visible without letting each family rediscover legal Rust roots on its own.

## Intended Flow

```text
project walker
  -> ProjectTree
  -> shared Rust topology facts
  -> shared owned-surface facts
  -> external typed family mapper
  -> family orchestrator
  -> typed rule inputs
  -> pure rules
```

## Target Code Shape

```text
apps/guardrail3/crates/app/rs/
  placement/                          # shared Rust root scope only
    Cargo.toml
    src/
      lib.rs
      ids.rs                          # stable root ids / shared root references
      roots.rs                        # eligible live Cargo root discovery
      exclusions.rs                   # target/, fixtures, snapshots, worktrees
      classification.rs               # app/package/auxiliary/other/ambiguous
      overlap.rs                      # overlap / dual-ownership support facts

  ownership/                          # shared family-relevant file discovery + attachment
    Cargo.toml
    src/
      lib.rs
      kinds.rs                        # family-owned file kinds
      discover.rs                     # non-excluded relevant file discovery
      attachment.rs                   # attach files to legal workspaces / illegal locations
      rust_sources.rs                 # shared Rust source-file surface for global families
      configs.rs                      # shared config-file surface for local families

  family_selection/                   # shared family-set selection only
    Cargo.toml
    src/
      lib.rs
      selection.rs                    # requested-family resolution + enabled-family/implied-family filtering

  family_mapper/                      # shared typed family mapping only
    Cargo.toml
    src/
      lib.rs
      rs.rs                           # map_rs_* typed Rust family inputs
      views.rs                        # narrow family-facing route view types
      scoped_files.rs                 # resolves raw staged/path scope into mapped family subsets

  runtime/                            # thin product entrypoint crate only
    Cargo.toml
    src/
      lib.rs
      context.rs
      registry.rs
      runners.rs
```

## API Shape

### Placement

`placement` should expose one shared scope model:

```rust
pub struct RustRootScope {
    pub roots: Vec<RustRootFact>,
    pub overlaps: Vec<RustZoneOverlapFact>,
    pub input_failures: Vec<RustRootInputFailure>,
}

pub struct RustRootFact {
    pub id: RootId,
    pub rel_dir: String,
    pub cargo_rel_path: String,
    pub classification: RustRootClassification,
    pub arch_role: Option<RustArchRole>,
    pub app_zone_candidates: Vec<String>,
    pub package_zone_candidates: Vec<String>,
}
```

`placement` owns:

- which Rust roots exist
- which roots are excluded
- how roots classify structurally
- overlap facts
- root-discovery input failures

`placement` must not own:

- which family gets which roots
- family enable/disable policy
- family-local parsing
- rule semantics

### Ownership

`ownership` should expose one shared owned-surface model:

```rust
pub struct RustOwnedSurfaceFacts {
    pub family_files: Vec<RustFamilyFileFact>,
}

pub struct RustFamilyFileFact {
    pub family: RustValidateFamily,
    pub rel_path: String,
    pub kind: RustFamilyFileKind,
    pub attachment: WorkspaceAttachment,
}

pub enum WorkspaceAttachment {
    OwnedByWorkspaceRoot { workspace_rel: String },
    NestedBeneathWorkspace { workspace_rel: String, rel_dir: String },
    OutsideEveryWorkspace,
    BeneathIllegalWorkspace { workspace_rel: String, rel_dir: String },
}
```

`ownership` owns facts only, not policy judgments.

It answers:

- where the file is
- which legal workspace, if any, covers it
- whether it sits below a workspace rather than at the root
- whether no workspace owns it

It does not answer:

- whether that placement is allowed for `clippy`
- whether that placement is allowed for `toolchain`
- whether the enclosing workspace topology is legal overall

That judgment remains with `arch` and the target family.

### Family Selection

`family_selection` should answer only:

- which families were requested
- which families are enabled
- which implied families are added

It should not decide root routing.

### Family Mapping

`family_mapper` should expose typed family routes rather than one generic route bag.

```rust
pub struct RsRootView {
    pub rel_dir: String,
    pub cargo_rel_path: String,
}

pub struct RsArchOverlapView {
    pub app_root_rel: String,
    pub app_cargo_rel_path: String,
    pub package_root_rel: String,
    pub package_cargo_rel_path: String,
}

pub struct RsRootInputFailureView {
    pub rel_path: String,
    pub message: String,
}

pub struct RsArchRootView {
    pub root: RsRootView,
    pub classification: RustRootClassification,
    pub arch_role: Option<RustArchRole>,
    pub app_zone_candidates: Vec<String>,
    pub package_zone_candidates: Vec<String>,
}

pub struct RsArchRoute {
    pub roots: Vec<RsArchRootView>,
    pub overlaps: Vec<RsArchOverlapView>,
    pub input_failures: Vec<RsRootInputFailureView>,
}

pub struct RsWorkspaceFileView {
    pub rel_path: String,
    pub kind: RustFamilyFileKind,
    pub attachment: WorkspaceAttachment,
}

pub struct RsFmtRoute {
    pub files: Vec<RsWorkspaceFileView>,
}

pub struct RsCodeRoute {
    pub files: Vec<RsWorkspaceFileView>,
}

pub struct RsTestRoute {
    pub files: Vec<RsWorkspaceFileView>,
    pub config_files: Vec<RsWorkspaceFileView>,
}

pub struct RsWorkspaceLocalRoute {
    pub workspaces: Vec<RsRootView>,
    pub files: Vec<RsWorkspaceFileView>,
}

pub struct RsHexarchRoute {
    pub workspaces: Vec<RsRootView>,
    pub repo_root_cargo_rel_path: Option<String>,
    pub guardrail_config_rel_path: Option<String>,
}
```

Prefer explicit namespaced mapping methods:

```rust
pub struct FamilyMapper<'a> { ... }

impl<'a> FamilyMapper<'a> {
    pub fn new(
        tree: &'a ProjectTree,
        scope: &'a RustRootScope,
        ownership: &'a RustOwnedSurfaceFacts,
        config: Option<&'a GuardrailConfig>,
        selected_families: &'a RustFamilySelection,
    ) -> Self;

    pub fn map_rs_arch(&self) -> RsArchRoute;
    pub fn map_rs_fmt(&self) -> RsFmtRoute;
    pub fn map_rs_code(&self) -> RsCodeRoute;
    pub fn map_rs_hexarch(&self) -> RsHexarchRoute;
    pub fn map_rs_test(&self) -> RsTestRoute;
}
```

`family_mapper` owns:

- per-family routing of topology facts
- per-family routing of owned-file facts
- per-family route projection into narrow owned views
- keeping global families global
- keeping workspace-local families visible to misplaced relevant files

`family_mapper` must not own:

- root discovery
- root classification
- family-relevant file discovery
- file attachment facts
- family enable/disable policy
- family-local parsing
- rule semantics

## Route Shape

Routes should describe owned surfaces, not hand families a private reduced tree.

The general contract is:

- global families receive repo-global owned surfaces
- workspace-local families receive all legal workspaces plus all family-relevant files
- all routes are derived from shared discovery/topology layers

Examples:

- `arch`
  - all live non-excluded Rust root/topology facts
- `fmt`
  - all non-excluded formatting config candidates
- `code`
  - all non-excluded Rust source files
- `test`
  - all non-excluded owned Rust test surfaces plus owned config/hook surfaces
- `clippy`
  - all legal workspaces plus all Clippy-relevant files
- `toolchain`
  - all legal workspaces plus all toolchain-relevant files
- `deny`
  - all legal workspaces plus all deny-relevant files
- `cargo`
  - all legal workspaces plus the manifests and lockfiles required for Cargo policy

The route boundary exists so:

- global families cannot silently narrow to a single routed workspace
- workspace-local families cannot silently hide misplaced family-owned files
- no family needs to rediscover legal ownership from raw tree shape

## Fact vs Judgment

The shared layers compute facts.
Families and `arch` compute legality.

Examples of shared facts:

- file `clippy.toml` is attached to workspace `apps/foo`
- file `clippy.toml` is nested beneath workspace `apps/foo/crates/bar`
- file `deny.toml` is outside every workspace
- file `src/lib.rs` is a non-excluded Rust source file beneath illegal nested workspace `apps/foo/crates/demo`

Examples of judgments:

- `arch`: nested workspace `apps/foo/crates/demo` is illegal
- `clippy`: `clippy.toml` nested under a member crate is illegal
- `deny`: `deny.toml` outside every workspace is illegal
- `code`: `src/lib.rs` is still governed even though its enclosing workspace is illegal

## Family Check Signatures

Families should consume injected typed family route.

Target shape:

```rust
pub fn check(
    tree: &ProjectTree,
    route: &RsArchRoute,
) -> Vec<CheckResult>
pub fn check(
    tree: &ProjectTree,
    route: &RsHexarchRoute,
) -> Vec<CheckResult>
pub fn check(
    tree: &ProjectTree,
    route: &RsTestRoute,
    tc: &dyn ToolChecker,
) -> Vec<CheckResult>
```

Families may:

- parse files supplied through routed owned surfaces
- discover family-local components inside routed owned surfaces when the family contract allows it
- normalize family facts
- fan out per-rule inputs
- inspect candidate family-owned files supplied by the route when misplaced placement is part of the family contract

Allowed family input contents:

- routed narrow root views
- routed narrow overlap views
- routed narrow input-failure views
- mapper-resolved file subsets
- selection-decided family mode flags already decided outside the family

Families must not:

- discover live Rust roots
- exclude roots
- decide which routed roots they validate
- rerun root routing locally
- receive the whole shared scope just because it is convenient
- invent a weaker visibility universe that makes illegal family-owned files disappear

In concrete terms:

1. `runtime/src/lib.rs` should build shared Rust topology facts once.
2. `runtime/src/lib.rs` should build shared owned-surface facts once.
3. `runtime/src/lib.rs` should resolve selected families once via `family_selection`.
4. `runtime/src/lib.rs` should build one external typed `FamilyMapper` once.
5. Families should receive injected typed family routes.
6. `arch` should consume typed mapped route instead of collecting roots itself.
7. `code`, `fmt`, and `test` should consume repo-global owned surfaces from the mapper rather than narrowing to routed roots.
8. Workspace-local families should receive all legal workspaces plus all relevant files for that family.
9. No family should infer workspace attachment from raw paths.

Concrete flow:

```text
walk_project()
  -> placement::collect(&tree)
  -> ownership::collect(&tree, &placement)
  -> family_selection::resolve(...)
  -> FamilyMapper::new(&tree, &placement, &ownership, config, &selected_families)
  -> family_mapper.map_rs_*()
  -> family::check(&tree, route, ...)
```

## Current Direction

The shared root-topology seed already exists in:

- [placement/lib.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/placement/src/lib.rs)
- [placement/roots.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/placement/src/roots.rs)
- [placement/classification.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/placement/src/classification.rs)
- [placement/overlap.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/placement/src/overlap.rs)

`arch` and `test` now consume routed roots from shared placement scope. Other Rust families are still on older direct `ProjectTree` entrypoints, and runtime-level applicability filtering has not been fully collapsed yet.

Current code reality:

- [placement/src/lib.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/placement/src/lib.rs) only models root topology facts
- [family_mapper/src/rs.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs) currently routes mostly root lists and optional scoped files
- [family_mapper/src/views.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/family_mapper/src/views.rs) does not yet model owned file views or workspace attachment facts
- `RsClippyRoute`, `RsDepsRoute`, `RsLibarchRoute`, and `RsToolchainRoute` are currently aliases over the same simple root route shape
- `RsCodeRoute` and `RsGardeRoute` are currently aliases over a scoped-root/source route shape

That is sufficient for the current routed-root families, but it is not sufficient for the target architecture described above.

Required code changes from the current state:

- keep `placement` focused on topology
- add a new shared `ownership` crate/module for family-relevant file discovery and attachment
- extend `FamilyMapper::new(...)` to receive both topology facts and owned-surface facts
- replace the current route aliases in `views.rs` with explicit route structs that can carry:
  - legal workspace roots
  - repo-global file surfaces
  - relevant local-family files
  - attachment metadata for each file
- update `map_rs_code()` and `map_rs_test()` away from routed-root + scoped-file semantics into repo-global owned surfaces
- update `map_rs_clippy()`, `map_rs_toolchain()`, `map_rs_deny()`, `map_rs_cargo()`, `map_rs_garde()`, `map_rs_deps()`, and `map_rs_release()` to route:
  - all legal workspaces
  - all relevant files for that family
  - shared attachment metadata
- keep `hexarch` and `libarch` local, but stop using them to own generic workspace-topology legality once `arch` absorbs those rules

## Migration Plan

1. Keep `placement` as the shared Rust topology layer.
   It should stay family-agnostic.
   It should describe live roots, overlaps, exclusions, and root-discovery failures.

2. Add a shared owned-surface layer under `ownership/`.
   It should:
   - discover family-relevant files across the non-excluded tree
   - attach those files to legal workspaces or to illegal/outside positions
   - remain fact-only, not semantic

3. Define a shared family-selection layer under `family_selection/`.
   It owns requested-family resolution, enabled-family filtering, and implied-family expansion.

4. Redefine the external typed family-mapper layer under `family_mapper/`.
   [lib.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/runtime/src/lib.rs) should call it, not implement it inline.
   It should map shared topology facts and shared owned-surface facts into typed family orchestrator inputs.
   Families must not invent their own routed universe.

5. Refactor `arch` to consume injected typed mapped route instead of calling `placement::collect(...)` internally.

6. Refactor global families to consume repo-global owned surfaces:
   - `fmt`
   - `code`
   - `test`

7. Refactor workspace-local families to consume:
   - all legal workspaces
   - all relevant files for that family
   - shared attachment metadata

8. Delete duplicate root collectors, duplicate file discovery, and duplicate ownership inference from families after the shared path is live.

9. Add regressions proving:
   - global families do not narrow to one workspace
   - workspace-local families still see misplaced relevant files
   - families no longer infer ownership from raw paths
   - `arch` and local families agree on workspace attachment facts

## Design Constraints

- Families must not decide which `Cargo.toml` roots are live Rust-validation roots.
- Families must not decide which in-scope roots they are allowed to validate.
- Families must not decide which workspace, if any, owns a family-relevant file.
- Rules must not decide root scope or root routing at all.
- The external orchestrator may route different root sets to different families, but that routing policy must live outside the family crates.
- The external orchestrator may also route different file surfaces to different families, but that owned-surface policy must live outside the family crates.
- `runtime/src/lib.rs` should stay thin; if family mapping becomes nontrivial, it belongs in `family_mapper/`, not inline in runtime.
- `family_selection/` and `family_mapper/` are separate because selecting a family set is not the same problem as routing roots into typed family routes.
- Shared scope must not encode family semantics.
- Shared owned-surface discovery must not encode family legality semantics.
- External family mapping may encode family ownership/routing policy, but not family-internal parsing semantics.
- Shared scope must be stable enough that families cannot silently diverge.

## Acceptance Criteria

This plan is complete when:

- `arch` no longer performs family-local live-root discovery
- `arch` no longer performs family-local root routing
- one shared owned-surface layer discovers family-relevant files once
- one shared owned-surface layer computes workspace attachment facts once
- `fmt`, `code`, and `test` consume repo-global owned surfaces from the mapper
- one shared exclusion policy governs Rust root scope
- one external typed family-selection layer chooses families once
- one external typed family-mapper layer feeds all Rust families that need topology or owned-file ownership
- family `check(...)` entrypoints consume injected typed family routes instead of rediscovering roots or file ownership
- disagreements between families are about semantics, not scope
