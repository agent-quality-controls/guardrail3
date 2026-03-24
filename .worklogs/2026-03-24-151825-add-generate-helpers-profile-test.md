# Add Generate Helpers Profile Test

**Date:** 2026-03-24 15:18
**Scope:** `apps/guardrail3/crates/adapters/inbound/cli/generate_helpers_tests.rs`

## Summary
Committed the missing sidecar test for `generate_helpers.rs`. The test locks in that per-app deny generation uses the app’s effective profile rather than the outer default profile.

## Context & Problem
After sorting the worktree into family commits, one untracked file remained: `generate_helpers_tests.rs`. It belongs logically with the earlier clippy/deny generator-profile fix, but it was not staged in that larger commit. Leaving it uncommitted would break the “commit everything in reasonable groups” requirement and would also leave the new `#[path = "generate_helpers_tests.rs"]` module declaration without its sidecar test file in history.

## Decisions Made

### Commit the stranded sidecar as a small follow-up
- **Chose:** Commit the test by itself rather than rewriting the earlier history.
- **Why:** The earlier family commit is already recorded; a tiny follow-up is cleaner than trying to amend after several subsequent commits.
- **Alternatives considered:**
  - Amend the earlier clippy/deny commit — rejected because multiple later commits already depend on the current history.
  - Fold it into an unrelated later batch — rejected because this test clearly belongs to `generate_helpers.rs`.

## Architectural Notes
- This test protects a generator-side parity point:
  - root-local app generation must resolve the effective profile for the app/root being generated
  - not just inherit the outer/default service profile blindly

## Information Sources
- `apps/guardrail3/crates/adapters/inbound/cli/generate_helpers.rs`
- the already-committed clippy/deny family batch that changed the deny-generation profile handling

## Open Questions / Future Considerations
- No new open product question here; this is a small parity lock-in test.

## Key Files for Context
- `apps/guardrail3/crates/adapters/inbound/cli/generate_helpers.rs` — generator helper under test
- `apps/guardrail3/crates/adapters/inbound/cli/generate_helpers_tests.rs` — new sidecar test
- `.worklogs/2026-03-24-151441-harden-clippy-and-deny-families.md` — earlier batch that changed the relevant generation semantics

## Next Steps / Continuation Plan
1. Verify the worktree is clean.
2. Hand the branch back with the sorted commit stack.
