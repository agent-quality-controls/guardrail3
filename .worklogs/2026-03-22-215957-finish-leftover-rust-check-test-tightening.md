# Finish Leftover Rust Check Test Tightening

**Date:** 2026-03-22 21:59
**Scope:** `apps/guardrail3/crates/app/rs/checks/rs/{fmt,toolchain,cargo,clippy,deny}/*tests.rs`, `apps/guardrail3/crates/app/rs/checks/rs/deny/test_support.rs`

## Summary
Committed the remaining dirty test-hardening edits that were left behind from the earlier adversarial cleanup work. These changes do not alter checker semantics; they tighten test assertions so rule tests check exact branches, severities, and messages instead of broad “some result exists” conditions.

## Context & Problem
After the `rs/code` completion and the docs cleanup, the repo was still dirty in a large set of Rust check test files. Inspecting the diff showed that this was not a new feature or unrelated noise — it was the leftover half-committed test-hardening work across `fmt`, `toolchain`, `cargo`, `clippy`, and `deny`. Leaving it uncommitted would keep the repo perpetually dirty and blur the actual state of the earlier audit cleanup.

## Decisions Made

### Commit the leftover test-tightening batch separately
- **Chose:** isolate and commit the remaining dirty Rust check test edits as their own cleanup commit.
- **Why:** the changes are coherent, already passing, and represent real prior work rather than incidental dirt. A separate commit keeps history clear instead of mixing them into a later feature.
- **Alternatives considered:**
  - Discard them — rejected because they are valid hardening changes and already verified.
  - Fold them into a future family commit — rejected because they belong to the earlier audit/test-quality line of work.

### Preserve scope to tests and test support only
- **Chose:** include only the modified test files and the tiny `deny/test_support.rs` helper adjustment.
- **Why:** the goal was to clean the worktree and preserve the intended branch-tightening work without sweeping unrelated files.
- **Alternatives considered:**
  - Bundle in broader docs or feature work — rejected because that would muddy the purpose of the commit.

## Architectural Notes
These changes continue the repo-wide rule-test standard:
- assert the exact result count when practical
- assert the exact rule ID, severity, title, and message
- avoid permissive `iter().any(...)` checks when the test is supposed to target a single branch

No production checker behavior changed in this batch.

## Information Sources
- `git diff --stat` and `git diff` over the dirty Rust check test files — confirmed the remaining changes were only test tightening.
- `.worklogs/2026-03-22-203352-finish-rust-check-test-hardening.md` — previous checkpoint for this line of work.
- `cargo test --lib checks::rs --quiet` — verified the full Rust check suite still passes with the tightened tests.

## Open Questions / Future Considerations
- There may still be historical references elsewhere describing the earlier, looser test style, but the current code now reflects the tighter exact-assertion standard.
- If future audit passes find more permissive rule tests, they should be fixed in similarly scoped cleanup commits instead of being left dirty.

## Key Files for Context
- `.worklogs/2026-03-22-203352-finish-rust-check-test-hardening.md` — prior audit cleanup checkpoint.
- `apps/guardrail3/crates/app/rs/checks/rs/fmt/*_tests.rs` — tightened rule-local assertions.
- `apps/guardrail3/crates/app/rs/checks/rs/toolchain/*_tests.rs` — tightened rule-local assertions.
- `apps/guardrail3/crates/app/rs/checks/rs/cargo/*_tests.rs` — tightened rule-local assertions.
- `apps/guardrail3/crates/app/rs/checks/rs/clippy/*_tests.rs` — tightened rule-local assertions.
- `apps/guardrail3/crates/app/rs/checks/rs/deny/*_tests.rs` and `test_support.rs` — tightened deny tests and supporting fixtures.

## Next Steps / Continuation Plan
1. With the leftover dirty batch committed, move on to the next unfinished Rust family without carrying old worktree noise.
2. Run the adversarial audit for `rs/code` if the next step is to verify the newly completed family before starting `hexarch` or another heavy family.
3. Keep future scoped work cleanly committed as soon as it is validated to avoid another backlog like this one.
