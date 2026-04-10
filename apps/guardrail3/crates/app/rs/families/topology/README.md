# RS-TOPOLOGY

Rust root placement and topology ownership family.

Current source of truth for this family:

- this README for family-local behavior and rule boundaries
- [apps/guardrail3/crates/app/rs/README.md](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/README.md) for shared `placement` / `FamilyMapper` wiring
- [.plans/todo/checks/rs/topology.md](/Users/tartakovsky/Projects/websmasher/guardrail3/.plans/todo/checks/rs/topology.md) for the live rule inventory

Older handoffs and migration briefs are historical only.

This family enforces a target state where Rust topology ownership is:

- repo-global
- live-root-only
- emitted once
- explicit about auxiliary roots
- fail-closed on required inputs
- shared across topology families through one placement substrate

## What This Family Prevents

- misplaced Rust roots drifting outside governed topology zones
- `hexarch` and `arch` disagreeing about root ownership
- fixture `Cargo.toml` files polluting live topology results
- app/package dual ownership and illegal nesting
- dead scoped `topology` config that looks supported but does nothing

## Owned Surface

`RS-TOPOLOGY` owns only repo-global Rust root placement.

It evaluates:

- eligible live Rust `Cargo.toml` roots
- root-to-zone classification
- declared auxiliary Rust roots
- root-to-family ownership
- app/package overlap and dual ownership
- exact workspace-membership equivalence for governed workspaces
- global Rust topology enablement

It does not own:

- app-internal structure
- package-internal structure
- generic Cargo policy

Those belong to:

- `RS-HEXARCH`
- `RS-ARCH`
- `RS-CARGO`

## Global-Only Family

`topology` is global-only.

The only valid config surface is:

```toml
[rust.checks]
topology = true
```

Forbidden:

```toml
[rust.apps.backend.checks]
topology = true

[rust.packages.checks]
topology = true
```

`RS-TOPOLOGY` must not have per-app or per-package enablement.
Runtime, config validation, help text, and generated config must agree on that.

## Shared Placement Substrate

Rust root placement must not live privately inside this family crate.

The shared Rust scope and typed family-mapper contract are documented in:

- [apps/guardrail3/crates/app/rs/README.md](/Users/tartakovsky/Projects/websmasher/guardrail3/apps/guardrail3/crates/app/rs/README.md)

For `RS-TOPOLOGY`, the important contract is:

- live Rust root scope comes from shared `placement`
- family routing comes from shared `FamilyMapper`
- `RS-TOPOLOGY` is the family that emits repo-global placement findings and exact workspace-membership topology findings over that shared scope

`RS-HEXARCH` and `RS-ARCH` may consume the same placement facts, but must not emit misplaced-root findings.

## Live Root Discovery Domain

This family does not judge every `Cargo.toml` in the repository.
It judges every eligible live Rust root.

Eligible live Rust roots are discovered from `ProjectTree` and include:

- repository root `Cargo.toml`, if present
- any directory containing `Cargo.toml`

Excluded from live topology discovery:

- any root under `**/tests/fixtures/**`
- any root under `**/tests/snapshots/**`
- any root under `target/**`
- any root under `.claude/worktrees/**`

These excluded roots are test corpora or build outputs, not live topology.

The exclusion rule is structural and path-based.
It is not inferred from file contents.

## Classification Model

Every eligible live Rust root is classified as exactly one of:

- `app`
  - root path is inside `apps/<name>/...`
- `package`
  - root path is inside `packages/<name>/...`
- `auxiliary`
  - root is outside governed zones but explicitly declares:
    - `[package.metadata.guardrail3] topology_role = "auxiliary"`
    - or `[workspace.metadata.guardrail3] topology_role = "auxiliary"`
- `other`
  - root path is outside both governed zones and does not declare an auxiliary role
- `ambiguous`
  - root path matches both an app zone and a package zone

Classification is segment-based, not top-level-only.

That means:

- `apps/backend/crates/domain/types` is still `app`
- `packages/shared/crates/domain/types` is still `package`
- `apps/backend/packages/shared` is `ambiguous`
- `packages/shared/apps/tooling` is `ambiguous`

Same-zone nesting is not ambiguity by itself.
It remains a same-zone root and may later matter to the owning family.

## Reporting Model

`RS-TOPOLOGY` is the only family that emits repo-global misplaced-root findings.

That means:

