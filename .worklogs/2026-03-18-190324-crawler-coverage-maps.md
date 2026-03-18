# Filesystem crawler + project map + clippy coverage map

**Date:** 2026-03-18 19:03
**Scope:** New crawl module, project_map module, coverage/clippy module, map command, cleanup

## Summary
Built the foundation for project-aware guardrail3: a filesystem crawler using the `ignore` crate (ripgrep's walker), a project structure builder, and a clippy.toml coverage map that shows which crates are covered by which clippy.toml.

## Context & Problem
guardrail3's discovery was a mess of hardcoded path assumptions (`apps/*/`, `crates/`, `apps/backend/`) with fallback chains. It missed standalone crates (substack-publisher), couldn't find nested workspaces deeper than one level, didn't discover TS packages, and used a `primary_workspace_root()` that pointed to the wrong workspace.

## Decisions Made

### Crawler model (ignore crate)
- **Chose:** Single filesystem walk using `ignore` crate, collecting every file guardrail3 cares about
- **Why:** No hardcoded path assumptions. Works with any project layout. .gitignore filtering is built-in (skips node_modules, target, etc. automatically). Battle-tested (ripgrep's walker).
- **Alternatives considered:**
  - Keep walkdir + manual ignore patterns — rejected: fragile, must hardcode every ignore path
  - Recursive glob per file type — rejected: multiple walks instead of one

### Coverage map as verification tool
- **Chose:** `guardrail3 map --clippy` shows a tree of workspaces/packages with clippy.toml coverage annotations
- **Why:** Need to verify our own discovery is correct before building generate/validate on top of it. Shows enforcement gaps (uncovered crates) at a glance.

### Project cleanup
- Deleted `apps/guardrail3/local/` (old override dir, replaced by `.guardrail3/overrides/`)
- Deleted `apps/guardrail3/guardrail3/` (leftover from wrong path resolution)
- Moved `golden-tests/` → `tests/golden-tests/`

## Architectural Notes
- `src/app/crawl.rs` — single walk, flat CrawlResult of file paths
- `src/app/project_map.rs` — builds RustScope/TsScope tree from CrawlResult
- `src/commands/coverage/clippy.rs` — renders clippy coverage tree, documents clippy's actual config resolution rules
- `src/commands/map.rs` — general project map display using ProjectMap

## Key Files for Context
- `apps/guardrail3/src/app/crawl.rs` — the crawler
- `apps/guardrail3/src/app/project_map.rs` — structure builder
- `apps/guardrail3/src/commands/coverage/clippy.rs` — clippy coverage tree
- `apps/guardrail3/src/commands/map.rs` — general map command
- `.plans/by_file/shared/discovery.md` — discovery design plan
- `.plans/by_file/rs/clippy-toml.md` — clippy.toml per-file plan

## Next Steps
1. Build coverage maps for deny.toml, rustfmt.toml, eslint, tsconfig, stylelint, cspell, npmrc
2. Each in its own file under `src/commands/coverage/`, with documented resolution rules
3. Use ProjectMap as the foundation for generate and validate commands
