# Split Ingestion Entry Points By Input Domain

**Date:** 2026-04-07 16:10
**Scope:** `.gitignore`; `packages/rs/{cargo,clippy,deny,fmt,garde,release,toolchain}/g3rs-*-config-ingestion`; `packages/rs/{cargo,clippy,deny,fmt,garde,release,toolchain}/g3rs-*-types`; lockfiles for touched package workspaces

## Summary
Reworked every current Rust config-ingestion package so the public API is split by input domain: `ingest_config`, `ingest_ast`, and `ingest_file_tree`. Removed the old `ingest` alias entirely, updated all current tests/callers to use `ingest_config`, and added explicit typed stubs plus explicit not-yet-implemented errors for AST and file-tree ingestion.

The same pass also fixed two test harness issues that surfaced during verification and generalized `.gitignore` so nested Cargo `target/` directories stop polluting the worktree during per-package builds.

## Context & Problem
The existing ingestion packages exposed a single `ingest(...)` entry point, but the project direction is now to model ingestion by the kind of check input being produced: config, AST, or file-tree. Keeping one undifferentiated method would make later AST/file-tree work harder to wire consistently, and preserving a compatibility alias would let callers keep using the old shape indefinitely.

At the same time, the repo does not yet have a general AST/file-tree ingestion layer. The only real AST checks package currently present is garde AST checks. The ingestion API therefore needed to establish the final surface area now, while still being honest about what is and is not implemented.

During verification, repeated per-package builds also recreated many nested `target/` directories that were not being ignored because the root ignore rule only matched shallow `packages/*/target` paths, not the grouped package layout introduced earlier in the day.

## Decisions Made

### Split ingestion APIs by domain now
- **Chose:** expose `ingest_config`, `ingest_ast`, and `ingest_file_tree` from every current ingestion facade/runtime.
- **Why:** this makes the future contract explicit now and forces all new callers to state which input domain they want.
- **Alternatives considered:**
  - Keep a single `ingest` and add more methods later — rejected because it preserves ambiguity and delays call-site cleanup.
  - Keep `ingest` as a compatibility alias to `ingest_config` — rejected because the user explicitly wanted the old name removed rather than tolerated.

### Keep config ingestion concrete, stub the other two
- **Chose:** keep current config ingestion behavior fully working; return explicit typed errors for AST/file-tree ingestion until those pipelines exist.
- **Why:** config ingestion already exists and is covered by tests, while AST/file-tree packages are not implemented yet across the repo.
- **Alternatives considered:**
  - Add placeholder methods that panic or use `todo!()` — rejected because the workspace lints deny that style and it hides the intended API contract behind runtime failure.
  - Delay adding AST/file-tree methods until real implementations exist — rejected because the user wanted the shape established now.

### Add named placeholder input types to family type crates
- **Chose:** define placeholder `*AstChecksInput` and `*FileTreeChecksInput` structs in the family types crates, with garde AST using the already-existing real garde AST input type.
- **Why:** callers and future implementations now have stable return types instead of opaque placeholders or generic unit values.
- **Alternatives considered:**
  - Return `()` or generic placeholders from stub methods — rejected because that does not encode the intended future contract.
  - Duplicate garde AST types in `g3rs-garde-types` — rejected because a real garde AST checks input package already exists and duplicating the contract would create drift.

### Make stub status explicit in ingestion error enums
- **Chose:** add `AstIngestionNotImplemented` and `FileTreeIngestionNotImplemented` variants to each ingestion error type.
- **Why:** these methods need a stable failure mode that is descriptive, typed, and lint-compliant.
- **Alternatives considered:**
  - Reuse existing parse or missing-file errors — rejected because those would misdescribe the actual failure mode.
  - Introduce one shared cross-family stub error type — rejected because the current ingestion packages already expose family-local error enums and changing that would widen scope.

### Remove the old `ingest` alias entirely
- **Chose:** delete all facade/runtime alias exports and update all tests to call `ingest_config`.
- **Why:** the user explicitly requested no preserved alias, and this avoids future mixed usage across packages.
- **Alternatives considered:**
  - Leave both names available temporarily — rejected because it would keep the migration half-finished.

### Fix verification issues encountered during the API change
- **Chose:** fix the `fmt` runtime test type import and the `toolchain` real-workspace fixture traversal during this change.
- **Why:** both blocked green verification of the ingestion workspaces and were revealed by the requested refactor.
- **Alternatives considered:**
  - Ignore failing tests as unrelated — rejected because that would leave the commit without reliable verification.
  - Revert the broader test run and only compile-check changed crates — rejected because the API change touched all ingestion packages and the runtime test suites were the right signal.

