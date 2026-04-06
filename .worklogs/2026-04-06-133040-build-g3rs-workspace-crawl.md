# Build `g3rs-workspace-crawl`

**Date:** 2026-04-06 13:30
**Scope:** `packages/g3rs-workspace-crawl/**`

## Summary
Scaffolded and implemented the first new stage package, `g3rs-workspace-crawl`, as a real package rather than a shell. It now provides a neutral per-workspace filesystem crawl, shared types, simple query helpers, and package-local tests for hidden config files and `.gitignore` state.

## Context & Problem
The new `g3rs` runtime direction had already been split into three stages:

- `g3rs-workspace-crawl`
- `g3rs-*-ingestion`
- `g3rs-*-checks`

The immediate next move was to build the crawl package first, because every ingestion package will depend on it and the user explicitly wanted shared ignore/recovery/path semantics to live in one place instead of drifting across family-specific ingestion packages.

The package also needed to follow the existing extracted package structure. `g3rs-garde-ast-checks` and `g3rs-fmt-config-checks` were used as the concrete structural specimens for:

- root facade
- `crates/types`
- `crates/runtime`
- `crates/assertions`
- `README.md`
- `TODO.md`
- package-local `Cargo.lock`

## Decisions Made

### Make the first crawl contract intentionally small and neutral
- **Chose:** expose `G3RsWorkspaceCrawl`, `G3RsWorkspaceEntry`, `G3RsWorkspacePath`, `G3RsWorkspaceEntryKind`, and `G3RsWorkspaceIgnoreState`, plus simple query helpers like `entry`, `root_file`, and `files_with_extension`.
- **Why:** the first package needed to centralize filesystem semantics without smuggling family logic into the crawl result. A minimal neutral surface is enough for the first ingestion packages and can be expanded later only when real needs appear.
- **Alternatives considered:**
  - Add family-aware queries immediately — rejected because that would turn crawl into a hidden mapper.
  - Make the crawl result only a bag of strings — rejected because ingestion packages need absolute paths, kinds, ignore state, and readability.

### Implement real `.gitignore` state but keep the first symlink policy conservative
- **Chose:** walk the workspace tree with `walkdir`, compute ignore state with `ignore::gitignore`, include hidden files like `.clippy.toml`, skip `.git` internals, and skip non-file/non-directory entries such as symlinks for now.
- **Why:** hidden config files matter for Rust families, `.gitignore` state must be centralized immediately, and symlink policy was not agreed yet. Skipping symlinks is safer than pretending they are ordinary files.
- **Alternatives considered:**
  - Let each ingestion package decide ignore behavior — rejected because that recreates the exact drift the crawl package is meant to stop.
  - Eagerly follow symlinks — rejected because it changes filesystem semantics before there is a clear policy.
  - Respect gitignore by filtering ignored entries out of the crawl entirely — rejected because ingestion packages may need to know that a path exists but is ignored.

### Keep the package structure aligned with the existing `g3rs-*` pattern
- **Chose:** build the package as a root facade plus `types`, `runtime`, and `assertions` crates.
- **Why:** the user explicitly wanted the new packages to be built the same way the current good packages are built. This keeps the new crawl package structurally compatible with the existing extracted line.
- **Alternatives considered:**
  - Build crawl as a single crate — rejected because it would break the newly agreed stage/type split immediately.
  - Omit the assertions crate “until needed” — rejected because keeping the same package idiom makes the next packages easier to scaffold consistently.

### Clean local arch/code violations instead of normalizing them away
- **Chose:** fix the package-specific `arch/code` violations that were purely structural:
  - split `types/src/lib.rs` into facade-only re-exports
  - remove the fake runtime import from the assertions facade
  - mark internal crates as `shared = true`
  - route file-open/read-dir checks through a tiny `fs.rs` module
  - strengthen weak test `expect(...)` messages
- **Why:** this package is the first specimen for the new line and should not start with avoidable local debt.
- **Alternatives considered:**
  - Ignore the local arch/code findings because older extracted packages still carry some debt — rejected because the new line should start cleaner where the fixes are straightforward.
  - Eliminate all remaining warnings including public-field warnings in `types` — rejected for this pass because the current `g3rs-*` package contracts still use public contract fields and that broader style change should be deliberate.

