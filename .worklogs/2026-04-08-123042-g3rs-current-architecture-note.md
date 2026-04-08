# g3rs Current Architecture Note

**Date:** 2026-04-08 12:30
**Scope:** `.plans/todo/checks/2026-04-08-g3rs-current-architecture.md`

## Summary
Added one current architecture note for the new `g3rs` pipeline. The goal was to stop relying on stale scattered plan fragments and capture the actual working template for crawl, ingestion, checks runtime, and tiny rule boundaries.

## Context & Problem
The repo already had many architecture notes, but they were spread across old app-era plans, family-specific ledgers, and package READMEs. The user wanted a simple current statement of how the new package pipeline is supposed to work, especially around where mapping belongs and how config / AST / file-tree lanes relate.

The specific issue was that the repo had enough drift and transitional history that older notes could no longer be treated as reliable source of truth for new work.

## Decisions Made

### Write one fresh current-architecture file
- **Chose:** Add a new focused architecture note under `.plans/todo/checks/`.
- **Why:** The current direction needed one place that states the live intended template without forcing future work to reconstruct it from older scattered documents.
- **Alternatives considered:**
  - Update several old plan files in place — rejected because that would still leave the architecture spread across many locations.
  - Reuse old app-family architecture notes — rejected because they mix current direction with now-stale app-era assumptions.

### Make the note describe the pipeline as two mapping stages
- **Chose:** Define the pipeline as:
  - crawl -> ingestion package input mapping
  - checks runtime -> rule-local input mapping
- **Why:** That matches the actual package direction discussed in-session and explains why rules stay tiny even when a lane needs heavy internal support logic.
- **Alternatives considered:**
  - Put all mapping responsibility only in ingestion — rejected because AST and other checks packages still need support/runtime fan-out from their own bounded input.
  - Put more mapping inside rules — rejected because that breaks the tiny pure-rule contract.

### Make AST scope explicitly variable
- **Chose:** State that AST scope may be file, crate, root, or package depending on the rule family.
- **Why:** The session established that AST is not always one-file, and future package architecture must not hardcode that assumption.
- **Alternatives considered:**
  - Treat AST as one-file-only — rejected because it does not fit garde/test/hexarch-style cross-file facts.
  - Allow repo-wide AST scope by default — rejected because that is too broad and violates the bounded-input design.

## Architectural Notes
The note defines these boundaries:

- `g3rs-workspace-crawl` is neutral and owns no family semantics
- ingestion packages own crawl-to-checks-input mapping for config, AST, and file-tree lanes
- checks package runtimes may do parse-once and support-layer mapping from their own bounded input
- rule files remain pure and minimal

The note also records one known drift item: some current packages, especially garde AST, still reflect transitional behavior and should not be mistaken for the final template automatically.

## Information Sources
- `packages/rs/g3rs-workspace-crawl/README.md`
- `packages/rs/deps/g3rs-deps-config-ingestion/README.md`
- `packages/rs/garde/g3rs-garde-ast-checks/README.md`
- `packages/rs/cargo/g3rs-cargo-config-checks/README.md`
- `.plans/todo/checks/2026-03-21-153251-checker-architecture.md`
- `.plans/todo/checks/rs/code.md`
- `.plans/todo/checks/rs/garde.md`
- `.plans/2026-04-04-142819-family-checks-packages.md`

## Open Questions / Future Considerations
- A dedicated AST package architecture note is still needed; this file only states the general current template.
- Some current package implementations still use transitional boundaries and should be aligned deliberately rather than copied blindly.
- The file-tree lane remains underdefined compared with config and AST.

## Key Files for Context
- `.plans/todo/checks/2026-04-08-g3rs-current-architecture.md` — the new current architecture statement
- `.plans/todo/checks/2026-03-21-153251-checker-architecture.md` — older generic checker architecture baseline
- `.plans/2026-04-04-142819-family-checks-packages.md` — older package-split architecture note
- `.plans/todo/checks/rs/code.md` — clearest current note on streamed one-file AST handling
- `.plans/todo/checks/rs/garde.md` — current detailed ledger for a multi-file AST family
- `.worklogs/2026-04-07-161057-split-ingestion-entrypoints.md` — prior package-ingestion API standardization

## Next Steps / Continuation Plan
1. Write a dedicated AST package architecture note that is independent of current garde implementation details.
2. Define the allowed AST scopes clearly: file, crate, root, package.
3. Define the split between AST ingestion and AST checks runtime support/mapping.
4. Use that note as the template for the first real AST-ingestion package build.
