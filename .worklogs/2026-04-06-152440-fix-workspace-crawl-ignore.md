# Fix workspace crawl: correct ignore semantics + targeted recovery

**Date:** 2026-04-06 15:24
**Scope:** `packages/g3rs-workspace-crawl/crates/runtime/src/**`

## Summary
Replaced the broken single-root gitignore matcher with a two-phase crawl: `ignore::WalkBuilder` for correct ancestor+nested gitignore semantics, plus targeted `walkdir` recovery of ignored-but-relevant config/manifest files.

## Decisions Made

### Use ignore::WalkBuilder instead of manual Gitignore matching
- **Chose:** Phase 1 walks with `WalkBuilder` configured for repo-local semantics only (`git_global=false`, `git_exclude=false`, `parents=true`, `hidden=false`, `ignore=false`).
- **Why:** The previous implementation only loaded the workspace-root `.gitignore` via `GitignoreBuilder`. It missed ancestor gitignores (above the workspace) and nested gitignores (inside subdirectories). The `WalkBuilder` handles the full ignore stack internally — ancestor chain, nested scoping, negation patterns — without reimplementation.
- **Alternatives rejected:** Two-walk diff approach (walk everything with walkdir, diff against WalkBuilder output to label ignored entries) — rejected because it walks into `target/` and `node_modules/`, bloating the result with thousands of useless ignored entries.

### Targeted recovery for ignored config files
- **Chose:** Phase 2 walks with `walkdir`, skips banned directories (`.git`, `target`, `node_modules`), and recovers only files matching a hardcoded recovery list (same as old app's `should_cache`/`should_recover_ignored`).
- **Why:** The `WalkBuilder` skips ignored entries entirely — no flag to emit them. But ignored config files (Cargo.toml, .clippy.toml, etc.) matter for validation. Recovery avoids silent invisibility while staying bounded.
- **Alternatives rejected:** Making recovery pluggable per-family — adds API complexity for no current need. The recovery list is just data, not logic.

### Machine-independent validation semantics
- **Chose:** `git_global=false`, `git_exclude=false` — only checked-in `.gitignore` files affect ignore state.
- **Why:** If validation depends on `~/.config/git/ignore` or `.git/info/exclude`, results vary per machine and CI environment.

## Key Files for Context
- `packages/g3rs-workspace-crawl/crates/runtime/src/crawl.rs` — two-phase crawl logic
- `packages/g3rs-workspace-crawl/crates/runtime/src/recovery.rs` — recovery list, banned dirs
- `packages/g3rs-workspace-crawl/crates/runtime/src/support.rs` — simplified entry builder
- `packages/g3rs-workspace-crawl/crates/runtime/src/crawl_tests/ignore_state.rs` — 8 tests covering nested/ancestor/negation/recovery/banned semantics

## Next Steps
1. Build `g3rs-cargo-config-ingestion` using `&G3RsWorkspaceCrawl` as input
2. The crawl is ready to feed ingestion packages with correct filesystem visibility
