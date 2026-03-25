# RS-ARCH Implementation Handoff

**Date:** 2026-03-24

## Summary

Implemented a new Rust validation family, `rs/arch`, for repo-global Rust root placement and architecture ownership.

The family now exists under the new checker architecture with:

- one production file per `RS-ARCH-*` rule
- one rule-specific `*_tests/` directory per rule
- shared root discovery/classification substrate for future reuse
- runtime/config/report wiring so `arch` is a selectable Rust validation family

This pass did **not** reach a completed end-to-end runtime proof on the current branch because binary compilation remained very slow and full test compilation is already blocked by unrelated existing breakage in other families.

## Files Added

- `apps/guardrail3/crates/app/rs/checks/rs/rust_root_placement.rs`
- `apps/guardrail3/crates/app/rs/checks/rs/arch/mod.rs`
- `apps/guardrail3/crates/app/rs/checks/rs/arch/facts.rs`
- `apps/guardrail3/crates/app/rs/checks/rs/arch/inputs.rs`
- `apps/guardrail3/crates/app/rs/checks/rs/arch/test_support.rs`
- `apps/guardrail3/crates/app/rs/checks/rs/arch/rs_arch_01_root_classification.rs`
- `apps/guardrail3/crates/app/rs/checks/rs/arch/rs_arch_02_no_misplaced_roots.rs`
- `apps/guardrail3/crates/app/rs/checks/rs/arch/rs_arch_03_no_dual_ownership.rs`
- `apps/guardrail3/crates/app/rs/checks/rs/arch/rs_arch_04_no_zone_overlap.rs`
- `apps/guardrail3/crates/app/rs/checks/rs/arch/rs_arch_05_enablement_coherence.rs`
- all five rule-specific `*_tests/` directories under `apps/guardrail3/crates/app/rs/checks/rs/arch/`
- `.plans/2026-03-24-rs-arch-handoff.md`

## Files Modified

- `apps/guardrail3/crates/app/rs/checks/rs/mod.rs`
- `apps/guardrail3/crates/app/rs/runtime.rs`
- `apps/guardrail3/crates/domain/config/types.rs`
- `apps/guardrail3/crates/domain/report/mod.rs`
- `apps/guardrail3/crates/adapters/inbound/cli/help_gen.rs`
- `apps/guardrail3/crates/adapters/inbound/cli/init.rs`
- `apps/guardrail3/crates/domain/modules/guide.rs`
- `.plans/todo/checks/rs/arch.md`

## What Was Implemented Exactly

### 1. Shared root-placement substrate

Added `rs/rust_root_placement.rs` to centralize:

- discovery of all Rust `Cargo.toml` roots from `ProjectTree`
- classification into:
  - `app`
  - `package`
  - `other`
  - `ambiguous`
- owner-family candidate detection:
  - `hexarch`
  - `libarch`
- illegal app/package zone overlap pair collection
- fail-closed discovery/input failures for missing cached `Cargo.toml` content

Important implementation detail:

- classification is segment-based, not only top-level-root-based
- this means nested shapes like `apps/<app>/packages/<pkg>` and `packages/<pkg>/apps/<app>` are treated as real ambiguous/overlap cases

### 2. `rs/arch` family orchestration

Added `arch/mod.rs`, `facts.rs`, and `inputs.rs`.

The orchestrator currently:

- collects shared placement facts
- parses `guardrail3.toml`
- resolves effective `hexarch` / `libarch` enablement
- builds typed inputs per rule

### 3. Implemented rules

#### `RS-ARCH-01`

`rs_arch_01_root_classification.rs`

Flags roots whose classification is ambiguous.

#### `RS-ARCH-02`

`rs_arch_02_no_misplaced_roots.rs`

Flags `other` roots when architecture enforcement is active.

Current reporting rule:

- if `hexarch` or `libarch` is effectively enabled anywhere relevant, `other` roots error
- if both are disabled, `RS-ARCH-02` stays quiet

#### `RS-ARCH-03`

`rs_arch_03_no_dual_ownership.rs`

Flags roots with both app and package ownership candidates.

#### `RS-ARCH-04`

`rs_arch_04_no_zone_overlap.rs`

Flags illegal overlap/nesting between app-owned and package-owned roots.

#### `RS-ARCH-05`

`rs_arch_05_enablement_coherence.rs`

Currently owns:

- governed root with owning family effectively disabled
- malformed `guardrail3.toml`
- missing cached content for discovered `Cargo.toml` inputs

This is the family’s current fail-closed rule.

## Test Structure Implemented

Each rule has its own directory:

- `rs_arch_01_root_classification_tests/`
- `rs_arch_02_no_misplaced_roots_tests/`
- `rs_arch_03_no_dual_ownership_tests/`
- `rs_arch_04_no_zone_overlap_tests/`
- `rs_arch_05_enablement_coherence_tests/`

Patterns used:

- `golden.rs`
- attack-vector files
- `false_positives.rs`
- `fail_closed.rs` where relevant

