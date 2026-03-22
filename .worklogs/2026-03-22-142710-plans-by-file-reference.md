# Add By-File Planning References

**Date:** 2026-03-22 14:27
**Scope:** `.plans/by_file/**`

## Summary
Committed the `.plans/by_file` planning references as their own archive group. These documents organize planning by concrete config files and external tool entrypoints rather than by implementation session or todo stream.

## Context & Problem
After the top-level `.plans` archive was committed, the remaining exposed planning material naturally split into by-file references and active todo/audit/check trees. The by-file tree is a distinct kind of reference material and deserves its own commit.

## Decisions Made

### Keep `.plans/by_file` separate from `.plans/todo`
- **Chose:** Commit `.plans/by_file/**` independently.
- **Why:** These files are reference-oriented and map directly to specific files/tools. They are different from the active todo/check inventory documents under `.plans/todo`.
- **Alternatives considered:**
  - Merge by-file and todo plans into one commit — rejected because it would collapse reference docs and active planning into the same history unit.

## Architectural Notes
This commit preserves the research and planning material that ties guardrail enforcement back to concrete files like `Cargo.toml`, `clippy.toml`, `rust-toolchain.toml`, pre-commit hooks, and external tools.

## Information Sources
- `git status --short`
- `find .plans/by_file -type f | sort`
- `.worklogs/2026-03-22-142625-plans-top-level-archive.md`

## Open Questions / Future Considerations
- The active `.plans/todo` tree still remains to be committed as the next archive group.

## Key Files for Context
- `.plans/by_file/rs/*` — Rust config-file planning references
- `.plans/by_file/shared/*` — shared/discovery/hook references
- `.plans/by_file/tools/**` — tool-specific research and edge cases
- `.plans/by_file/ts/*` — TypeScript file references retained as historical material

## Next Steps / Continuation Plan
1. Commit `.plans/todo/**` as the active todo/audit/check inventory group.
2. Re-check the worktree after that; the goal is a clean status aside from intentional future work.
