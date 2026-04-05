# Deny Content Tests Refactor

**Date:** 2026-04-05 16:52
**Scope:** `packages/g3-deny-content-checks`

## Summary
Validated and packaged the dirty deny changes as a coherent refactor of `g3-deny-content-checks` from flat rule files into the project’s one-rule/one-sidecar-directory layout. The extracted deny package is green at the package layer and still compatible with the app deny family.

## Context & Problem
After the cargo commit, the remaining dirty tree was dominated by `packages/g3-deny-content-checks`. The package showed a large delete/add pattern: old flat runtime rule files were removed and replaced by per-rule directories, sidecar test directories, and a parallel assertions surface. That kind of broad file churn is easy to misread as half-finished unless the package still compiles, tests, and matches the extracted-family architecture.

The task here was to decide whether the deny package was actually commit-ready or whether it still needed follow-up fixes before committing.

## Decisions Made

### Treated the deny package changes as a standalone commit candidate
- **Chose:** evaluate `packages/g3-deny-content-checks` independently from unrelated dirty files like `.gitignore` and handoff worklogs.
- **Why:** The deny package changes are internally coherent and testable on their own, while the other dirty files are separate concerns and would muddy the commit.
- **Alternatives considered:**
  - Rolling all remaining dirty files into one commit — rejected because it would mix code, handoff docs, and unrelated local edits.
  - Leaving deny uncommitted despite green tests — rejected because the package refactor is substantial, internally consistent, and easier to reason about as its own changeset.

### Accepted the deny refactor as architecture-aligned
- **Chose:** keep the new per-rule directory + sidecar test layout.
- **Why:** It matches the repo’s extracted-family pattern better than the old flat-file layout and now has strong package-local coverage.
- **Alternatives considered:**
  - Reverting to the old flat runtime files — rejected because the new layout is cleaner and already proven green.
  - Requiring additional app-side changes before committing — rejected because app-family deny tests already pass unchanged, which shows the package boundary remained compatible.

## Architectural Notes
The dirty deny work does not appear to violate the extracted-package boundary:
- package still takes typed `DenyToml` input
- package runtime still owns only content rules
- app deny family remains the owner of structural/profile-sensitive rules
- package-local tests now directly exercise the extracted rules instead of relying only on app-family coverage

The changed package shape is primarily a runtime/tests reorganization plus assertions scaffolding, not a semantic expansion of package inputs.

## Information Sources
- `git status --short --untracked-files=all` — remaining dirty tree inventory after cargo commit
- `git diff --name-status -- packages/g3-deny-content-checks` — deny package-only file churn
- `cargo test --workspace --manifest-path packages/g3-deny-content-checks/Cargo.toml` — package verification (`121` runtime tests passing)
- `cargo test --manifest-path apps/guardrail3/Cargo.toml -p guardrail3-app-rs-family-deny --lib` — app-family compatibility verification (`46` tests passing)
- `packages/g3-deny-content-checks/crates/runtime/src/lib.rs` and `run.rs` — package entrypoint and module ownership
- `packages/g3-deny-content-checks/crates/assertions/src/lib.rs` — package assertions surface

## Open Questions / Future Considerations
- There are still unrelated dirty files outside the deny package:
  - `.gitignore`
  - `.worklogs/2026-04-04-150600-session3-handoff.md`
  - several untracked handoff/worklog files under `.worklogs/`
  These were intentionally left out of the deny commit.
- The deny package TODO changed as part of the refactor; future work should keep it synced with app-side structural parse ownership expectations.

## Key Files for Context
- `packages/g3-deny-content-checks/crates/runtime/src/lib.rs` — runtime module surface after the refactor
- `packages/g3-deny-content-checks/crates/runtime/src/run.rs` — package entrypoint
- `packages/g3-deny-content-checks/crates/assertions/src/lib.rs` — assertion modules for package-local tests
- `packages/g3-deny-content-checks/TODO.md` — known remaining follow-ups for the package
- `.worklogs/2026-04-05-131948-g3-rename-and-deny-content-checks.md` — earlier deny extraction work
- `.worklogs/2026-04-05-164952-cargo-content-checks-extraction.md` — immediate prior commit context so the sequence stays clear

## Next Steps / Continuation Plan
1. Stage and commit only `packages/g3-deny-content-checks` plus this worklog.
2. Leave `.gitignore`, the session handoff worklog, and the extra untracked handoff files out of the commit.
3. After the deny commit, re-run `git status --short --untracked-files=all` to confirm what unrelated local dirt remains and report it cleanly.
