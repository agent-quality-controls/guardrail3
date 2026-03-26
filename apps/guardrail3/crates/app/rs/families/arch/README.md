# RS-ARCH

Rust root placement and architecture ownership family.

This family enforces a target state where Rust architecture ownership is:

- repo-global
- live-root-only
- emitted once
- explicit about auxiliary roots
- fail-closed on required inputs
- shared across architecture families through one placement substrate

## What This Family Prevents

- misplaced Rust roots drifting outside governed architecture zones
- `hexarch` and `libarch` rediscovering roots differently and disagreeing
- fixture `Cargo.toml` files polluting live architecture results
- app/package dual ownership and illegal nesting
- dead scoped `arch` config that looks supported but does nothing

## Owned Surface

`RS-ARCH` owns only repo-global Rust root placement.

It evaluates:

- eligible live Rust `Cargo.toml` roots
- root-to-zone classification
- declared auxiliary Rust roots
- root-to-family ownership
- app/package overlap and dual ownership
- global Rust architecture enablement

It does not own:

- app-internal structure
- package-internal structure
- workspace member completeness inside one app or package
- generic Cargo policy

Those belong to:

- `RS-HEXARCH`
- `RS-LIBARCH`
- `RS-CARGO`

## Global-Only Family

`arch` is global-only.

The only valid config surface is:

```toml
[rust.checks]
arch = true
```

Forbidden:

```toml
[rust.apps.backend.checks]
arch = true

[rust.packages.checks]
arch = true
```

`RS-ARCH` must not have per-app or per-package enablement.
Runtime, config validation, help text, and generated config must agree on that.

## Shared Placement Substrate

Rust root placement must not live privately inside this family crate.

Target shared shape:

```text
crates/
  app/
    rs/
      placement/                         # shared Rust root-placement substrate
        Cargo.toml                       # shared by arch, hexarch, and later libarch
        src/
          lib.rs                         # exports live-root discovery and placement facts
          roots.rs                       # eligible live Cargo roots only
          classification.rs              # app/package/other/ambiguous classification
          overlap.rs                     # app/package overlap and dual-ownership facts
```

`RS-ARCH` uses that substrate and emits repo-global placement findings.

`RS-HEXARCH` and `RS-LIBARCH` may consume the same placement facts, but must not emit misplaced-root findings.

## Live Root Discovery Domain

This family does not judge every `Cargo.toml` in the repository.
It judges every eligible live Rust root.

Eligible live Rust roots are discovered from `ProjectTree` and include:

- repository root `Cargo.toml`, if present
- any directory containing `Cargo.toml`

Excluded from live architecture discovery:

- any root under `**/tests/fixtures/**`
- any root under `**/tests/snapshots/**`
- any root under `target/**`
- any root under `.claude/worktrees/**`

These excluded roots are test corpora or build outputs, not live architecture.

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
    - `[package.metadata.guardrail3] arch_role = "auxiliary"`
    - or `[workspace.metadata.guardrail3] arch_role = "auxiliary"`
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

`RS-ARCH` is the only family that emits repo-global misplaced-root findings.

That means:

- `RS-ARCH` reports `other`
- `RS-HEXARCH` does not report `other`
- `RS-LIBARCH` does not report `other`
- `RS-ARCH` does not report declared `auxiliary` roots as misplaced

This avoids:

- duplicate signaling
- drift between architecture families
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
- `Cargo.toml` is readable but malformed when auxiliary-role metadata must be resolved

Absence and unreadability are different states.
Unreadable-present inputs must not be treated as absent.

## Family Implementation Shape

Inside this guardrail family itself:

- the family root is a Cargo workspace
- `crates/runtime/src/lib.rs` orchestrates only
- `crates/runtime/src/facts.rs` binds routed placement facts with config resolution
- `crates/runtime/src/inputs.rs` produces minimal per-rule inputs
- `crates/assertions/src/*.rs` owns reusable semantic assertions
- `test_support/src/lib.rs` owns only generic `ProjectTree` builders/helpers
- exactly one `RS-ARCH-*` rule ID per production file
- exactly one rule-specific sidecar test module directory per production rule file
- live-root scope comes from shared `placement`
- root routing comes from shared `FamilyMapper`

Target family tree:

