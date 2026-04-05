# Add TODO Ledgers To Content-Check Packages

**Date:** 2026-04-05 13:49
**Scope:** `packages/g3-deny-content-checks/TODO.md`, `packages/g3-fmt-content-checks/TODO.md`, `packages/g3-toolchain-content-checks/TODO.md`

## Summary
Added package-local `TODO.md` files to the existing content-check packages so known defects and boundary issues are recorded next to the packages they affect. This captures the deny extraction attack findings plus the current fmt/toolchain compile drift against parser API changes.

## Context & Problem
Recent extraction work left package-specific follow-up work that was only present in transient conversation context. The user asked to start writing `TODO.md` files into each family's content-check package to record everything found broken for future work. The immediate known issues were:

- deny structural signaling gaps around typed parser rejection
- deny package test coverage absence
- fmt/toolchain compile drift after `cargo-toml-parser` changed to `InheritableValue<String>`

Without package-local ledgers, these issues would be easy to lose between sessions.

## Decisions Made

### Record issues at package roots
- **Chose:** Add one `TODO.md` at the root of each existing content-check package.
- **Why:** The problems are package-specific, and the package root is the most reliable place for future sessions to see them.
- **Alternatives considered:**
  - Centralize in one repo-level TODO file — rejected because it weakens package-local ownership.
  - Leave issues only in worklogs — rejected because worklogs are historical records, not active package ledgers.

### Keep TODO entries concrete and boundary-focused
- **Chose:** Write only issues that were actually observed, with file references and follow-up direction.
- **Why:** These TODOs are meant to be actionable, not speculative design notes.
- **Alternatives considered:**
  - Add broader future ideas for each package — rejected because that would dilute the immediate bug list.
  - Mirror full worklog detail into TODOs — rejected because it would duplicate context unnecessarily.

## Architectural Notes
The TODO files reinforce the current extraction boundary:

- content-check packages should only consume valid typed parsed files
- structural/orchestrator layers own parse failure and malformed-schema signaling
- package TODOs should therefore distinguish package bugs from app-boundary bugs

This was especially important for the deny findings, where malformed typed-parse rejection belongs to the app family rather than the content package itself.

## Information Sources
- `packages/g3-deny-content-checks/`
- `packages/g3-fmt-content-checks/`
- `packages/g3-toolchain-content-checks/`
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/run.rs`
- `apps/guardrail3/crates/app/rs/families/deny/crates/runtime/src/facts/mod.rs`
- `cargo test --manifest-path packages/g3-deny-content-checks/Cargo.toml --workspace -- --list`
- `cargo test --manifest-path packages/g3-fmt-content-checks/Cargo.toml --workspace -- --list`
- `cargo test --manifest-path packages/g3-toolchain-content-checks/Cargo.toml --workspace -- --list`

## Open Questions / Future Considerations
- Whether deny malformed-schema expectations should be reintroduced as dedicated structural rule IDs or attached to existing structural deny reporting.
- Whether all future content-check packages should require a `TODO.md` from initial scaffold time rather than adding them later.

## Key Files for Context
- `packages/g3-deny-content-checks/TODO.md` — deny package-local follow-up items from the extraction attack.
- `packages/g3-fmt-content-checks/TODO.md` — fmt compile drift and boundary reminders.
- `packages/g3-toolchain-content-checks/TODO.md` — toolchain compile drift and boundary reminders.
- `.worklogs/2026-04-05-131948-g3-rename-and-deny-content-checks.md` — prior extraction worklog that provides deny package background.

## Next Steps / Continuation Plan
1. Fix the `cargo-toml-parser` API drift in `g3-fmt-content-checks` and `g3-toolchain-content-checks`, then rerun their package and app-family tests.
2. Add structural deny findings for typed parser/schema rejection in the app deny family before calling `g3-deny-content-checks`.
3. Add direct package tests to `g3-deny-content-checks` so migrated rule behavior is exercised inside the package itself.