- `RS-TOPOLOGY` reports `other`
- `RS-HEXARCH` does not report `other`
- `RS-ARCH` does not report `other`
- `RS-TOPOLOGY` does not report declared `auxiliary` roots as misplaced

This avoids:

- duplicate signaling
- drift between topology families
- hidden placement policy in app/package-local families

## Fail-Closed Inputs

This family depends on:

- readable eligible `Cargo.toml` roots
- readable `guardrail3.toml`
- readable directory structure from `ProjectTree`

This family must fail closed when any required active input is unreadable or unparsable.

Examples:

- eligible root `Cargo.toml` exists in the tree but cached content is missing
- `guardrail3.toml` exists in the tree but cached content is missing
- `guardrail3.toml` content is malformed TOML
- governed app/package `Cargo.toml` is readable but malformed
- governed app/package `Cargo.toml` declares `topology_role`, which is invalid there
- malformed eligible live out-of-zone `Cargo.toml`

Absence and unreadability are different states.
Unreadable-present inputs must not be treated as absent.

## Family Implementation Shape

Inside this guardrail family itself:

- the family container lives under `families/topology/`
- `crates/runtime/src/lib.rs` orchestrates only
- `crates/runtime/src/facts.rs` binds routed placement facts with config resolution
- `crates/runtime/src/inputs.rs` produces minimal per-rule inputs
- `crates/assertions/src/*.rs` owns reusable semantic assertions
- `test_support/src/lib.rs` owns only generic `ProjectTree` builders/helpers
- exactly one `RS-TOPOLOGY-*` rule ID per production file
- exactly one rule-specific sidecar test module directory per production rule file
- live-root scope comes from shared `placement`
- root routing comes from shared `FamilyMapper`

Target family tree:

```text
apps/guardrail3/crates/app/rs/families/topology/
  crates/
    runtime/
      Cargo.toml
      src/
        lib.rs
        facts.rs
        inputs.rs
        rs_topology_01_root_classification.rs
        rs_topology_01_root_classification_tests/
          mod.rs
        rs_topology_02_no_misplaced_roots.rs
        rs_topology_02_no_misplaced_roots_tests/
          mod.rs
        rs_topology_03_no_dual_ownership.rs
        rs_topology_03_no_dual_ownership_tests/
          mod.rs
        rs_topology_04_no_zone_overlap.rs
        rs_topology_04_no_zone_overlap_tests/
          mod.rs
        rs_topology_05_scoped_topology_config_forbidden.rs
        rs_topology_05_scoped_topology_config_forbidden_tests/
          mod.rs
        rs_topology_06_owner_family_enablement_coherence.rs
        rs_topology_06_owner_family_enablement_coherence_tests/
          mod.rs
        rs_topology_07_required_inputs_fail_closed.rs
        rs_topology_07_required_inputs_fail_closed_tests/
          mod.rs
        rs_topology_08_auxiliary_roots_declared.rs
        rs_topology_08_auxiliary_roots_declared_tests/
          mod.rs
    assertions/
      Cargo.toml
      src/
        lib.rs
        rs_topology_01_root_classification.rs
        rs_topology_02_no_misplaced_roots.rs
        rs_topology_03_no_dual_ownership.rs
        rs_topology_04_no_zone_overlap.rs
        rs_topology_05_scoped_topology_config_forbidden.rs
        rs_topology_06_owner_family_enablement_coherence.rs
        rs_topology_07_required_inputs_fail_closed.rs
        rs_topology_08_auxiliary_roots_declared.rs
  test_support/
    Cargo.toml
    src/
      lib.rs
```

Forbidden:

- private family-local root discovery that `hexarch` cannot reuse
- local reimplementation of root routing that should come from `FamilyMapper`
- grouped rule files
- grouped family-wide test files
- fixture manifests treated as live roots

## Rules

### RS-TOPOLOGY-01

Every eligible live Rust root must classify unambiguously.

Severity:

- `Error`

Detection:

- collect eligible live roots from the shared placement substrate
- compute app-zone candidates from `apps/<name>`
- compute package-zone candidates from `packages/<name>`
- classify:
  - one app candidate and zero package candidates => `app`
- zero app candidates and one package candidate => `package`
- zero candidates and declared auxiliary role => `auxiliary`
- zero candidates => `other`
- any root with both app and package candidates => `ambiguous`
- emit on `ambiguous`

