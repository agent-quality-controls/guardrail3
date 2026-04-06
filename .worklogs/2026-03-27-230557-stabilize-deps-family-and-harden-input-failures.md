# Stabilize Deps Family And Harden Input Failures

**Date:** 2026-03-27 23:05
**Scope:** `apps/guardrail3/Cargo.toml`, `apps/guardrail3/crates/app/rs/Cargo.toml`, `apps/guardrail3/crates/app/rs/families/deps/Cargo.toml`, `apps/guardrail3/crates/app/rs/families/deps/README.md`, `apps/guardrail3/crates/app/rs/families/deps/crates/runtime/**`, `apps/guardrail3/crates/app/rs/families/deps/crates/assertions/**`, `apps/guardrail3/crates/app/rs/families/deps/test_support/**`

## Summary
Converted `families/deps` from a single crate into the standard self-hosted family workspace shape, rewired the app manifests to point at `crates/runtime`, and moved reusable testing helpers into dedicated `assertions` and `test_support` crates. Then ran repeated adversarial passes on `RS-DEPS` and fixed multiple false negatives and fail-open paths, especially around `workspace = true`, `.gitignore` precedence, unreadable inputs, and semantically malformed Cargo/guardrail policy shapes.

## Context & Problem
The `deps` family started from the older single-crate layout while the active Rust family pattern had already standardized on nested workspaces with `runtime`, `assertions`, and `test_support`. That made the family inconsistent with the current self-hosting model and harder to validate in isolation.

After the structural work, the bigger problem was trust. `RS-DEPS` mixes several scopes:
- tool installation checks
- crate-local dependency allowlists
- Rust-root lockfile ownership
- fail-closed handling for policy inputs

That mixed scope left a lot of room for detector bugs. The family needed aggressive attack coverage to prove it did not silently weaken when manifests or policy files were malformed in subtle but TOML-valid ways.

## Decisions Made

### Split `families/deps` into the standard nested workspace shape
- **Chose:** Convert `apps/guardrail3/crates/app/rs/families/deps` into a local Cargo workspace with `crates/runtime`, `crates/assertions`, and `test_support`.
- **Why:** This matches the active family pattern used elsewhere, keeps reusable result assertions out of runtime tests, and lets the family run directly from its own workspace root while the broader app workspace is still noisy.
- **Alternatives considered:**
  - Keep the family as a single crate and only patch the tests in place — rejected because it would preserve the structural drift that the handoff explicitly asked to remove.
  - Move only runtime code and leave test helpers inside the runtime crate — rejected because that would keep semantic proof helpers and generic fixture helpers mixed together.

### Rewire only the `deps` entries in shared manifests
- **Chose:** Update the shared app manifests so `deps` resolves through `families/deps/crates/runtime`, while keeping the commit scoped to the `deps` hunks in those shared files.
- **Why:** The working tree already contained unrelated `garde` edits in the same manifest files. Staging only the `deps` portions kept this commit accurate without sweeping in another family’s unfinished work.
- **Alternatives considered:**
  - Commit the full manifest files including unrelated `garde` rewires — rejected because that would mix ownership across families in one commit.
  - Omit shared-manifest rewiring entirely — rejected because the family split is incomplete if the app still points at the old root path.

### Treat malformed active inputs as `RS-DEPS-11`, not soft policy drift
- **Chose:** Harden `facts.rs` so malformed but present inputs become explicit `RS-DEPS-11` failures instead of collapsing into missing-policy or skipped-discovery behavior.
- **Why:** The family contract explicitly says required dependency-policy inputs must fail closed. If a malformed manifest can still be parsed as generic TOML but important fields have the wrong types, the collector should not guess.
- **Alternatives considered:**
  - Accept broad TOML shape and let downstream rules infer best-effort defaults — rejected because that causes silent false negatives and inconsistent severity.
  - Only fail on outright TOML parse errors — rejected because several of the attack cases were structurally invalid for this family while still syntactically valid TOML.

### Validate workspace and dependency table shapes before extracting facts
- **Chose:** Add explicit validation for:
  - `guardrail3.toml` deps policy shape
  - `[workspace].members`
  - `[workspace.dependencies].*`
  - dependency table entry shapes in `[dependencies]`, `[build-dependencies]`, and `[dev-dependencies]`