```text
apps/guardrail3/crates/app/rs/families/arch/
  Cargo.toml
  crates/
    runtime/
      Cargo.toml
      src/
        lib.rs
        facts.rs
        inputs.rs
        rs_arch_01_root_classification.rs
        rs_arch_01_root_classification_tests/
          mod.rs
        rs_arch_02_no_misplaced_roots.rs
        rs_arch_02_no_misplaced_roots_tests/
          mod.rs
        rs_arch_03_no_dual_ownership.rs
        rs_arch_03_no_dual_ownership_tests/
          mod.rs
        rs_arch_04_no_zone_overlap.rs
        rs_arch_04_no_zone_overlap_tests/
          mod.rs
        rs_arch_05_scoped_arch_config_forbidden.rs
        rs_arch_05_scoped_arch_config_forbidden_tests/
          mod.rs
        rs_arch_06_owner_family_enablement_coherence.rs
        rs_arch_06_owner_family_enablement_coherence_tests/
          mod.rs
        rs_arch_07_required_inputs_fail_closed.rs
        rs_arch_07_required_inputs_fail_closed_tests/
          mod.rs
        rs_arch_08_auxiliary_roots_declared.rs
        rs_arch_08_auxiliary_roots_declared_tests/
          mod.rs
    assertions/
      Cargo.toml
      src/
        lib.rs
        rs_arch_01_root_classification.rs
        rs_arch_02_no_misplaced_roots.rs
        rs_arch_03_no_dual_ownership.rs
        rs_arch_04_no_zone_overlap.rs
        rs_arch_05_scoped_arch_config_forbidden.rs
        rs_arch_06_owner_family_enablement_coherence.rs
        rs_arch_07_required_inputs_fail_closed.rs
        rs_arch_08_auxiliary_roots_declared.rs
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

### RS-ARCH-01

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

### RS-ARCH-02

No eligible live Rust root may live in unexpected `other` when architecture enforcement is active.

Severity:

- `Error`

Activation:

- active when global `arch` is enabled and at least one owner family is enabled:
  - `hexarch`
  - `libarch`
- inactive when both owner families are disabled

Detection:

- start from eligible live roots
- reuse `RS-ARCH-01` classification facts
- for each root classified as `other`:
  - if both owner families disabled, emit nothing
  - otherwise emit one misplaced-root finding

Do not emit for:

- declared `auxiliary` roots
- structurally excluded roots

This rule owns repo-global misplaced-root reporting exclusively.

### RS-ARCH-03

No eligible live Rust root may be owned by both architecture families.

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

### RS-ARCH-04

App-zone roots and package-zone roots must not overlap illegally.

Severity:

- `Error`

Detection:

- start from eligible live roots
- compare only app-owned roots against package-owned roots
- emit if one root directory is an ancestor of the other
- sort findings deterministically by app root path then package root path

Do not emit for:

- sibling app/package roots
- same-zone nesting

### RS-ARCH-05

Scoped `arch` config is forbidden.

Severity:

- `Error`

Detection:

- parse `guardrail3.toml`
- require `arch` to be read only from `[rust.checks]`
- emit if scoped `arch` config appears under:
  - `[rust.apps.<name>.checks]`
  - `[rust.packages.checks]`

### RS-ARCH-06

Owned roots must stay coherent with their owner family enablement.

Severity:

- `Error`

Detection:

- for governed roots:
  - app-owned roots require effective `hexarch = true`
  - package-owned roots require effective `libarch = true`

### RS-ARCH-07

Required architecture inputs must fail closed.

Severity:

- `Error`

Detection:

- if required config or placement inputs are unreadable or unparsable, emit fail-closed errors
- active required inputs include:
  - readable eligible live `Cargo.toml`
  - readable `guardrail3.toml`
  - parseable `guardrail3.toml`
  - parseable auxiliary-role metadata in eligible live `Cargo.toml`

## Test Expectations

Every rule should have:

- golden pass
- direct attack case
- exact hit set
- exact non-hit set
- fail-closed coverage where applicable
- exact severity assertions

The family must also have at least one runtime-surface test proving:

- `--family arch` survives CLI parsing
- runtime dispatch runs `RS-ARCH`
- report section name is `arch`
- generated config includes global `arch = true`
- generated config does not place `arch` under app/package-scoped checks

Declared auxiliary roots should also be surfaced in reports as informational results so the live CLI output makes the exemption explicit rather than silently hiding it.
