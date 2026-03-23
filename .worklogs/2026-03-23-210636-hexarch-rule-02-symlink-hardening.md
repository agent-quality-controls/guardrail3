# Hexarch Rule 02 Symlink Hardening

**Date:** 2026-03-23 21:06
**Scope:** `apps/guardrail3/crates/app/core/project_walker.rs`, `apps/guardrail3/crates/domain/project_tree.rs`, `apps/guardrail3/crates/domain/project_tree_tests.rs`, `apps/guardrail3/crates/app/rs/checks/rs/hexarch/`, shared Rust-family `test_support.rs` helpers, `.plans/todo/check_review/test_hardening/{01-hexarch.md,11-hexarch-agent-brief.md}`

## Summary
This pass closed `RS-HEXARCH-02` under repeated adversarial attack rounds by fixing two real symlink-related blind spots and deepening the rule-specific sidecar suite to 43 tests. The key library hardening is that `ProjectTree` now preserves immediate symlink children, including broken child symlinks that the main walker can omit when following links, so rule 02 can enforce exact top-level shape without losing filesystem truth.

## Context & Problem
The user explicitly wanted exhaustive adversarial validation, not a shallow “tests are green” stop condition. During repeated 4-agent rounds on `RS-HEXARCH-02`, we found that the exact-contents rule was still vulnerable in two ways:
- after enabling `follow_links(true)` for rule-01/top-level `crates` symlink support, a required child symlink like `crates/domain -> crates/app` could be accepted by name unless symlink identity was preserved
- a symlink named `.gitkeep` was being exempted as if it were the one allowed real `.gitkeep` file

The first attempt fixed visible symlink children by recording `symlink_dirs` / `symlink_files` in `ProjectTree`, but targeted `rs_hexarch_02` execution exposed that broken child symlinks still disappeared from the walker output entirely. That meant the rule could still under-observe one important class of malformed top-level entry. In parallel, one of the new compound tests was itself wrong because it destroyed the nested root path before mutating it.

## Decisions Made

### Preserve immediate symlink children in `ProjectTree`
- **Chose:** Extend `DirEntry` with `symlink_dirs` / `symlink_files`, keep `follow_links(true)`, and add a filesystem-backed patch pass that rescans the immediate children of every discovered directory to record symlinks that the main walker may omit, especially broken symlinks.
- **Why:** The library needs to preserve filesystem truth for structural rules. Following links is still needed so a valid top-level `crates` directory symlink remains discoverable for rule 01, but rule 02 also needs to know that a required child entry is a symlink rather than a real template directory.
- **Alternatives considered:**
  - Revert `follow_links(true)` — rejected because that would break the earlier rule-01 fix for valid top-level `crates` directory symlinks.
  - Relax the broken-symlink tests to “missing only” — rejected because it would discard real information about a stray non-directory object occupying the required slot.
  - Detect broken symlinks only inside hexarch facts — rejected because preserving structural truth belongs in the shared project snapshot, not in one family’s collector.

### Allow only a real `.gitkeep` file, not a symlinked `.gitkeep`
- **Chose:** Keep `.gitkeep` exemption only for entries coming from `files`, while treating `.gitkeep` entries from `symlink_dirs` / `symlink_files` as bad top-level entries.
- **Why:** The rule intent is “one specific real loose file is allowed,” not “the name `.gitkeep` is magical regardless of object type.”
- **Alternatives considered:**
  - Exempt by filename everywhere — rejected because that reintroduces the symlink loophole the final attack round found.
  - Ban `.gitkeep` entirely — rejected because the fixture and rule semantics intentionally allow a real placeholder file.

### Treat rule-02 test failures as either real semantics or bad mutations, not noise
- **Chose:** Fix the tests that were actually wrong and keep the ones that exposed real semantics holes.
- **Why:** The user wanted a robust library, not weakened assertions for convenience.
- **Alternatives considered:**
  - Drop the broken-symlink expectation — rejected because the new `ProjectTree` patch makes the stronger semantics achievable and correct.
  - Keep the all-four-missing compound test as written — rejected because deleting outer `adapters/` first destroys the nested path and makes the test mutate a non-existent directory.

