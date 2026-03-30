# Rust Validation Scope Plan

This directory needs one shared Rust-root scope layer and one external typed family-mapper layer.

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

There should also be exactly one external answer to:

- which in-scope roots are routed to which family
- which root-level scope filters are applied before a family runs

That answer should come from a shared layer under:

- [placement](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/placement)

Families should then consume shared root scope and only do family-specific work.

## Responsibility Split

Shared Rust scope must own:

- live `Cargo.toml` root discovery
- exclusions like `target/`, fixtures, and worktrees
- structural root classification and overlap facts
- root-discovery input failures
- any shared root metadata needed by multiple families

External typed family mapping must own:

- per-family root routing from the shared scope
- mapper-resolved file scoping for any family that needs it
- mapping shared root scope into typed family orchestrator inputs

Families must not own:

- deciding which `Cargo.toml` roots are live
- deciding which in-scope roots they are allowed to validate

Families must own only:

- family-specific parsing and normalization inside already-routed roots
- family-specific component discovery inside an already-routed root
- per-rule input fan-out

## Intended Flow

```text
project walker
  -> ProjectTree
  -> shared Rust root scope
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

pub struct RsHexarchRoute {
    pub roots: Vec<RsRootView>,
    pub scoped_files: Option<BTreeSet<String>>,
    pub repo_root_cargo_rel_path: Option<String>,
    pub guardrail_config_rel_path: Option<String>,
}

pub struct RsTestRoute {
    pub roots: Vec<RsRootView>,
    pub scoped_files: Option<BTreeSet<String>>,
}
```

Prefer explicit namespaced mapping methods:

```rust
pub struct FamilyMapper<'a> { ... }

impl<'a> FamilyMapper<'a> {
    pub fn new(
        tree: &'a ProjectTree,
        scope: &'a RustRootScope,
        config: Option<&'a GuardrailConfig>,
        selected_families: &'a RustFamilySelection,
        scoped_files: Option<&'a BTreeSet<String>>,
    ) -> Self;

    pub fn map_rs_arch(&self) -> RsArchRoute;
    pub fn map_rs_hexarch(&self) -> RsHexarchRoute;
    pub fn map_rs_test(&self) -> RsTestRoute;
}
```

`family_mapper` owns:

- per-family root routing
- per-family route projection into narrow owned views
- mapper-resolved file scoping for any family that needs it

`family_mapper` must not own:

- root discovery
- root classification
- family enable/disable policy
- family-local parsing
- rule semantics

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

- parse files inside routed roots
- discover family-local components inside routed roots
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

In concrete terms:

1. `runtime/src/lib.rs` should build shared Rust scope once.
2. `runtime/src/lib.rs` should resolve selected families once via `family_selection`.
3. `runtime/src/lib.rs` should build one external typed `FamilyMapper` once.
4. Families should receive injected typed family routes.
5. `arch` should consume typed mapped route instead of collecting roots itself.
6. `test` should consume typed mapped route instead of collecting roots itself.

Concrete flow:

```text
walk_project()
  -> placement::collect(&tree)
  -> family_selection::resolve(...)
  -> FamilyMapper::new(&tree, &scope, config, &selected_families, scoped_files)
  -> family_mapper.map_rs_*()
  -> family::check(&tree, route, ...)
```

## Current Direction

The shared root-scope seed already exists in:

- [placement/lib.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/placement/src/lib.rs)
- [placement/roots.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/placement/src/roots.rs)
- [placement/classification.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/placement/src/classification.rs)
- [placement/overlap.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/placement/src/overlap.rs)

`arch` and `test` now consume routed roots from shared placement scope. Other Rust families are still on older direct `ProjectTree` entrypoints, and runtime-level applicability filtering has not been fully collapsed yet.

## Migration Plan

1. Define a shared Rust root-scope API in `placement`.
   Keep it family-agnostic.
   It should describe live roots, overlaps, exclusions, and root-discovery failures.

2. Define a shared family-selection layer under `family_selection/`.
   It owns requested-family resolution, enabled-family filtering, and implied-family expansion.

3. Define an external typed family-mapper layer under `family_mapper/`.
   [lib.rs](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/runtime/src/lib.rs) should call it, not implement it inline.
   It should map shared scoped roots to typed family orchestrator inputs using narrow owned route views.
   Families must not invent their own routed universe.

4. Refactor `arch` to consume injected typed mapped route instead of calling `placement::collect(...)` internally.

5. Refactor `test` to consume injected typed mapped route instead of doing family-local `Cargo.toml` discovery.

6. Delete duplicate root collectors and duplicate root-routing logic from families after the shared path is live.

7. Add regressions proving `arch` and `test` agree on:
   - which roots are in scope
   - which roots are routed to them
   - which roots are excluded
   - which nested roots are valid
   - which root-discovery failures are fail-closed

## Design Constraints

- Families must not decide which `Cargo.toml` roots are live Rust-validation roots.
- Families must not decide which in-scope roots they are allowed to validate.
- Rules must not decide root scope or root routing at all.
- The external orchestrator may route different root sets to different families, but that routing policy must live outside the family crates.
- `runtime/src/lib.rs` should stay thin; if family mapping becomes nontrivial, it belongs in `family_mapper/`, not inline in runtime.
- `family_selection/` and `family_mapper/` are separate because selecting a family set is not the same problem as routing roots into typed family routes.
- Shared scope must not encode family semantics.
- External family mapping may encode family ownership/routing policy, but not family-internal parsing semantics.
- Shared scope must be stable enough that families cannot silently diverge.

## Acceptance Criteria

This plan is complete when:

- `arch` no longer performs family-local live-root discovery
- `arch` no longer performs family-local root routing
- `test` no longer performs family-local live-root discovery
- `test` no longer performs family-local root routing
- one shared exclusion policy governs Rust root scope
- one external typed family-selection layer chooses families once
- one external typed family-mapper layer feeds all Rust families that need root ownership
- family `check(...)` entrypoints consume injected typed family routes instead of rediscovering roots
- disagreements between families are about semantics, not scope