## Architectural Notes
- The final package shape is:
  - root facade crate exporting `crawl(...)` and crawl types behind `api`
  - `crates/types` with neutral public crawl types
  - `crates/runtime` with crawl logic split into:
    - `crawl.rs`
    - `ignore.rs`
    - `fs.rs`
    - `support.rs`
    - `run.rs`
  - `crates/assertions` with package-local test assertions
- The crawl package stays family-neutral:
  - it does not know about `fmt`, `garde`, or any other family
  - it does not parse config/source files
  - it only exposes neutral filesystem facts and simple lookup helpers
- Local test coverage now proves:
  - hidden config files are included
  - ignored files and directories are still surfaced with `Ignored` state
  - root file and extension queries work

## Information Sources
- `.plans/2026-04-06-g3rs-workspace-crawl-and-ingestion.md` — stage architecture plan
- `.plans/2026-04-06-g3rs-package-structure.md` — concrete package-shape plan
- `packages/g3rs-garde-ast-checks/**` — structural specimen for the package layout
- `packages/g3rs-fmt-config-checks/**` — structural specimen for config-package layout
- validation and test feedback from:
  - `cargo test --workspace --manifest-path packages/g3rs-workspace-crawl/Cargo.toml`
  - `cargo run --quiet --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate packages/g3rs-workspace-crawl --family arch --family code --format json | jq ...`

## Open Questions / Future Considerations
- Symlink handling is still deferred. The package currently skips symlink entries rather than modeling them explicitly.
- The current crawl result still uses public fields, which matches the current `g3rs-*` style but leaves `RS-CODE-31` warnings.
- The assertion helper crate still has one `panic!`-style warning in `common.rs`; it is not blocking, but it is now explicit package-local debt instead of hidden output noise.
- The first crawl only loads `.gitignore` from the explicit workspace root. If later ingestion packages require per-directory gitignore semantics, expand the matcher deliberately rather than sneaking it into ingestion.

## Key Files for Context
- `packages/g3rs-workspace-crawl/Cargo.toml` — root facade package and workspace metadata
- `packages/g3rs-workspace-crawl/src/lib.rs` — facade exports for crawl logic and types
- `packages/g3rs-workspace-crawl/crates/types/src/lib.rs` — facade-only public types crate entrypoint
- `packages/g3rs-workspace-crawl/crates/types/src/entry.rs` — neutral path/entry contract
- `packages/g3rs-workspace-crawl/crates/types/src/crawl.rs` — crawl snapshot contract and query helpers
- `packages/g3rs-workspace-crawl/crates/runtime/src/run.rs` — public crawl entrypoint
- `packages/g3rs-workspace-crawl/crates/runtime/src/crawl.rs` — main crawl assembly logic
- `packages/g3rs-workspace-crawl/crates/runtime/src/ignore.rs` — gitignore-state logic
- `packages/g3rs-workspace-crawl/crates/runtime/src/fs.rs` — centralized runtime filesystem access
- `packages/g3rs-workspace-crawl/crates/runtime/src/crawl_tests/` — first package-local crawl behavior tests
- `.plans/2026-04-06-g3rs-package-structure.md` — source of truth for the package structure
- `.plans/2026-04-06-g3rs-workspace-crawl-and-ingestion.md` — source of truth for the stage pipeline

## Next Steps / Continuation Plan
1. Use `g3rs-workspace-crawl` as the input boundary for the first ingestion package instead of adding more crawl semantics first.
2. Build `packages/g3rs-cargo-config-ingestion` next, using:
   - `&G3RsWorkspaceCrawl` as ingestion input
   - family-local file selection for the workspace root `Cargo.toml`
   - parse-once assembly into the existing `g3rs-cargo-config-checks` input type
3. Keep validating new crawl/ingestion packages with:
   - package-local `cargo test`
   - filtered `arch/code` output scoped to the new package
4. Revisit symlink and per-directory ignore semantics only if the first ingestion package actually needs them.