### Generalize the root `target/` ignore rule
- **Chose:** replace the shallow path-specific ignore entries with a generic `target/` rule in the repo root `.gitignore`.
- **Why:** the current grouped package layout creates nested build directories across `packages/parsers`, `packages/shared`, `packages/rs`, and `apps/guardrail3`; the old pattern did not match them.
- **Alternatives considered:**
  - Add more path-specific ignore globs for each new package depth — rejected because it is brittle and unnecessary when Cargo build output is universally named `target/`.
  - Leave build output unignored and clean manually — rejected because it makes the worktree noisy during normal development.

## Architectural Notes
This commit establishes the ingestion interface the rest of the new pipeline can target:

```text
ingest_config(crawl) -> current config checks input
ingest_ast(crawl) -> future AST checks input
ingest_file_tree(crawl) -> future file-tree checks input
```

The shape is now uniform across all current ingestion packages, even though only config ingestion is implemented broadly today. This keeps the family boundary explicit and aligns with the wider pipeline direction described in the session handoff.

Current AST/file-tree reality after this commit:
- Config ingestion exists for `cargo`, `clippy`, `deny`, `fmt`, `garde`, `release`, `toolchain`
- A real AST checks package exists only for `garde`
- No dedicated AST ingestion packages exist yet
- No file-tree checks or file-tree ingestion packages exist yet

## Information Sources
- `AGENTS.md` — worklog rules and current project direction
- `.worklogs/2026-04-07-150226-session-handoff.md` — current pipeline status and remaining gaps
- `.worklogs/2026-04-07-111614-remove-target-artifacts.md` — prior accidental build-artifact staging issue
- `packages/rs/*/g3rs-*-config-ingestion/src/lib.rs` and `crates/runtime/src/{lib.rs,run.rs}` — current ingestion API shape
- `packages/rs/*/g3rs-*-types/src/lib.rs` — current family input contracts
- `packages/rs/garde/g3rs-garde-ast-checks` — only existing real AST checks package
- `cargo test --workspace -q` run in each touched ingestion package workspace

## Open Questions / Future Considerations
- `ingest_ast` and `ingest_file_tree` are intentionally stubs for most families; the next implementation pass should decide whether those become separate package families or remain methods inside the current ingestion packages.
- Only garde currently has a real AST checks contract. If more AST check packages are added, the placeholder types in family type crates should be replaced by the real package-owned input contracts rather than duplicated.
- File-tree checks do not exist yet at all, so the future package naming and ownership pattern is still open.
- The repo still has per-package `Cargo.lock` churn because each package workspace is currently committed independently; if that policy changes later, many of the lockfiles in this commit may become unnecessary.

## Key Files for Context
- `packages/rs/cargo/g3rs-cargo-config-ingestion/src/lib.rs` — representative ingestion facade export shape
- `packages/rs/cargo/g3rs-cargo-config-ingestion/crates/runtime/src/run.rs` — representative three-method runtime API
- `packages/rs/cargo/g3rs-cargo-types/src/lib.rs` — representative config/AST/file-tree input contract layout
- `packages/rs/garde/g3rs-garde-config-ingestion/crates/runtime/src/run.rs` — special-case family that reuses the real garde AST checks input type
- `packages/rs/garde/g3rs-garde-ast-checks/crates/types/src/lib.rs` — current real AST input contract in the repo
- `packages/rs/toolchain/g3rs-toolchain-config-ingestion/crates/runtime/src/ingest_tests/real_workspaces.rs` — updated real-workspace verification against the grouped package tree
- `.gitignore` — generalized build-output ignore rule
- `.worklogs/2026-04-07-150226-session-handoff.md` — broader pipeline status and pending work after this API split

## Next Steps / Continuation Plan
1. Decide the actual AST ingestion packaging pattern, starting with garde since it already has a real AST checks input contract and runtime.
2. Replace placeholder `*AstChecksInput` structs in the other family type crates only when a real AST checks package exists for that family.
3. Define the first real file-tree checks package before implementing any `ingest_file_tree` method body; do not invent file-tree discovery without a corresponding checks contract.
4. Continue the pending `deps` pipeline work after this API split, including extracting a standalone parser for repo-level `guardrail3.toml`.