## Architectural Notes
- `ProjectTree::DirEntry` now carries four parallel facts:
  - `dirs`
  - `files`
  - `symlink_dirs`
  - `symlink_files`
- `project_walker::walk_project` now has three phases:
  1. main `ignore` walk with `follow_links(true)`
  2. tracked-file patch for gitignored-but-tracked files
  3. immediate-child symlink patch using the filesystem port

This keeps symlink identity available to structural families without forcing each family to hit the filesystem directly.

For hexarch specifically:
- `HexRootFacts` / `HexRootInput` now expose the symlink sets
- `RS-HEXARCH-02` treats required child symlinks as:
  - missing required dir
  - plus loose top-level entry
- only non-symlink `.gitkeep` remains exempt

The rule-02 sidecar suite was expanded to 43 tests and now locks down:
- valid, broken, and non-dir child symlink parity
- outer `adapters/` reachability special case
- loose `.gitignore` / loose `Cargo.toml`
- symlinked `.gitkeep`
- mixed outer-root and nested-root compound attacks
- non-owned nested-lookalike boundaries

## Information Sources
- `.plans/todo/check_review/test_hardening/01-hexarch.md`
- `.plans/todo/check_review/test_hardening/11-hexarch-agent-brief.md`
- `.plans/todo/checks/rs/hexarch.md`
- `apps/guardrail3/tests/unit/rs_arch_01/rule_02.rs`
- repeated `test-attack`-style adversarial rounds via subagents on `RS-HEXARCH-02`
- direct verification commands:
  - `cargo check --manifest-path apps/guardrail3/Cargo.toml --lib`
  - `cargo test --manifest-path apps/guardrail3/Cargo.toml rs_hexarch_02 -- --nocapture`

## Open Questions / Future Considerations
- `RS-HEXARCH-03` is now the next incomplete structural rule under the explicit 4-agent repeated-attack protocol.
- The `DirEntry` shape change touched shared test helpers in multiple Rust families. Those updates compile and are intentional, but any future manual `DirEntry` construction must now set the symlink fields too.
- The broader repo worktree remains very dirty due to parallel family work. This commit should stay narrowly scoped to the rule-02 / walker / handoff slice.

## Key Files for Context
- `apps/guardrail3/crates/app/core/project_walker.rs` — shared project snapshot builder, now with immediate symlink-child patching
- `apps/guardrail3/crates/domain/project_tree.rs` — `DirEntry` contract with symlink child fields
- `apps/guardrail3/crates/app/rs/checks/rs/hexarch/facts.rs` — root facts now carry symlink info into hexarch
- `apps/guardrail3/crates/app/rs/checks/rs/hexarch/inputs.rs` — rule inputs now expose symlink sets
- `apps/guardrail3/crates/app/rs/checks/rs/hexarch/rs_hexarch_02_exact_contents.rs` — exact-contents rule logic, including symlink handling and real `.gitkeep` exemption
- `apps/guardrail3/crates/app/rs/checks/rs/hexarch/rs_hexarch_02_exact_contents_tests/` — 43-test adversarial suite for rule 02
- `.plans/todo/check_review/test_hardening/01-hexarch.md` — current lane matrix and rule-02 closure note
- `.plans/todo/check_review/test_hardening/11-hexarch-agent-brief.md` — handoff brief now pointing at `RS-HEXARCH-03`
- `.worklogs/2026-03-23-182954-hardening-wave-and-cargo-unblockers.md` — prior hardening/context baseline for this lane

## Next Steps / Continuation Plan
1. Commit this rule-02/walker slice only, leaving unrelated family work untouched.
2. Resume the explicit repeated attack protocol at `RS-HEXARCH-03`:
   - read current rule and old corpus `apps/guardrail3/tests/unit/rs_arch_01/rule_03.rs`
   - launch 4 focused attack agents
   - patch rule/test gaps until the final fresh round stops finding meaningful improvements
3. Keep `01-hexarch.md` and `11-hexarch-agent-brief.md` current after each completed rule.
