# Migrate Rust Deps Family Tests

**Date:** 2026-03-24 15:16
**Scope:** `apps/guardrail3/crates/app/rs/checks/rs/deps/**`

## Summary
Committed the `rs/deps` family migration/hardening packet. The batch updates family facts, removes the old flat sidecar tests, and replaces them with rule-specific test directories across the dependency-policy family.

## Context & Problem
After separating out cargo, clippy/deny, code, and garde, the remaining deps-family changes formed a clean isolated packet:
- deps facts updates
- small rule-file adjustments
- deletion of old `*_tests.rs` files
- addition of per-rule `*_tests/` directories

This was much narrower than the surrounding batches and did not need to be folded into another family’s commit.

## Decisions Made

### Keep deps as a standalone migration commit
- **Chose:** Commit `rs/deps` on its own.
- **Why:** The family changes are self-contained and easier to review independently from garde or hexarch.
- **Alternatives considered:**
  - Fold it into garde — rejected because the families own different surfaces and have different semantics.
  - Fold it into cargo — rejected because cargo owns manifest lint policy, while deps owns dependency-tooling and allowlist policy.

### Treat the change as both structure migration and fact-shape tightening
- **Chose:** Describe the commit as more than just test-file shuffling.
- **Why:** `facts.rs` changed along with the test structure, so the commit is a small family reconciliation rather than a pure move-only patch.
- **Alternatives considered:**
  - Call it only test migration — rejected because that would understate the fact-layer change.

## Architectural Notes
- `rs/deps` now follows the same per-rule test-directory pattern expected across the newer checker families.
- The family remains distinct from `rs/cargo`:
  - `rs/cargo` owns Cargo manifest lint policy
  - `rs/deps` owns installed tooling, allowlists, lockfile policy, and dependency-policy input failures

## Information Sources
- `apps/guardrail3/crates/app/rs/checks/rs/deps/**`
- existing family plan already committed earlier in the session

## Open Questions / Future Considerations
- The broader runtime cutover still needs to wire `rs/deps` into the public validator path.
- If later dependency graph rules are added beyond the current surface, they should land as new rules rather than re-bloating the facts input indiscriminately.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/checks/rs/deps/facts.rs` — dependency-family fact normalization
- `apps/guardrail3/crates/app/rs/checks/rs/deps/` — full family rule/test packet

## Next Steps / Continuation Plan
1. Commit the hexarch/shared-discovery batch.
2. Verify the worktree is clean after the final family batch lands.