The family test strategy is mostly synthetic `ProjectTree` construction via `arch/test_support.rs`, not temp-fixture copying.

Reason:

- `rs/arch` is repo-structure and config-resolution heavy
- in-memory trees make ownership/overlap cases easier to express precisely

## Runtime Wiring Completed

`arch` is wired at the source level into Rust validate:

- `RustValidateFamily::Arch` added in `domain/report/mod.rs`
- `RustChecksConfig.arch` added in `domain/config/types.rs`
- `runtime.rs` dispatches `RustValidateFamily::Arch` to `rs::checks::rs::arch::check(&tree)`
- `checks/rs/mod.rs` exports `pub mod arch`
- help/guide/init text updated to mention `arch`

So the intended runtime path is:

```text
guardrail3 rs validate . --family arch
```

## Issues Encountered

### 1. Full test compilation is already broken outside this change

`cargo test -p guardrail3 rs_arch --quiet` does not isolate the new family cleanly because unrelated existing tests in other families compile first and fail.

The observed failures are not from `rs/arch`.

They are existing callsite mismatches against changed `check(...)` signatures in:

- `apps/guardrail3/crates/app/rs/checks/rs/code/**`
- `apps/guardrail3/crates/app/rs/checks/rs/garde/**`
- `apps/guardrail3/crates/app/rs/checks/rs/test/**`

Representative failures:

- code tests calling `code::check(&tree)` instead of `code::check(&tree, scoped_files)`
- garde tests calling `garde::check(&tree)` instead of `garde::check(&tree, scoped_files)`
- test-family tests calling `test::check(&tree, tc)` instead of `test::check(&tree, tc, scoped_files)`

Because of that, I used:

- `cargo check -p guardrail3 --lib`

as the reliable compile verification step for this pass.

### 2. End-to-end runtime proof was blocked by very slow binary recompilation

I attempted:

```text
cargo run -p guardrail3 -- rs validate . --family arch
```

Observed behavior:

- multiple earlier `cargo run` attempts left stale artifact-lock contention until they were killed
- after cleanup, the current run still spent several minutes recompiling `apps/guardrail3/crates/lib.rs`
- `rustc` was actively compiling at high CPU, but validation output never arrived within the practical wait window

Conclusion:

- runtime dispatch appears wired
- but this pass did **not** complete a real CLI output capture from the current repo

### 3. Dirty worktree / overlapping branch changes

The repo already had many unrelated modified and untracked files before and during this pass.

I avoided reverting any of that.

This means:

- `git status` contains substantial unrelated noise
- review should focus only on the specific `rs/arch` files and the small runtime/config/help wiring touched here

## Verification Achieved

Completed:

- `cargo fmt --all`
- `cargo check -p guardrail3 --lib`

Not completed:

- clean `cargo test -p guardrail3 ...`
- successful captured output from `cargo run -p guardrail3 -- rs validate . --family arch`

## Current Gaps / Follow-ups

### 1. `libarch` is still not implemented

`rs/arch` now reasons about `libarch` ownership and enablement, but the actual `rs/libarch` family does not yet exist in runtime.

### 2. Shared placement substrate is not yet adopted by `hexarch`

`rust_root_placement.rs` exists for reuse, but current `hexarch` still has its own discovery/facts world.

Next logical cleanup:

- make `hexarch` consume the shared root inventory where appropriate
- use the same substrate when `libarch` lands

### 3. `RS-ARCH-05` may need expansion once `libarch` is real

Right now `RS-ARCH-05` covers:

- effective owner disabled
- malformed config
- missing cached Cargo content

Once `libarch` exists, revisit whether any additional cross-family coherence states belong in this rule.

## Recommended Next Steps

1. Fix unrelated existing test-callsite breakage in `code`, `garde`, and `test` so targeted family tests can compile again.
2. Re-run:
   - `cargo test -p guardrail3 rs_arch --quiet`
3. Re-run:
   - `cargo run -p guardrail3 -- rs validate . --family arch`
4. Capture the actual findings from the current repo and compare them against expected root-placement policy.
5. Decide whether `rs/arch` should also be included by default in any generated/init config templates beyond the current `arch = true` insertion.
6. After that, start `rs/libarch` using `rust_root_placement.rs` as the base discovery layer.

## Cold Start Read List For This Work

- `apps/guardrail3/crates/app/rs/checks/rs/rust_root_placement.rs`
- `apps/guardrail3/crates/app/rs/checks/rs/arch/mod.rs`
- `apps/guardrail3/crates/app/rs/checks/rs/arch/facts.rs`
- `apps/guardrail3/crates/app/rs/checks/rs/arch/inputs.rs`
- all `apps/guardrail3/crates/app/rs/checks/rs/arch/rs_arch_*`
- `apps/guardrail3/crates/app/rs/runtime.rs`
- `apps/guardrail3/crates/domain/config/types.rs`
- `apps/guardrail3/crates/domain/report/mod.rs`
- `.plans/todo/checks/rs/arch.md`
