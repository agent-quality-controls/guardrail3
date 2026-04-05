# Deny Content Package Tests

**Date:** 2026-04-05 16:53
**Scope:** `packages/g3-deny-content-checks`

## Summary
Finished the direct test surface for `g3-deny-content-checks` by converting the migrated deny rules to the standard per-rule directory layout, adding package-local assertions, and wiring rule-specific sidecar test directories. The package now verifies its own migrated content rules without depending on the app deny family as the primary test surface.

## Context & Problem
`g3-deny-content-checks` had already absorbed the migrated content rules, but it still had a major structural gap: the package itself had no direct tests. That meant deny behavior was mostly validated indirectly through app-family tests, which weakens the extracted-package architecture and makes it harder to trust the package as an isolated unit.

The desired direction for these extracted packages is:
- package-local tests for package-local rules
- one rule per file
- sidecar `*_tests/` directories close to each rule
- app family tests reserved for orchestration/bridge behavior

## Decisions Made

### Converted deny runtime rules to directory-per-rule layout
- **Chose:** replace flat rule files like `rs_deny_04_deprecated_advisories.rs` with `rs_deny_04_deprecated_advisories/mod.rs` + `rule.rs` plus sidecar test directories.
- **Why:** This matches the extracted-family pattern already used elsewhere and keeps rule ownership/test ownership explicit.
- **Alternatives considered:**
  - Keep flat rule files and add grouped package tests — rejected because it would drift from the repo’s one-rule/one-test-surface pattern.
  - Rely only on app-family tests — rejected because it leaves the package under-tested as an independent unit.

### Added package-local assertions crate coverage
- **Chose:** populate `packages/g3-deny-content-checks/crates/assertions/src/` with per-rule assertion helpers and shared `common.rs`.
- **Why:** The runtime package tests need a stable assertion layer similar to the other extracted packages.
- **Alternatives considered:**
  - Assert directly in every test module — rejected because it would duplicate result-shape checking across 22 migrated rules.

### Kept malformed parse ownership out of the package
- **Chose:** package tests parse valid typed `DenyToml` inputs and focus on content semantics only.
- **Why:** Structural parse failure remains app-owned by design; the package should not widen back into malformed-input handling.
- **Alternatives considered:**
  - Add malformed-schema tests inside the package — rejected because that would blur the app/package boundary.

## Architectural Notes
This deny change strengthens the extracted-package architecture rather than changing the deny family boundary itself:

- app deny family still owns coverage, shadowing, policy-context, and parse-failure behavior
- `g3-deny-content-checks` owns pure typed `deny.toml` content rules
- package tests now directly exercise those content rules
- app-family tests remain as bridge/orchestration verification, not the only correctness layer

## Information Sources
- `packages/g3-deny-content-checks` current package layout and TODO notes
- existing extracted package patterns from `g3-clippy-content-checks` and `g3-cargo-content-checks`
- `.plans/2026-04-05-151043-deny-content-tests.md` — test-surface plan for the deny package
- prior extracted-family worklogs, especially `.worklogs/2026-04-05-145142-clippy-extraction-and-parser-contract-fixes.md`

## Open Questions / Future Considerations
- The deny package tests are now strong, but malformed typed-parse rejection still needs to remain explicit at the app boundary whenever parser contracts change.
- There are still unrelated dirty files in the repo (`.gitignore`, old handoff worklogs, auxiliary untracked worklogs/plans) that should be handled separately and not bundled with deny.

## Key Files for Context
- `packages/g3-deny-content-checks/crates/runtime/src/lib.rs` — runtime module surface after the directory-per-rule conversion
- `packages/g3-deny-content-checks/crates/assertions/src/lib.rs` — package-local assertion exports
- `packages/g3-deny-content-checks/crates/assertions/src/common.rs` — shared assertion helpers
- `packages/g3-deny-content-checks/TODO.md` — remaining known package issues
- `.plans/2026-04-05-151043-deny-content-tests.md` — test plan that drove this package-local hardening
- `.worklogs/2026-04-05-145142-clippy-extraction-and-parser-contract-fixes.md` — prior package-boundary precedent

## Next Steps / Continuation Plan
1. Audit whether `g3-deny-content-checks/TODO.md` still reflects the current gaps after this test migration and trim stale TODO entries if necessary.
2. Continue the family-extraction sequence from the next unfinished Rust family, now that `fmt`, `toolchain`, `clippy`, `deny`, and `cargo` all have extracted content-package coverage in place.
3. Keep future deny changes split from unrelated repo dirt; there are still leftover handoff/worklog files outside this package that should be evaluated independently.
