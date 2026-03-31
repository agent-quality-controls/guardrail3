# Rust Validation Scope Plan

This directory needs one shared Rust structure pass, one shared Rust legality pass, one external typed family mapper, and one family runner/orchestrator layer.

The problem:

- Rust topology discovery and Rust file attachment are still split awkwardly
- `arch` reports legality today, but legality is not yet a shared pre-family fact set
- family mapping and family invocation are still easy to confuse
- workspace-local families still risk seeing either too much or too little

That boundary is wrong.

## Target Architecture

There should be exactly one shared answer to:

- which `Cargo.toml` roots are in scope for Rust validation
- which roots are excluded
- how overlapping roots are classified structurally
- which root-level input failures happened during root discovery
- which family-owned Rust files exist in the non-excluded tree
- how those files attach to nearby Rust topology
- which ancestor/descendant relations matter for walk-up or shadowing behavior

There should then be exactly one shared answer to:

- which discovered roots are legal
- which attached family-owned files are legally placed
- which topology/file relations are illegal but must stay visible

There should then be exactly one external answer to:

- which legal topology and owned-surface facts are routed to which family
- which global families receive repo-global owned surfaces
- which workspace-local families receive one legal workspace-local family surface at a time

Those answers should come from shared layers under:

- one shared Rust structure pass under `apps/guardrail3/crates/app/rs/`
- one shared Rust legality pass reported through `RS-ARCH`

Families should then consume legality-aware routed surfaces and only do family-specific work.

The shared layers must not hand families their own mini filesystem snapshots.

Instead:

- `ProjectTree` remains the one full repository snapshot
- shared layers derive typed owned surfaces from that snapshot
- families receive routed family-owned surfaces derived from that snapshot
- global families do not receive raw `ProjectTree`; they receive repo-global owned slices
- workspace-local families do not receive raw `ProjectTree`; they receive one legal workspace-local slice at a time

## Responsibility Split

Shared Rust structure pass must own:

- live `Cargo.toml` root discovery
- exclusions like `target/`, fixtures, and worktrees
- structural root classification and overlap facts
- root-discovery input failures
- non-excluded family-relevant file discovery
- file-to-workspace attachment facts
- attachment/relation facts such as:
  - exact root
  - nested beneath root
  - ancestor of roots
  - outside roots
  - walk-up / shadow candidates where relevant
- any shared file metadata needed by multiple families

Shared Rust legality pass must own:

- repo-global Rust topology legality
- repo-global placement legality for workspace-local family artifacts
- the legal/illegal verdicts other families rely on before routing

External typed family mapping must own:

- per-family legal-surface routing from the shared passes
- building each family's eligibility surface
- mapping shared structure and legality facts into typed family-owned views

Family runner/orchestrator must own:

- turning a family surface into one or more invocation units
- for workspace-local families: one invocation per legal workspace
- for global families: the repo-global invocation surface
- family-local parsing and rule fan-out inside one invocation

Families must not own:

- deciding which `Cargo.toml` roots are live
- deciding from raw paths which workspace, if any, owns a family-relevant file
- deciding global placement legality for workspace-local family artifacts

Families must own only:

- family-specific parsing and normalization inside already-routed legal invocation surfaces
- family-specific component discovery inside already-routed legal invocation surfaces
- per-rule input fan-out

## Test Architecture

The test split must follow the production split.

There are three different kinds of tests, and they must not be blurred together:

### Rule tests

Rule tests exist to prove one pure rule over one minimal typed input.

They must:

- live in the rule's sidecar test directory
- construct the rule's typed input directly
- test only that rule's logic

They must not:

- go through `ProjectTree`
- go through shared structure or legality
- go through mapper or runner
- depend on whether a fixture root shape is globally legal

If a rule test needs a whole repo tree to work, it is usually not a rule test.

### Family/orchestrator tests

Family tests exist to prove:

- family-local fact collection
- family-local parsing
- fan-out from family facts into rule inputs
- family-local behavior on one legal routed invocation surface

They may use routed family surfaces, mapper routes, and family-local fixtures.

But they must use legal routed shapes for workspace-local families.

That means:

- if the test is proving routed workspace-local family behavior, the fixture must contain a legal workspace root
- if the fixture shape is illegal, the test belongs in `arch` instead

### Shared legality / routing tests

Shared legality and mapper tests exist to prove:

- illegal topology is classified and reported
- illegal family-file placement is classified and reported
- legal routed workspace-local surfaces are sliced correctly
- subtree routing does not bleed across siblings

These tests belong under shared crates and `arch`, not under workspace-local
families.

### Hard rule

Workspace-local family tests must not reintroduce fake routed surfaces when the
real mapper would return no legal workspace roots.

That means:

- no synthetic test route that rebuilds illegal ownership behind the mapper's back
- no test-only bypass that makes a family see files production would never route
- no preserving standalone-package or misplaced-root fixtures just because older tests used them

When production would not route an illegal root or misplaced file to a
workspace-local family, tests must respect that.

So the migration rule is:

- illegal root or illegal placement expectations move to `arch`
- legal workspace-local content expectations stay in the local family
- pure content-rule semantics use direct typed inputs

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

Workspace-local means:

- the shared legality pass decides which workspace-local artifacts are legally placed
- the mapper builds a legal family surface for that family
- the family runner invokes the family once per legal workspace
- misplaced or illegally placed family-owned files are not judged by the local family; they are judged by the shared legality pass and reported through `RS-ARCH`

## Intended Flow

```text
project walker
  -> ProjectTree
  -> shared Rust structure facts
  -> shared Rust legality facts
  -> external typed family mapper
  -> family runner / orchestrator
  -> typed rule inputs
  -> pure rules
```

## Target Code Shape

```text
apps/guardrail3/crates/app/rs/
  structure/                          # shared Rust structure facts
    Cargo.toml
    src/
      lib.rs
      facts.rs                        # unified structure fact bundle
      discover.rs                     # eligible live Cargo root discovery
      attachment.rs                   # attach family-owned files to Rust structure

  family_selection/                   # shared family-set selection only
    Cargo.toml
    src/
      lib.rs
      selection.rs                    # requested-family resolution + enabled-family/implied-family filtering

  legality/                           # shared Rust legality derivation
    Cargo.toml
    src/
      lib.rs
      topology.rs                     # legal/illegal Cargo-root topology
      placement.rs                    # legal/illegal family-file placement
      views.rs                        # legality facts consumed by arch and mapper

  family_mapper/                      # shared typed family mapping only
    Cargo.toml
    src/
      lib.rs
      rs.rs                           # build per-family legal surfaces
      views.rs                        # narrow family-facing route view types
      scoped_files.rs                 # resolves raw staged/path scope into mapped family subsets

  runtime/                            # thin product entrypoint crate only
    Cargo.toml
    src/
      lib.rs
      context.rs
      registry.rs
      runners.rs                      # turns family surfaces into invocation units
```

## API Shape

### Structure

The shared Rust structure pass should expose one shared scope model:

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

The structure pass owns:

- which Rust roots exist
- which roots are excluded
- how roots classify structurally
- overlap facts
- root-discovery input failures
- family-owned file discovery
- file attachment facts
- ancestor/descendant relation facts relevant to tool behavior

The structure pass must not own:

- which family gets which legal roots or files
- whether a root or file placement is legal
- family enable/disable policy
- family-local parsing
- rule semantics

### Legality

The shared Rust legality pass should expose one shared legality model:

```rust
pub struct RustLegalityFacts {
    pub legal_roots: Vec<LegalRustRoot>,
    pub illegal_roots: Vec<IllegalRustRoot>,
    pub legal_family_files: Vec<LegalRustFamilyFile>,
    pub illegal_family_files: Vec<IllegalRustFamilyFile>,
}
```

The legality pass owns:

- whether a discovered root is a legal workspace root, illegal top-level package, illegal nested workspace, illegal non-member crate, or another illegal topology shape
- whether a discovered family-owned file is legally placed
- which legal roots/files local families may actually receive

The legality pass does not own:

- family-specific config parsing
- family-specific content validation
- requested-family selection

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
        legality: &'a RustLegalityFacts,
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

- per-family legal-surface routing
- building one eligibility surface per family
- per-family route projection into narrow legal views
- keeping global families global
- keeping workspace-local families local

`family_mapper` must not own:

- root discovery
- root classification
- family-relevant file discovery
- file attachment facts
- legality derivation
- family enable/disable policy
- family-local parsing
- rule semantics

## Surface vs Invocation

There are two different slices in the system and they must stay separate.

### Family surface

The mapper builds one legal family surface.

Examples:

- `arch`
  - repo-global legal and illegal Rust topology + placement facts