- **Why:** The collector previously filtered invalid values with `as_*` accessors, which silently converted malformed fields into “not present” and weakened the rule surface.
- **Alternatives considered:**
  - Keep the current best-effort extraction and add only more tests — rejected because the tests would still be proving incorrect behavior.
  - Push validation down into individual rules — rejected because malformed input shape is a collector responsibility shared by several rules.

## Architectural Notes
The family now follows the same structure as the stabilized specimens:

- `crates/runtime` owns collection, inputs, rule fan-out, and rule-local sidecar tests
- `crates/assertions` owns reusable result assertions
- `test_support` owns generic fixture and test-only helpers

The most important semantic hardening stayed in `crates/runtime/src/facts.rs`. The collector now guards the boundary between “usable facts” and “invalid active input” more aggressively.

The concrete detector fixes covered in this batch include:
- `workspace = true` external path dependencies are no longer skipped just because the workspace dependency used `path`
- anchored `.gitignore` patterns like `/Cargo.lock` and `/Cargo.*` stay anchored instead of collapsing to nested roots
- unreadable relevant `.gitignore`, `guardrail3.toml`, and routed `Cargo.toml` surfaces now report `RS-DEPS-11`
- unknown deps-policy keys and empty `allowed_deps` entries in `guardrail3.toml` now fail closed
- non-string `[workspace].members` entries now fail closed
- invalid `[workspace.dependencies].*.package` shapes now fail closed
- invalid dependency `workspace` flag types now fail closed

## Information Sources
- `.plans/todo/checks/rs/deps.md`
- `.plans/todo/family-stabilization-handoffs/deps.md`
- `apps/guardrail3/crates/app/rs/README.md`
- `apps/guardrail3/crates/app/rs/families/test/README.md`
- live `deps` family files under `apps/guardrail3/crates/app/rs/families/deps/**`
- recent worklogs:
  - `.worklogs/2026-03-27-230141-toolchain-attack-followup.md`
  - `.worklogs/2026-03-27-222125-clarify-clippy-cargo-override-ownership.md`
  - `.worklogs/2026-03-27-221725-close-clippy-cargo-config-override-gap.md`

## Open Questions / Future Considerations
- The outer app workspace still has unrelated family drift elsewhere, so the reliable checkpoint remains the nested family workspace test command rather than repo-wide Cargo entrypoints.
- `target.*.{dependencies,build-dependencies,dev-dependencies}` are still an explicit remaining gap in `RS-DEPS-CONFIG-01..07`; this commit does not change that contract.
- The shared manifest files still have unrelated unstaged `garde` work in the working tree. This commit stages only the `deps` wiring in those files.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/deps/Cargo.toml` — nested workspace root for the family
- `apps/guardrail3/crates/app/rs/families/deps/README.md` — current family shape and local verification entrypoint
- `apps/guardrail3/crates/app/rs/families/deps/crates/runtime/src/facts.rs` — collector and all fail-closed/input-shape hardening
- `apps/guardrail3/crates/app/rs/families/deps/crates/runtime/src/test_harness.rs` — family test route and shared runtime test helpers
- `apps/guardrail3/crates/app/rs/families/deps/crates/runtime/src/rs_deps_config_01_dependencies_allowlisted_tests/workspace_path.rs` — `workspace = true` external path regression
- `apps/guardrail3/crates/app/rs/families/deps/crates/runtime/src/rs_deps_10_gitignore_not_ignoring_cargo_lock_tests/precedence.rs` — anchored `.gitignore` precedence regressions
- `apps/guardrail3/crates/app/rs/families/deps/crates/runtime/src/rs_deps_11_input_failures_tests/fail_closed.rs` — fail-closed attack coverage for malformed and unreadable inputs
- `apps/guardrail3/Cargo.toml` — top-level workspace member rewiring for `deps`
- `apps/guardrail3/crates/app/rs/Cargo.toml` — app-level family dependency rewiring for `deps`
- `.plans/todo/checks/rs/deps.md` — current rule contract and remaining gaps

## Next Steps / Continuation Plan
1. Keep future `deps` attack work focused on remaining semantics gaps, especially policy inheritance edge cases and any mixed-root lockfile ownership oddities that still survive current tests.
2. When the broader app workspace is healthy again, rerun the full validator entrypoints for `RS-ARCH`, `RS-TEST`, and `RS-DEPS` on the `deps` family root to prove the local nested-workspace green state survives top-level orchestration.
3. Leave repo-wide dependency debt alone unless a future finding proves another detector bug. The next work here should be more attacks on the checker, not cleanup of unrelated crates.
