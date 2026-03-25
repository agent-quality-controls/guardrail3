# Cut Over Cargo, Deny, And Deps Family Crates

**Date:** 2026-03-25 02:12
**Scope:** `apps/guardrail3/crates/app/rs/families/cargo/Cargo.toml`, `apps/guardrail3/crates/app/rs/families/cargo/src/*`, `apps/guardrail3/crates/app/rs/families/deny/Cargo.toml`, `apps/guardrail3/crates/app/rs/families/deny/src/*`, `apps/guardrail3/crates/app/rs/families/deps/Cargo.toml`, `apps/guardrail3/crates/app/rs/families/deps/src/*`, `apps/guardrail3/crates/app/rs/checks/rs/deps/rs_deps_11_input_failures_tests/fail_closed.rs`

## Summary
Promoted `cargo`, `deny`, and `deps` from shim crates into real family-owned crates under `families/*/src`. The three crates now compile and run their own standalone test suites, which expands the fast per-family loop beyond `fmt`, `toolchain`, and `clippy`.

## Context & Problem
The split had reached the point where several family crates were real Cargo workspace members but still not real code owners. `cargo`, `deny`, and `deps` were still thin wrappers around the old `checks/rs/*` tree, which meant the workspace structure had improved while the runtime ownership model had not.

The user asked to stop doing tiny one-family iterations and push wider sweeps. This batch follows that direction while still syncing to the plan: it clears the next substantial tranche of no-hook-shell Rust families in one pass and verifies each crate with its own library tests.

## Decisions Made

### Promote three low-friction families together
- **Chose:** cut over `cargo`, `deny`, and `deps` in one batch instead of one commit per family.
- **Why:** these families do not depend on the shared hook-shell parser and can be migrated with the same crate-ownership pattern already proven by `toolchain`, `fmt`, and `clippy`.
- **Alternatives considered:**
  - Continue one family at a time — rejected because it slows the split and produces unnecessary review/commit overhead.
  - Jump straight to `code` or `test` — rejected because those families have larger shared-substrate dependencies and are worse candidates for a broad sweep.

### Make the crates real owners instead of preserving wrapper shims
- **Chose:** copy the old `checks/rs/{cargo,deny,deps}` sources into each family crate `src/` tree, replace the shim `lib.rs` files with real family entrypoints, and delete the copied wrapper `mod.rs` files.
- **Why:** the point of the split is to make Cargo crate ownership line up with source ownership. Re-exporting or path-including the old tree would keep the same architectural lie in a different package layout.
- **Alternatives considered:**
  - Keep `#[path = ...]` wrappers for now — rejected because that only changes manifests, not ownership.
  - Delete the old `checks/rs/*` trees immediately — rejected because the repo is heavily dirty and compatibility callers/tests still exist.

### Normalize imports onto real crate owners while pruning manifests
- **Chose:** rewrite copied sources to import `guardrail3_domain_*`, `guardrail3_app_core`, `guardrail3_adapters_outbound_fs`, and `guardrail3_outbound_traits` directly, then remove unused dependencies from the new family manifests.
- **Why:** the wider split only helps if the family crates stop depending on fake nested module trees and stop carrying monolith-era dependency baggage.
- **Alternatives considered:**
  - Keep broad dependency lists from the old wrappers — rejected because `unused-crate-dependencies` would keep surfacing and the manifests would still hide real ownership.
  - Introduce new compatibility shims to preserve old import paths — rejected because that would repeat the same problem the split is trying to remove.

### Describe `deps` fail-closed behavior more accurately in tests
- **Chose:** update the `RS-DEPS-11` fail-closed tests to assert failure categories and paths rather than assuming one exact parser message per broken manifest.
- **Why:** once the crate owns its tests directly, those tests need to reflect what the collector actually emits: a malformed manifest can surface both workspace-parse and root-discovery failures, and parser wording/columns are not the contract.
- **Alternatives considered:**
  - Force the collector to deduplicate by file path — rejected because the current rule is explicitly about surfacing input failures, and distinct failure stages are meaningful.
  - Keep exact parser-string assertions — rejected because they are brittle and already mismatched current TOML parser output.

