# Stabilize RS-CLIPPY Workspace Baseline

**Date:** 2026-03-27 20:20
**Scope:** `apps/guardrail3/Cargo.toml`, `apps/guardrail3/Cargo.lock`, `apps/guardrail3/crates/adapters/inbound/cli/Cargo.toml`, `apps/guardrail3/crates/app/rs/Cargo.toml`, `apps/guardrail3/crates/app/rs/validate/Cargo.toml`, `apps/guardrail3/crates/app/rs/families/clippy/**`

## Summary
Converted the live `clippy` migration from a broken nested-workspace half-state into a coherent baseline that builds again. The family now runs as a real `crates/runtime` package, validates cleanly under `RS-ARCH` and `RS-CLIPPY`, and exposes the actual remaining work as `RS-TEST` migration debt rather than manifest/path failures.

## Context & Problem
After the last `RS-CODE` hardening rounds, focus shifted to `RS-CLIPPY`. The family had already been partially moved on disk into `crates/runtime`, `crates/assertions`, and `test_support`, but the migration was not coherent:

- the top-level app workspace still depended on `families/clippy` instead of `families/clippy/crates/runtime`
- `apps/guardrail3/Cargo.toml` still listed the family root as a workspace member, which made Cargo see two workspace roots
- the family root workspace existed but lacked `workspace.package` / `workspace.lints`, so member crates using `lints.workspace = true` could not parse
- the runtime test harness was broken because `lib.rs` still declared `mod test_support;` while the actual `test_support` implementation had been moved out of `crates/runtime`

That meant `cargo test`, `rs validate --family clippy`, and `rs validate --family test` were all failing for workspace reasons instead of telling us anything meaningful about the family.

## Decisions Made

### Make `clippy` a real nested workspace before touching deeper `RS-TEST` debt
- **Chose:** Finish the manifest/path wiring first: point every dependency at `crates/runtime`, make the app workspace list `crates/runtime`, `crates/assertions`, and `test_support`, and add nested `workspace.package` / `workspace.lints` to the family root.
- **Why:** Until Cargo saw one consistent workspace graph, none of the family-level validation output was trustworthy.
- **Alternatives considered:**
  - Jump straight into assertions extraction — rejected because the family was not even loading as a package graph.
  - Revert back to the old single-crate layout — rejected because the migration was already largely on disk and the nested workspace is the correct target shape.

### Restore a minimal runtime-local test harness shim to expose the real next debt
- **Chose:** Recreate `crates/runtime/src/test_support.rs` as a shim that re-exports generic helpers from the sibling `test_support` crate and keeps the runtime-only route/facts helpers needed by the existing sidecars.
- **Why:** The family’s test corpus was still written against a local `test_support` module. Reintroducing the shim is not the end state, but it is the smallest change that restores a working baseline and surfaces the remaining `RS-TEST` violations accurately.
- **Alternatives considered:**
  - Move all sidecars immediately to sibling assertions/test-support APIs — rejected for this checkpoint because it would mix workspace repair with large semantic test refactoring.
  - Put runtime-aware helpers directly into the sibling `test_support` crate — rejected because that would immediately blur the boundary `RS-TEST-18` is meant to enforce.

### Treat the green `RS-CLIPPY` result as a baseline checkpoint, not completion
- **Chose:** Add the family-local `clippy.toml`, keep the placeholder `assertions` crate buildable, and update the README to record the new state: green under `RS-CLIPPY`/`RS-ARCH`, still very red under `RS-TEST`.
- **Why:** The family needed a documented, reproducible starting point before the larger assertions extraction starts.
- **Alternatives considered:**
  - Wait to document until `RS-TEST` is also green — rejected because the live contract would stay stale during the most error-prone part of the migration.

## Architectural Notes
This checkpoint intentionally does **not** solve the `RS-TEST` migration for `clippy`. It clarifies the boundary instead:

- `crates/runtime` is now the real package consumed by the app and validator
- `crates/assertions` exists as a sibling crate, but is still mostly a placeholder
- `test_support` is the sibling generic-helper crate, but the runtime shim still exists because the sidecars have not been rewritten yet

So the family has crossed from “broken migration state” to “stable baseline with explicit remaining debt.” The remaining debt is now narrow and visible:

- `RS-TEST-03`: `612`
- `RS-TEST-16`: `16`
- `RS-TEST-02`: `1`
- `RS-TEST-01`: `1`
- `RS-TEST-18`: `1`

## Information Sources
- `apps/guardrail3/crates/app/rs/families/clippy/README.md`
- `apps/guardrail3/crates/app/rs/families/code/README.md` for the stabilized family pattern
- `apps/guardrail3/crates/app/rs/families/fmt/Cargo.toml` for nested family workspace boilerplate
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/lib.rs`
- `apps/guardrail3/crates/app/rs/families/clippy/test_support/src/lib.rs`
- `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-clippy --lib`
- `cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/clippy --family clippy --inventory --format json`
- `cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3/crates/app/rs/families/clippy --family test --inventory --format json`

## Open Questions / Future Considerations
- The runtime-local `test_support` shim is still an architectural compromise. It should disappear once the sidecars are rewritten to use owned assertions modules and generic sibling `test_support` helpers only.
- The sibling `test_support` crate still contains clippy-semantic helpers like `canonical_clippy_toml()` and tree builders. That is probably where `RS-TEST-18` will bite once the broader structural debt is reduced.
- `crates/assertions` is still only a placeholder. The next real migration slice should move semantic result assertions there rule by rule rather than trying to “bless” the current runtime-side assertions.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/clippy/README.md` — current family contract and migration status
- `apps/guardrail3/crates/app/rs/families/clippy/Cargo.toml` — nested family workspace root
- `apps/guardrail3/crates/app/rs/families/clippy/clippy.toml` — family-local managed baseline config that removed the `RS-CLIPPY-01` self-hit
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/Cargo.toml` — real runtime package manifest
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/lib.rs` — family orchestrator and the runtime-side test entrypoint
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/test_support.rs` — temporary runtime shim that keeps the migrated tests compiling
- `apps/guardrail3/crates/app/rs/families/clippy/test_support/src/lib.rs` — sibling generic-helper crate, likely future `RS-TEST-18` pressure point
- `apps/guardrail3/crates/app/rs/families/clippy/crates/assertions/src/lib.rs` — current empty assertions placeholder
- `.worklogs/2026-03-27-195847-baseline-clippy-and-unblock-fmt-workspace.md` — prior README/baseline checkpoint
- `.worklogs/2026-03-27-200723-stabilize-rs-fmt-family.md` — working specimen for the nested family workspace shape

## Next Steps / Continuation Plan
1. Remove the bulk `RS-TEST-03` boundary errors by rewriting the `clippy` sidecars to stop importing the runtime-local `test_support` shim. Start with the simplest cluster: `rs_clippy_02`, `03`, `09`, `10`, `11`, `21`, and `22`, since they mostly share `collected_facts` / `config_input` patterns.
2. Populate `crates/assertions/src` with rule-local assertion modules and move semantic result checks there. Start with `rs_clippy_01`, `12`, `13`, and one threshold rule so the pattern is proven before the full sweep.
3. Once the sidecars no longer need runtime-local test support, delete `crates/runtime/src/test_support.rs`, rerun `RS-TEST`, and then attack-review the live `RS-CLIPPY` rules themselves for false positives/false negatives.
