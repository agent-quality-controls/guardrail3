# Continue RS-CODE Allow Cleanup

**Date:** 2026-03-29 21:36
**Scope:** `apps/guardrail3/crates/domain/modules`, `apps/guardrail3/crates/shared/fs`, `apps/guardrail3/crates/adapters/inbound/cli/help_gen_tests.rs`, `apps/guardrail3/crates/app/rs/ast/src/ast_helpers_tests/mod.rs`, `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/*/cases.rs`

## Summary
Continued the `RS-CODE` allow cleanup by removing another batch of justified-but-unnecessary item-level allows. The main gains came from deleting dead fixture fragments in cargo family case tables, normalizing the remaining fixture constants, and tightening a few small helper/test modules so they no longer needed test-only allow attributes.

## Context & Problem
After the earlier `RS-CODE-03` sweep, the remaining allow-related debt had moved into `RS-CODE-04`: item-level allows that were documented but still often unnecessary. The user explicitly wanted removals and refactors rather than “reason comment” cop-outs, so this pass focused on places where the code shape could be simplified without weakening any rule behavior.

The first easy candidates were small local helpers:
- `domain/modules` still used a `vec_init_then_push` suppression for a fixed registry list.
- `shared/fs` still carried a `manual_ok_err` suppression on a straightforward metadata wrapper.
- CLI help tests still relied on `expect_used` for test navigation.
- `ast_helpers_tests` still had indexing/expect scaffolding that could be rewritten more directly.

The larger structural win was in the cargo family case fixtures. Those files all used the same blanket `dead_code, non_upper_case_globals` allow on four fixture fragments. The lowercase-name half was fake debt, and the dead-code half turned out to be a mix of live fragments and genuinely unused fragments. That made them a good target for real cleanup instead of more exception comments.

## Decisions Made

### Remove dead and style-only allows instead of rewriting reasons
- **Chose:** Refactor call sites and fixture declarations so the existing allows could be deleted.
- **Why:** The user requirement was to stop “coping out” with reasons where code cleanup was possible. These were mostly test/helper surfaces, so the safest fix was to tighten code shape rather than debate policy.
- **Alternatives considered:**
  - Keep the code and only rewrite the reason text — rejected because it would preserve obviously avoidable suppressions.
  - Relax `RS-CODE-04` or reinterpret it more narrowly — rejected because the user explicitly asked for no weakened rules.

### Keep the cargo fixture fragments only when a specific case file actually uses them
- **Chose:** Rename the shared cargo case-table constants to proper constant names and delete any fragment block that was only referenced at its own definition site.
- **Why:** This preserved the useful shared fragments while removing the real dead-code debt that the blanket allow had been hiding.
- **Alternatives considered:**
  - Restore a narrower `#[allow(dead_code)]` on the unused constants — rejected because the fragments were not semantically needed.
  - Convert all fragments to helper functions — rejected because unused helpers would still be dead code and the test files are clearer as static TOML fragments.

### Add a local dev-dependency instead of weakening the cspell JSON-structure test
- **Chose:** Add `serde_json` as a `dev-dependency` to `guardrail3-domain-modules`.
- **Why:** The test was already asserting JSON validity structurally. Replacing it with string checks would weaken test meaning. The dependency already exists in the workspace lock and is appropriate for this test.
- **Alternatives considered:**
  - Downgrade the test to string containment checks — rejected because it would stop proving valid JSON.
  - Back out the test cleanup and keep the old panic-based shape — rejected because the cleanup itself was correct and local.

## Architectural Notes
This pass kept the same enforcement model:
- no `RS-CODE` rule behavior was loosened
- no checker logic was changed
- reductions came from removing dead exception surfaces in production/test code

The cargo case-table cleanup is useful beyond one family. It demonstrates a cleaner pattern for declarative test fragments:
- use real constant naming for shared config fragments
- keep only fragments the file actually needs
- do not preserve “template leftovers” behind broad test-only lint suppressions

## Information Sources
- Current worktree and recent allow-cleanup worklogs:
  - `.worklogs/2026-03-29-211331-trim-rs-code-allow-surface.md`
  - `.worklogs/2026-03-29-212617-continue-rs-code-allow-cleanup.md`
- Live `RS-CODE` validation from:
  - `cargo run --manifest-path apps/guardrail3/Cargo.toml -p guardrail3 -- rs validate apps/guardrail3 --family code --format json`
- Relevant source files:
  - `apps/guardrail3/crates/domain/modules/mod.rs`
  - `apps/guardrail3/crates/domain/modules/cspell_tests.rs`
  - `apps/guardrail3/crates/shared/fs/src/lib.rs`
  - `apps/guardrail3/crates/adapters/inbound/cli/help_gen_tests.rs`
  - `apps/guardrail3/crates/app/rs/ast/src/ast_helpers_tests/mod.rs`
  - `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/*/cases.rs`

## Open Questions / Future Considerations
- The remaining `RS-CODE-04` inventory is now concentrated in CLI entry/dispatch files and a few reporting helpers. Some of those are probably legitimate and should stay; others still need targeted refactors.
- The next major `RS-CODE` buckets are no longer allow-related. `RS-CODE-24`, `RS-CODE-32`, and `RS-CODE-15` now dominate the family.
- There are unrelated dirty files in the repo outside this commit lane, especially in `hooks-rs`, `project-tree`, and pre-existing code-family test files. They were intentionally left out of this checkpoint.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/rs_cargo_01_workspace_lints_tests/cases.rs` — representative cargo case-table cleanup pattern
- `apps/guardrail3/crates/domain/modules/mod.rs` — fixed registry construction without `vec_init_then_push`
- `apps/guardrail3/crates/domain/modules/cspell_tests.rs` — tightened JSON validation test and required local dev-dependency
- `apps/guardrail3/crates/shared/fs/src/lib.rs` — removed avoidable `manual_ok_err`
- `apps/guardrail3/crates/adapters/inbound/cli/help_gen_tests.rs` — helper-based removal of `expect_used`
- `apps/guardrail3/crates/app/rs/ast/src/ast_helpers_tests/mod.rs` — direct helper cleanup that removed multiple test-only allows
- `.worklogs/2026-03-29-211331-trim-rs-code-allow-surface.md` — first large `RS-CODE` allow reduction pass
- `.worklogs/2026-03-29-212617-continue-rs-code-allow-cleanup.md` — immediate prior continuation and measured state before this batch

## Next Steps / Continuation Plan
1. Continue the `RS-CODE-04` sweep in the remaining high-count files, starting with `crates/adapters/inbound/cli/generate.rs`, `crates/bin/guardrail3/src/main.rs`, `crates/adapters/inbound/cli/diff.rs`, and `crates/adapters/inbound/cli/init.rs`.
2. For each of those files, separate legitimate CLI/process/printing exceptions from weak shape/style exceptions, and remove the weak ones by splitting helpers or simplifying types rather than rewriting comments.
3. Once the remaining allow inventory is materially smaller, switch to the next dominant `RS-CODE` buckets in order: `RS-CODE-24`, `RS-CODE-32`, then `RS-CODE-15`.
4. Keep measuring repo-root `RS-CODE` after each slice with `rs validate ... --family code --format json` so each commit records a real bucket reduction, not just local file churn.
