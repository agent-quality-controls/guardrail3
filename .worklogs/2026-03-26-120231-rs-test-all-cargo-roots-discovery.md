# RS-TEST All-Cargo-Roots Discovery

**Date:** 2026-03-26 12:02
**Scope:** `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/discover.rs`, `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/lib.rs`, `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_02_owned_sidecar_shape_tests/golden.rs`, `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_03_runtime_assertions_split_tests/golden.rs`, `apps/guardrail3/crates/app/rs/families/test/README.md`, `.plans/todo/checks/rs/test.md`

## Summary
Changed `RS-TEST` discovery to consider every `Cargo.toml` directory as a discovered Rust root, while assigning Rust files to the deepest enclosing component root when a package owns the `crates/runtime` + `crates/assertions` split. Added nested package-root regressions and updated the family contract text to match the new root model.

## Context & Problem
The prior `RS-TEST` root model only evaluated workspace roots and standalone packages outside workspaces. That made nested package roots inside a workspace invisible to the test family, which is wrong for nested hex structures where package directories under a workspace still carry their own `Cargo.toml` and should be independently eligible for `RS-TEST`.

Simply switching to “every `Cargo.toml` is a root” is not enough on its own, because componentized packages also contain inner `crates/runtime` and `crates/assertions` workspace members. If file ownership follows the deepest raw Cargo root, the inner `runtime` crate steals its files from the outer package root and the family misclassifies valid component layouts.

## Decisions Made

### Discover all `Cargo.toml` directories as roots
- **Chose:** Build `TestRootFacts` for every discovered `Cargo.toml` directory.
- **Why:** This matches the user requirement and makes nested package roots inside workspaces visible to `RS-TEST`.
- **Alternatives considered:**
  - Keep the previous “workspace roots + standalone packages only” model — rejected because it hides nested package roots.
  - Infer roots only from structural `crates/runtime` matches — rejected because roots also own mutation/nextest/config surfaces and should still originate from explicit manifests.

### Prefer component-root ownership over inner member-crate ownership
- **Chose:** When a file is under a discovered component root, assign it to the deepest enclosing root whose component owns that file, rather than to the deepest raw Cargo root.
- **Why:** This preserves the intended package-level ownership for `component-root/crates/{runtime,assertions}` layouts and prevents the inner `runtime` or `assertions` subcrate manifests from hijacking file ownership.
- **Alternatives considered:**
  - Let nearest Cargo root win for all files — rejected because valid component layouts then fail `RS-TEST-02/03`.
  - Exclude inner `runtime`/`assertions` manifests from root discovery entirely — rejected because the user explicitly asked for every `Cargo.toml` root to participate in discovery.

### Remove duplicate Cargo input-failure emission for discovered roots
- **Chose:** Keep the global Cargo failure fallback only for failures whose root was not actually discovered into `facts.roots`.
- **Why:** Under the new all-roots model, malformed manifests already have a root-scoped input-failure path. Emitting both paths duplicates `RS-TEST-10`.
- **Alternatives considered:**
  - Leave duplicate failures in place — rejected because it produces noisy, structurally incorrect output.
  - Delete the global fallback entirely — rejected because it may still be needed for exceptional cases where a manifest read failure prevents a root from being materialized.

## Architectural Notes
The root model is now:

- root discovery starts from every `Cargo.toml`
- component ownership is structural and package-level
- file classification prefers the deepest component-owning root over the deepest raw manifest directory

This preserves the orchestrator/facts split from the previous refactor while fixing the nested package ownership bug without reintroducing special-case family exemptions.

## Information Sources
- `AGENTS.md` — worklog and commit requirements
- `apps/guardrail3/crates/app/rs/families/test/README.md` — current `RS-TEST` contract
- `.plans/todo/checks/rs/test.md` — mirrored rule inventory/plan
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/discover.rs` — discovery/orchestration code
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/lib.rs` — root-scoped execution and input-failure emission
- `.worklogs/2026-03-26-115739-rs-test-discovery-orchestrator-refactor.md` — prior architectural cleanup that moved discovery into `discover.rs`

## Open Questions / Future Considerations
- The current implementation still prefers direct `crates/runtime` discovery inside each owned root. If the contract ever changes to allow multi-component sets inside one root, `collect_components(...)` will need a broader structural scan.
- The README language now says every `Cargo.toml` directory is an owned root, but the family still conceptually treats componentized package roots as the meaningful owners of runtime/assertions files. That distinction is now encoded in classification rather than in the root list itself.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/discover.rs` — all-root discovery and component-preferred file ownership
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/lib.rs` — top-level execution and root-scoped input-failure handling
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_02_owned_sidecar_shape_tests/golden.rs` — nested package-root sidecar regression
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/rs_test_03_runtime_assertions_split_tests/golden.rs` — nested package-root runtime/assertions regression
- `apps/guardrail3/crates/app/rs/families/test/README.md` — updated owned-root contract
- `.worklogs/2026-03-26-115739-rs-test-discovery-orchestrator-refactor.md` — preceding refactor that made this change cleaner to implement

## Next Steps / Continuation Plan
1. Attack the new ownership model against deeper nested package paths and malformed mixed-root scenarios, especially where both an outer workspace root and a nested component root exist.
2. Re-check whether any other `RS-TEST` rules still implicitly assume the old workspace-root-only model and add targeted regressions where necessary.
3. If the user wants broader nested-component support beyond one direct `crates/runtime` pair per root, widen `collect_components(...)` structurally and add adversarial tests before changing the contract again.