Must not inspect:

- excluded fixture/snapshot/output roots

### RS-TOPOLOGY-02

No eligible live Rust root may live in unexpected `other` when topology enforcement is active.

Severity:

- `Error`

Activation:

- active when global `topology` is enabled and at least one owner family is enabled:
  - `arch`
  - `hexarch`
- inactive when both owner families are disabled

Detection:

- start from eligible live roots
- reuse `RS-TOPOLOGY-01` classification facts
- for each root classified as `other`:
  - if reporting is inactive, emit one inventory info result explaining that misplaced-root enforcement is suppressed
  - otherwise emit one misplaced-root finding

Do not emit for:

- declared `auxiliary` roots
- structurally excluded roots

This rule owns repo-global misplaced-root reporting exclusively.

### RS-TOPOLOGY-03

No eligible live Rust root may be owned by both topology families.

Severity:

- `Error`

Detection:

- start from eligible live roots
- compute owner-family candidates from classification facts
- if a root has:
  - at least one app-zone candidate
  - and at least one package-zone candidate
  then emit dual ownership

This is about family ownership, not same-zone nesting.

### RS-TOPOLOGY-04

App-zone roots and package-zone roots must not overlap illegally.

Severity:

- `Error`

Detection:

- start from eligible live roots
- derive actual app-root/package-root pairs from zone candidates on discovered live roots
- emit if an app-root directory and a package-root directory have a direct ancestor/descendant relationship
- sort findings deterministically by app root path then package root path

This rule is layout-level, not root-level:

- nested cross-zone shapes such as `apps/<app>/packages/<pkg>` and `packages/<pkg>/apps/<app>` should emit:
  - `RS-TOPOLOGY-01` for ambiguous root classification
  - `RS-TOPOLOGY-03` for dual ownership
  - `RS-TOPOLOGY-04` for the illegal app/package containment pair itself

Do not emit for:

- sibling app/package roots
- same-zone nesting

### RS-TOPOLOGY-05

Scoped `topology` config is forbidden.

Severity:

- `Error`

Detection:

- parse `guardrail3.toml`
- require `topology` to be read only from `[rust.checks]`
- emit if scoped `topology` config appears under:
  - `[rust.apps.<name>.checks]`
  - `[rust.packages.checks]`

### RS-TOPOLOGY-06

Owned roots must stay coherent with their owner family enablement.

Severity:

- `Error`

Detection:

- for governed roots:
  - app-owned roots require effective `hexarch = true`
  - package-owned roots require effective `arch = true`
  - app-scoped `hexarch` overrides win over the global `hexarch` default for every root under that app

### RS-TOPOLOGY-07

Required topology inputs must fail closed.

Severity:

- `Error`

Detection:

- if required config or placement inputs are unreadable or unparsable, emit fail-closed errors
- active required inputs include:
  - readable eligible live `Cargo.toml`
  - parseable governed app/package `Cargo.toml`
  - governed app/package `Cargo.toml` that does not declare `topology_role`
  - readable `guardrail3.toml`
  - parseable `guardrail3.toml`
  - parseable auxiliary-role metadata in eligible live `Cargo.toml`

### RS-TOPOLOGY-08

Declared auxiliary roots must be surfaced explicitly.

Severity:

- `Info`

Detection:

- start from eligible live roots
- emit for roots outside governed zones that explicitly declare:
  - `[package.metadata.guardrail3] topology_role = "auxiliary"`
  - or `[workspace.metadata.guardrail3] topology_role = "auxiliary"`
- inventory-mark these confirmations so the exemption stays visible without masquerading as an error

Do not emit for:

- governed roots under `apps/*` or `packages/*`
- malformed or invalid auxiliary metadata, which belongs to `RS-TOPOLOGY-07`

## Test Expectations

Every rule should have:

- golden pass
- direct attack case
- exact hit set
- exact non-hit set
- fail-closed coverage where applicable
- exact severity assertions

The family must also have at least one runtime-surface test proving:

- `--family topology` survives CLI parsing
- explicit `--family topology` still runs when `[rust.checks] topology = false`
- runtime dispatch runs `RS-TOPOLOGY`
- report section name is `topology`
- generated config includes global `topology = true`
- generated config does not place `topology` under app/package-scoped checks

Declared auxiliary roots should also be surfaced in reports as informational results so the live CLI output makes the exemption explicit rather than silently hiding it.