## Architectural Notes
`cargo`, `deny`, and `deps` now follow the same real-owner pattern as the earlier migrated crates:
- family-local `src/lib.rs` owns orchestration
- family-local modules own `facts`, `inputs`, rules, and sidecar rule tests
- manifests declare only the real shared crates they depend on

The `deps` family test adjustment matters beyond this batch because it makes the compatibility path and the new family crate agree on the fail-closed contract. That reduces one source of false red during the transition where both the new family crate and old tree still coexist.

This batch further reduces the monolith test bottleneck by adding three more family crates that can be tested directly:
- `guardrail3-app-rs-family-cargo`
- `guardrail3-app-rs-family-deny`
- `guardrail3-app-rs-family-deps`

## Information Sources
- `.plans/todo/checks/2026-03-24-guardrail3-workspace-crate-split.md` — end-state crate split plan and rationale.
- `.plans/todo/check_review/test_hardening/30-workspace-split-phase1-agent-brief.md` — requirement that crate boundaries become real and imports move to crate paths.
- `apps/guardrail3/crates/app/rs/checks/rs/cargo/*` — original `cargo` family sources promoted into the crate.
- `apps/guardrail3/crates/app/rs/checks/rs/deny/*` — original `deny` family sources promoted into the crate.
- `apps/guardrail3/crates/app/rs/checks/rs/deps/*` — original `deps` family sources promoted into the crate.
- `.worklogs/2026-03-25-011916-runtime-shim-and-toolchain-cutover.md` — prior runtime/toolchain ownership cut.
- `.worklogs/2026-03-25-015759-fmt-and-clippy-family-cutover.md` — prior family sweep that established the current promotion pattern.

## Open Questions / Future Considerations
- `code`, `hexarch`, `garde`, `release`, and `arch` still need the same cutover from shim crates to real owners.
- `test` and the hook families still have additional shared-substrate debt around hook-shell parsing and should be left for a later dedicated sweep.
- The root crate and root test harness are still much wider than the intended thin-facade end state, so fast family tests are improving while the monolithic root loop remains heavy.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/cargo/src/lib.rs` — real `cargo` family entrypoint after cutover.
- `apps/guardrail3/crates/app/rs/families/deny/src/lib.rs` — real `deny` family entrypoint and baseline export.
- `apps/guardrail3/crates/app/rs/families/deps/src/lib.rs` — real `deps` family entrypoint and fact fanout.
- `apps/guardrail3/crates/app/rs/families/deps/src/rs_deps_11_input_failures_tests/fail_closed.rs` — updated fail-closed expectations for the family-owned crate.
- `apps/guardrail3/crates/app/rs/checks/rs/deps/rs_deps_11_input_failures_tests/fail_closed.rs` — matching compatibility-path test update.
- `.worklogs/2026-03-25-015759-fmt-and-clippy-family-cutover.md` — previous family cutover batch to read before continuing the same split pattern.
- `.worklogs/2026-03-25-011916-runtime-shim-and-toolchain-cutover.md` — earlier runtime-path cleanup that made family crates the live owners.

## Next Steps / Continuation Plan
1. Commit this `cargo + deny + deps` batch and keep the next sweep broad instead of falling back to one-family turns.
2. Re-scan the remaining Rust family crates and choose the next no-hook-shell cohort, likely `arch + garde + release + hexarch` or a nearby subset depending on actual shim/blocker shape.
3. Promote that next cohort into real crate-owned `src/` trees with direct crate imports, then rerun standalone family tests plus `cargo check --manifest-path apps/guardrail3/Cargo.toml --workspace --lib`.
4. After the no-hook-shell families are exhausted, extract the remaining shared substrate needed by `code`, `test`, and hook-related families so those can be cut over in their own broader sweep.
