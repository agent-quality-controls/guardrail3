# Plan code AST profile resolution

**Date:** 2026-04-09 12:40
**Scope:** `.plans/todo/checks/2026-04-09-code-ast-profile-resolution.md`, `packages/rs/code/g3rs-code-ast-ingestion/TODO.md`

## Summary
Wrote the implementation plan for `code` AST profile resolution. The plan makes `g3rs-code-ast-ingestion` responsible for classifying Rust source files as library or binary owned files and for marking the actual library root file, so the remaining library-sensitive `code` AST rules can migrate without pushing Cargo/workspace discovery into the checks runtime.

## Context & Problem
Most single-file `code` AST rules are already migrated. The remaining rules are the ones that only make sense for library API surface, such as `pub use foo::*` in `lib.rs`, facade-only `lib.rs`, public field bags, and bad public error forms. The current AST lane carries `profile_name: None`, which means the checks runtime can parse one file but cannot know whether that file belongs to a library target or whether it is the actual `lib.rs` entrypoint. That missing context is the next blocker.

## Decisions Made

### Keep crate/profile classification in ingestion
- **Chose:** Put source-file-to-target ownership, library/binary classification, and `lib.rs` root detection into `g3rs-code-ast-ingestion`.
- **Why:** This is workspace/Cargo mapping, not AST semantics. If the checks runtime did it, the AST package would start rediscovering crate ownership instead of staying a bounded parser/fanout layer.
- **Alternatives considered:**
  - Let the AST checks runtime infer context from file paths — rejected because raw `src/lib.rs` heuristics are weaker than target ownership and would smear Cargo logic into the checks package.
  - Keep `profile_name` vague and add ad hoc rule exceptions — rejected because the remaining rules need explicit, stable ownership context.

### Use a small explicit source-file contract
- **Chose:** Keep `profile_name` and add `is_library_root` to `G3RsSourceFile`.
- **Why:** The remaining rules need two separate answers: “am I in a library target?” and “am I the actual library root file?” One field is not enough without making rules fall back to path string guessing.
- **Alternatives considered:**
  - Encode everything into `profile_name` strings like `library-root` — rejected because it mixes two axes into one field and makes rule logic clumsy.
  - Add a larger crate context object immediately — rejected because the next rules only need minimal classification, not a full package model.

### Fail closed only when manifest parsing is broken enough to block ownership
- **Chose:** Unknown ownership may stay `None`, but broken manifest parsing for owned Rust code should fail ingestion.
- **Why:** Non-profile rules can still run on files with unknown ownership, but profile-sensitive rules would fail open if manifests are broken and ingestion silently guessed.
- **Alternatives considered:**
  - Fail on every unowned file — rejected because not every selected source file needs profile context.
  - Always guess from path shape — rejected because that would silently hide classification errors.

## Architectural Notes
This plan keeps the AST lane split clean:
- workspace crawl discovers files and manifests
- AST ingestion maps file -> owning target context
- AST checks runtime parses source and slices facts
- rules stay small and do not perform Cargo or workspace discovery

The plan also keeps the immediate scope narrow: only `library` vs `binary`, plus whether the file is the library root. It does not try to solve every Cargo target kind up front.

## Information Sources
- `.plans/todo/checks/2026-04-08-ast-checks-package-architecture.md` — AST lane contract
- `.plans/todo/checks/rs/code.md` — remaining `code` rule inventory and which ones are library-sensitive
- `packages/rs/code/g3rs-code-ast-ingestion/README.md` — current package limitation
- `packages/rs/code/g3rs-code-ast-ingestion/TODO.md` — local next-step list before this change

## Open Questions / Future Considerations
- If `code` later needs finer target kinds than library/binary, extend the source-file contract only after a real rule requires it.
- If multiple packages in one workspace expose overlapping ownership edge cases, add focused ingestion tests before expanding the classification model.

## Key Files for Context
- `.plans/todo/checks/2026-04-09-code-ast-profile-resolution.md` — the actual plan for the next implementation step
- `packages/rs/code/g3rs-code-ast-ingestion/TODO.md` — short package-local reminder that points back to the plan
- `.plans/todo/checks/2026-04-08-ast-checks-package-architecture.md` — broader AST lane template
- `.plans/todo/checks/rs/code.md` — which remaining rules need this work
- `.worklogs/2026-04-09-110552-harden-code-ast-after-test-attack.md` — latest stabilized state before this plan

## Next Steps / Continuation Plan
1. Update `G3RsSourceFile` in `g3rs-code-ast-checks-types` to add `is_library_root`.
2. Replace the current `resolve_profile_name(...)` stub in `g3rs-code-ast-ingestion` with Cargo-target-based ownership classification.
3. Add ingestion tests for library root, library module, binary root, mixed lib+bin package, and unknown ownership.
4. After profile resolution is green, migrate the remaining library-sensitive `code` AST rules in this order: `RS-CODE-26`, `27`, `29`, `31`, `33`.