- `fmt`
  - repo-global legal formatting surface
- `code`
  - repo-global Rust source surface
- `clippy`
  - all legal workspace-local Clippy surfaces, grouped by workspace

This is not yet one run of the family.

### Family invocation

The family runner/orchestrator turns a family surface into actual invocations.

Examples:

- global family
  - usually one invocation over the repo-global surface
- workspace-local family
  - one invocation per legal workspace

So:

- mapper slices by family ownership and legality
- runner slices by execution unit

The general contract is:

- global families receive repo-global owned surfaces
- workspace-local families receive legal workspace-local surfaces only
- all routes are derived from the shared structure and legality passes

Examples:

- `arch`
  - all live non-excluded Rust root/topology facts plus legality results
- `fmt`
  - all non-excluded formatting config candidates
- `code`
  - all non-excluded Rust source files
- `test`
  - all non-excluded owned Rust test surfaces plus owned config/hook surfaces
- `clippy`
  - legal Clippy surfaces grouped by workspace
- `toolchain`
  - legal toolchain surfaces grouped by workspace
- `deny`
  - legal deny surfaces grouped by workspace
- `cargo`
  - legal Cargo policy surfaces grouped by workspace

The route boundary exists so:

- global families cannot silently narrow to a single routed workspace
- workspace-local families do not need to judge misplaced global files
- no family needs to rediscover legal ownership from raw tree shape

## Fact vs Judgment

The shared structure pass computes facts.
The shared legality pass computes legal/illegal judgments.
Families compute family-local content judgments.

Examples of shared facts:

- file `clippy.toml` is attached to workspace `apps/foo`
- file `clippy.toml` is nested beneath workspace `apps/foo/crates/bar`
- file `deny.toml` is outside every workspace
- file `src/lib.rs` is a non-excluded Rust source file beneath illegal nested workspace `apps/foo/crates/demo`

Examples of legality judgments:

- nested workspace `apps/foo/crates/demo` is illegal
- `clippy.toml` nested under a member crate is illegally placed
- `deny.toml` outside every workspace is illegally placed

Examples of family-local judgments:

- `clippy`: the legal workspace-root `clippy.toml` content is too weak
- `toolchain`: the legal workspace-root `rust-toolchain.toml` channel is wrong
- `code`: `src/lib.rs` violates a source rule even though the file is structurally governed

## Family Check Signatures

Families should consume injected typed family invocation.

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

- parse files supplied through routed legal invocation surfaces
- discover family-local components inside routed legal invocation surfaces when the family contract allows it
- normalize family facts
- fan out per-rule inputs

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
- judge misplaced global file placement that should already have been decided by the legality pass

In concrete terms:

1. `runtime/src/lib.rs` should build shared Rust structure facts once.
2. `runtime/src/lib.rs` should build shared Rust legality facts once.
3. `runtime/src/lib.rs` should resolve selected families once via `family_selection`.
4. `runtime/src/lib.rs` should build one external typed `FamilyMapper` once.
5. `runtime/src/runners.rs` should turn family surfaces into invocation units.
6. `arch` should report shared legality facts rather than rediscovering topology.
7. `code`, `fmt`, and `test` should consume repo-global legal surfaces from the mapper.
8. Workspace-local families should receive one legal workspace-local invocation at a time.
9. No family should infer workspace attachment or placement legality from raw paths.

## Actual Work Surface

This is the concrete implementation surface required to reach the target model.

1. Merge `placement` and `ownership` conceptually into one shared Rust structure stage.
2. Add a shared Rust legality stage that consumes structure facts and produces legal/illegal root and file placement facts.
3. Recast `RS-ARCH` as the reporting surface over that legality stage instead of the only place where legality exists.
4. Change `family_mapper` to map legal family surfaces rather than raw discovered files.
5. Change `runtime/src/runners.rs` so it owns invocation fan-out:
   - global families: repo-global invocation
   - workspace-local families: one invocation per legal workspace
6. Remove placement judgment from workspace-local families:
   - `toolchain`
   - `clippy`
   - `deny`
   - `cargo`
   - `garde`
   - `deps`
   - `release`
7. Keep only content validation in workspace-local families.
8. Keep global families repo-global:
   - `arch`
   - `fmt`
   - `code`
   - `test`

The work is done only when:

- legality exists as shared pre-family data
- mapper routes legal family surfaces
- runners fan those surfaces out into concrete invocations
- workspace-local families no longer judge repo-global placement

