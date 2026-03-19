# Directory-based coverage maps

**Date:** 2026-03-19 10:30
**Scope:** crawl.rs, coverage/engine.rs, coverage/clippy.rs, coverage/deny.rs, coverage/rustfmt.rs

## Summary
Changed coverage maps from crate-based to directory-based. Crawler now tracks which directories contain .rs, .ts/.tsx/.js, and .css source files. Coverage engine resolves per source directory. Uncovered directories collapsed to top-level ancestors.

## Decisions Made
- **Directory is the coverage unit:** Every file in the same directory resolves to the same config (walk-up starts from same place). No approximations.
- **Collapse uncovered:** If all siblings under a parent are uncovered, collapse to the parent. Repeats until stable. `packages/low-expectations/src/` + `packages/low-expectations/tests/` + `packages/seo-site-files/src/` + `packages/seo-site-files/tests/` → `packages/`.
- **Source dir tracking in crawler:** Cheap — just checks file extension during the walk, adds parent dir to a BTreeSet.

## Key Files
- `apps/guardrail3/src/app/crawl.rs` — dirs_with_rs, dirs_with_ts, dirs_with_css
- `apps/guardrail3/src/commands/coverage/engine.rs` — source_dirs trait method, collapse_to_ancestors
