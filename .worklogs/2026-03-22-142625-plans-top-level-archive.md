# Add Top-Level Plans Archive

**Date:** 2026-03-22 14:26
**Scope:** `.plans/*.md`

## Summary
Committed the exposed top-level `.plans` archive after removing the old ignore rule. This groups the timestamped historical planning documents and the top-level checker index docs into one archive commit.

## Context & Problem
Removing `.plans/` from `.gitignore` surfaced a large backlog of previously untracked planning material. The user explicitly asked to clean this up and commit all of it in reasonable groups rather than leaving the repository full of exposed untracked documents.

## Decisions Made

### Commit top-level `.plans` files as one archive group
- **Chose:** Batch the root `.plans/*.md` files into a single commit.
- **Why:** These files are mostly timestamped historical plans and top-level architecture/checker snapshots. They belong together as an archive layer.
- **Alternatives considered:**
  - One giant commit for every exposed `.plans` file — rejected because it would hide the structure of the planning corpus.
  - Dozens of tiny commits by day or topic — rejected because the history cleanup would become noise.

## Architectural Notes
This is a repository-history cleanup step, not a product-code change. The purpose is to preserve planning context that was already on disk but previously hidden by ignore rules.

## Information Sources
- `git status --short`
- `find .plans -maxdepth 1 -type f | sort`
- `.worklogs/2026-03-22-140612-docs-handoff-rust-architecture.md`

## Open Questions / Future Considerations
- Additional `.plans` subtrees remain to be committed separately in later archive commits.

## Key Files for Context
- `.gitignore` — `.plans/` ignore was removed earlier
- `.plans/*.md` — top-level historical and index planning documents
- `.worklogs/2026-03-22-140612-docs-handoff-rust-architecture.md` — prior context on why the ignore rule changed

## Next Steps / Continuation Plan
1. Commit `.plans/migration` and `.plans/per-app-config-design` as the design/migration archive group.
2. Commit `.plans/by_file` as the per-file planning reference group.
3. Commit `.plans/todo` as the active todo/audit/check inventory group.
