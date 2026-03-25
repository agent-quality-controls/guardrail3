# Drop Root Release Test Cluster

**Date:** 2026-03-25 11:11
**Scope:** `apps/guardrail3/tests/unit.rs`, `apps/guardrail3/tests/unit/test_release*.rs`

## Summary
Removed the old root-level Rust release test cluster from `apps/guardrail3/tests/unit.rs` and deleted the corresponding legacy root test files. The release surface is now validated at its actual owner boundary, `guardrail3-app-rs-family-release`, whose crate-local test target passed with 256 tests.

## Context & Problem
The workspace split plan is explicit that crate promotion does not count as a performance win if the root test harness keeps compiling the same surface through `tests/unit.rs`. The old release cluster was a particularly bad example:
- more than 1,100 lines of root tests
- all tied to legacy `app::rs::validate::release_*` helper APIs
- all covering a surface that already has a real owner crate with rule-local tests

Keeping that cluster in the root package meant the release family was effectively being tested twice, once correctly at the owner crate and once incorrectly through the legacy root harness.

## Decisions Made

### Remove the redundant root release cluster instead of migrating it file-by-file
- **Chose:** Delete the five root release test files and remove their entries from `tests/unit.rs`.
- **Why:** The release family crate already owns broad rule-local coverage, including repo rules, crate metadata rules, binary rules, readme/keyword/category cases, workflow parsing, workspace inheritance, and input-failure cases. Leaving the root copies in place just preserved monolith pressure.
- **Alternatives considered:**
  - Move each old root release test into the release crate one-by-one — rejected because the owner crate already has broader tests and this would mostly re-home legacy helper coverage rather than strengthen the new architecture.
  - Leave the root tests until all legacy validator code is deleted — rejected because the plan requires root-harness reduction in parallel, not after the fact.

### Treat the release family crate as the proof boundary
- **Chose:** Verify `guardrail3-app-rs-family-release` directly after the deletion.
- **Why:** The architectural goal is independent crate-local testing. The correct question is whether the owner crate is green, not whether the root harness still compiles the same cases.
- **Alternatives considered:**
  - Run the root harness again — rejected because that reintroduces the exact bottleneck this cleanup is meant to remove.

## Architectural Notes
This is the first removal in this session that deletes a large root test cluster outright rather than relocating individual test files. That is acceptable here because the owner crate already has stronger localized coverage.

After this change:
- `tests/unit.rs` is smaller again
- the root harness no longer carries the release family
- release testing pressure is concentrated on `guardrail3-app-rs-family-release`, which is the correct build/test boundary

This is directly in line with the split plan’s claim that root-test shrink matters as much as crate promotion.

## Information Sources
- `.plans/todo/checks/2026-03-24-guardrail3-workspace-crate-split.md` — root-test decomposition requirement
- `apps/guardrail3/tests/unit.rs` — root harness inventory before deletion
- `apps/guardrail3/tests/unit/test_release_checks.rs`
- `apps/guardrail3/tests/unit/test_release_crate_checks.rs`
- `apps/guardrail3/tests/unit/test_release_crate_deps.rs`
- `apps/guardrail3/tests/unit/test_release_repo_checks.rs`
- `apps/guardrail3/tests/unit/test_release_bin_checks.rs`
- `apps/guardrail3/crates/app/rs/families/release/src/*` — owner-local release rules and test suites
- `.worklogs/2026-03-25-110905-move-ast-and-report-tests-off-root.md` — immediately prior root-harness reduction batch

## Open Questions / Future Considerations
- The same deletion logic may apply to other legacy root clusters, but only where the owner crate clearly has broader localized coverage already.
- The remaining root harness is now more concentrated in `allow`/`code quality`/`deny inventory`/`dependency allowlist`/`test`/`garde`/`hexarch`/`rs_arch_01` and TS legacy buckets.
- Some remaining root tests still import direct report/fs concerns through `guardrail3::`; even before moving them, those can be narrowed to direct crate imports.

## Key Files for Context
- `apps/guardrail3/tests/unit.rs` — current root harness after the release-cluster deletion
- `apps/guardrail3/crates/app/rs/families/release/src/lib.rs` — release family owner entrypoint
- `apps/guardrail3/crates/app/rs/families/release/src/test_support.rs` — release family crate-local test substrate
- `.plans/todo/checks/2026-03-24-guardrail3-workspace-crate-split.md` — authoritative split plan
- `.worklogs/2026-03-25-110905-move-ast-and-report-tests-off-root.md` — previous batch in the same reduction stream

## Next Steps / Continuation Plan
1. Re-scan the remaining `tests/unit.rs` entries and rank them by how much root-only legacy surface they still drag in.
2. Prefer the next Rust-only cluster that already has a promoted owner crate and non-trivial owner-local tests, so more of `tests/unit.rs` can be removed instead of migrated.
3. For tests that cannot be deleted yet, switch any remaining `guardrail3::domain::report` and `guardrail3::adapters::outbound::fs` imports to direct crate imports as an intermediate step.
4. Keep proving owner crates directly with targeted `cargo test -p ... --lib` runs and only use the workspace-wide `cargo check --workspace --lib` as the broad regression gate.
