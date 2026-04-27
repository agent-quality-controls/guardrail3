# Wire Deps Content Checks

**Date:** 2026-04-05 20:07
**Scope:** `packages/g3rs-deps-config-checks`, `apps/guardrail3/crates/app/rs/families/deps`, `.plans/2026-04-04-142819-family-checks-packages.md`

## Summary
Wired `g3rs-deps-config-checks` into the app `deps` family for `g3rs-deps/dependencies-allowlisted`, `06`, `07`, `08`, and `12`. The package boundary was corrected to the repo’s actual current policy surface, using parsed workspace `Cargo.toml`, crate `Cargo.toml`, and legacy workspace `guardrail3.toml`, and the app now owns malformed-input signaling without duplicating `RS-DEPS-11`.

## Context & Problem
The extracted deps package had been built but not wired into the app family. The initial package draft also drifted from repo reality in two important ways:

- it assumed `guardrail3-rs.toml` even though the live deps family still reads legacy `guardrail3.toml`
- it still left the old app-side moved-rule files and assertions on disk, which made the tree lie about what actually ran

When the bridge work started, the first compile failures were stale test helpers building `DepsFacts` by hand. After that, the deeper semantic bug was duplicate `RS-DEPS-11` reporting: the app correctly emitted structural input failures, but the new package-site collection logic was re-reading and re-parsing the same broken files and creating a second error path.

## Decisions Made

### Keep the wired policy surface aligned with current repo reality
- **Chose:** wire the package against parsed legacy `guardrail3.toml` via `guardrail3_domain_config::types::GuardrailConfig`.
- **Why:** that is what the app deps family actually uses today for `allowed_deps` policy. Pretending the family already ran on `guardrail3-rs.toml` would have created a false boundary and broken real behavior.
- **Alternatives considered:**
  - Force the bridge onto `guardrail3-rs.toml` immediately — rejected because the repo does not yet have that live deps policy shape.
  - Smuggle derived allowlist/profile subsets into the package — rejected because content-check packages are supposed to take full parsed files only.

### Let the app remain the single owner of malformed-input signaling
- **Chose:** package-site collection in `deps` facts now degrades silently when typed package inputs cannot be built, instead of minting new structural failures.
- **Why:** malformed read/parse/schema signaling belongs to the app family through `RS-DEPS-11`. The package should only run on valid parsed input sites.
- **Alternatives considered:**
  - Let package-site collection push additional `InputFailureFacts` — rejected because it duplicated `RS-DEPS-11` for the same broken member manifest.
  - Move malformed handling into the package — rejected because that would break the extracted-package boundary.

### Reparse `guardrail3.toml` only at the final app-to-package bridge
- **Chose:** keep `PolicyContentCheckFacts` carrying `guardrail_content: String`, then parse that into `GuardrailConfig` in `run.rs` right before package invocation.
- **Why:** `GuardrailConfig` is not `Clone`, and widening the domain type solely to simplify this bridge would have been the wrong change. Keeping string content in the fact avoids that churn while still keeping structural failures app-owned.
- **Alternatives considered:**
  - Add `Clone` to `GuardrailConfig` — rejected because it would change a shared domain type for a local bridge convenience.
  - Store a borrowed config in facts — rejected because the facts model is owned, cloneable runtime data.

### Remove dead app-side surfaces for moved deps rules
- **Chose:** delete the old app runtime rule directories and assertion modules for `g3rs-deps/dependencies-allowlisted`, `06`, `07`, `08`, and `12`.
- **Why:** once the app family routes those rules through the package, leaving old rule files around creates false ownership and future confusion.
- **Alternatives considered:**
  - Keep the old files as dormant historical references — rejected because this repo has already cleaned that pattern out in other extracted families.

### Add family-bridge smoke tests at the app layer
- **Chose:** add `rs_deps_11_input_failures/tests/family_bridge.rs` to prove that moved deps rules are really executing through the app/package seam.
- **Why:** package-local tests are not enough; we need to prove the app bridge is wired and result conversion is correct.
- **Alternatives considered:**
  - Rely on package tests plus the binary smoke only — rejected because the family layer would still lack direct bridge coverage.

## Architectural Notes
The wired pipeline is now:

```text
deps app family
  -> discovers routed workspaces and member crates
  -> validates legacy guardrail3.toml + dependency-table shape
  -> emits RS-DEPS-11 for malformed inputs
  -> constructs parsed workspace/crate Cargo.toml + guardrail3.toml package sites
  -> calls g3rs-deps-config-checks
  -> keeps RS-DEPS-01..04 and RS-DEPS-09..11 in-app
```

The package owns:
- `g3rs-deps/dependencies-allowlisted`
- `g3rs-deps/build-dependencies-allowlisted`
- `g3rs-deps/dev-dependencies-allowlisted`
- `g3rs-deps/library-allowlist-present`
- `g3rs-deps/direct-dependency-cap`

The app keeps:
- `RS-DEPS-01`
- `RS-DEPS-02`
- `RS-DEPS-03`
- `RS-DEPS-04`
- `RS-DEPS-09`
- `RS-DEPS-10`
- `RS-DEPS-11`

The package boundary is still wider than ideal from a local code-style point of view, but it obeys the extraction rule that content packages get parsed files, not orchestrator-derived subsets.

## Information Sources
- `AGENTS.md`
- `.plans/2026-04-04-142819-family-checks-packages.md`
- `.worklogs/2026-04-05-191423-deps-content-checks-architecture-cleanup.md`
- `.worklogs/2026-04-05-193452-restore-sidecar-rule-tests.md`
- `apps/guardrail3/crates/app/rs/families/deps/crates/runtime/src/facts/guardrail.rs`
- `apps/guardrail3/crates/app/rs/families/deps/crates/runtime/src/facts/dependency_entries.rs`
- `packages/g3rs-deps-config-checks/crates/runtime/src/support.rs`
- `cargo test --workspace --manifest-path packages/g3rs-deps-config-checks/Cargo.toml`
- `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-deps --lib`
- `cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate <temp repo> --family deps --format json`

## Open Questions / Future Considerations
- `g3rs-deps-config-checks` still has the known skipped local package-rule debt around input width and local complexity.
- The package still uses current legacy `guardrail3.toml`. If deps policy later migrates to `guardrail3-rs.toml`, this bridge will need another contract change.
- There is still an exactness gap for some local path dependencies without explicit `package = "..."` naming if we ever want to match package identity more strictly than the current logic.

## Key Files for Context
- `packages/g3rs-deps-config-checks/crates/types/src/input.rs` — actual deps package input contracts after the bridge correction
- `packages/g3rs-deps-config-checks/crates/runtime/src/support.rs` — dependency identity and allowlist logic used by moved rules
- `apps/guardrail3/crates/app/rs/families/deps/crates/runtime/src/run.rs` — app/package bridge and `G3CheckResult -> CheckResult` conversion
- `apps/guardrail3/crates/app/rs/families/deps/crates/runtime/src/facts/dependency_entries.rs` — package-site collection and structural-failure ownership boundary
- `apps/guardrail3/crates/app/rs/families/deps/crates/runtime/src/facts/guardrail.rs` — legacy guardrail parsing and deps-specific shape validation
- `apps/guardrail3/crates/app/rs/families/deps/crates/runtime/src/policy/rs_deps_11_input_failures/tests/family_bridge.rs` — family-layer smoke proving the moved rules run through the bridge
- `apps/guardrail3/crates/app/rs/families/deps/README.md` — updated ownership split for the family
- `.plans/2026-04-04-142819-family-checks-packages.md` — extraction ledger updated to the wired deps boundary
- `.worklogs/2026-04-05-191423-deps-content-checks-architecture-cleanup.md` — prior deps package build context

## Next Steps / Continuation Plan
1. Run an adversarial test-attack over the wired deps family, focusing on: duplicate structural failures, package/app rule overlap, moved-rule bridge behavior, and malformed-input ownership.
2. If the attack finds no must-fix bugs, keep the deps family in the extracted set and move on to the next package in the sequence.
3. If parity concerns remain around local path dependency identity, add targeted package tests in `packages/g3rs-deps-config-checks/crates/runtime/src/.../rule_tests/`.
4. Only revisit the skipped package-local architecture debt after the extraction track is stable across all current families.