Concrete flow:

```text
walk_project()
  -> structure::collect(&tree)
  -> legality::collect(&tree, &structure)
  -> family_selection::resolve(...)
  -> FamilyMapper::new(&tree, &structure, config, &selected_families)
  -> family_mapper.map_rs_*()
  -> family::check(&tree, route, ...)
```

## Current Direction

The shared Rust pre-family pipeline now exists in:

- [structure/lib.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/structure/src/lib.rs)
- [placement/lib.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/placement/src/lib.rs)
- [ownership/lib.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/ownership/src/lib.rs)
- [legality/lib.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/legality/src/lib.rs)

`runtime` now:
- builds `ProjectTree`
- collects shared Rust structure
- derives shared Rust legality from that structure
- maps family surfaces from legality-aware facts
- fans those surfaces into family invocations

Current code reality:

- [structure/src/lib.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/structure/src/lib.rs) is the one shared Rust structure stage
- [placement/src/lib.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/placement/src/lib.rs) and [ownership/src/lib.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/ownership/src/lib.rs) are implementation details under that stage, not separate architectural stages
- [legality/src/lib.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/legality/src/lib.rs) is the shared legality stage
- [family_mapper/src/rs.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/family_mapper/src/rs.rs) maps family surfaces from shared legality-aware facts
- [runtime/src/runners.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/runtime/src/runners.rs) fans family surfaces into invocations
- [runtime/src/lib.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/runtime/src/lib.rs) constructs `structure` before `legality`

Required code changes from the current state:

- keep the shared structure stage family-agnostic
- keep the shared legality stage as the only source of legal/illegal root and file-placement facts
- keep mapper legality-aware
- keep runners responsible for invocation fan-out
- keep workspace-local families content-only

## Migration Plan

1. Keep `structure` as the only shared Rust discovery stage.
   It discovers roots, family-owned files, attachments, overlaps, exclusions, and read/parse-neutral structural facts.

2. Keep `legality` as the only shared Rust legality stage.
   It turns structure facts into legal and illegal root/file-placement facts before local families run.

3. Keep `RS-ARCH` as the reporting surface over shared legality, not as an ad hoc rediscovery family.

4. Keep `family_mapper/` legality-aware.
   It builds family surfaces from legal facts rather than rediscovering or re-judging ownership.

5. Keep `runtime/src/runners.rs` responsible for invocation fan-out.
   - global families: one repo-global invocation
   - workspace-local families: one invocation per legal workspace/package root

6. Keep workspace-local families content-only.
   Illegal placement, illegal topology, and impossible local surfaces belong to shared legality plus `arch`, not to local families.

7. Delete any remaining duplicate root collectors, duplicate file discovery, and duplicate ownership inference from family crates.

8. Add regressions proving:
   - structure always runs before legality
   - mapper only slices from shared legality facts
   - global families stay global
   - workspace-local families only see legal local surfaces
   - `arch` and local families agree on routed legal ownership

## Design Constraints

- Families must not decide which `Cargo.toml` roots are live Rust-validation roots.
- Families must not decide which in-scope roots they are allowed to validate.
- Families must not decide which workspace, if any, owns a family-relevant file.
- Rules must not decide root scope or root routing at all.
- The external orchestrator may route different root sets to different families, but that routing policy must live outside the family crates.
- The external orchestrator may also route different file surfaces to different families, but that owned-surface policy must live outside the family crates.
- `runtime/src/lib.rs` should stay thin; if family mapping becomes nontrivial, it belongs in `family_mapper/`, not inline in runtime.
- Shared structure must not encode family legality semantics.
- External family mapping may encode family ownership/routing policy, but not family-internal parsing semantics.
- Shared structure plus legality must be stable enough that families cannot silently diverge.

## Acceptance Criteria

This plan is complete when:

- one shared Rust structure stage discovers family-relevant files once
- one shared Rust structure stage computes attachment facts once
- one shared Rust legality stage decides legal/illegal root and file-placement facts once
- `fmt`, `code`, and `test` consume repo-global owned surfaces from the mapper
- one shared exclusion policy governs Rust root scope
- one external typed family-mapper layer feeds all Rust families that need topology or file ownership
- family `check(...)` entrypoints consume injected typed family routes instead of rediscovering roots or file ownership
- disagreements between families are about semantics, not scope
