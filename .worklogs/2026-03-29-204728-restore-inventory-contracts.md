# Restore Inventory Contracts For Finished Families

**Date:** 2026-03-29 20:47
**Scope:** `apps/guardrail3/crates/app/rs/families/{arch,cargo,clippy,hexarch,test}`, plus one new hexarch assertions helper module and family-level contract tests

## Summary
Restored the intended `--inventory` contract for the finished Rust families so compliant/evaluated rules emit positive inventory findings instead of silently passing. The work covered `RS-ARCH`, `RS-CARGO`, `RS-CLIPPY`, `RS-HEXARCH`, and `RS-TEST`, and also repaired new `RS-TEST` fallout introduced by the `hexarch` inventory tests themselves.

## Context & Problem
The repo had already been driven to failure-clean for several families, but an adversarial review showed that the reporting contract was still inconsistent: many successful checks did not surface in `--inventory`, even though inventory is meant to show what was actually evaluated. The user clarified the intended policy: hidden-by-default pass reporting is fine, but when `--inventory` is enabled, successful/evaluated checks must be visible. A first pass incorrectly quieted some `RS-TEST` mutation rules; that had already been reverted in `72ecb78a`, and this work completed the wider family-level inventory sweep without weakening any failure behavior.

## Decisions Made

### Restore positive inventory in the family runtimes, not by weakening tests
- **Chose:** Add explicit positive inventory branches in the finished family rules for compliant/evaluated success paths.
- **Why:** The problem was incomplete reporting, not false failures. The fix needed to preserve the same warn/error branches while making success visible under `--inventory`.
- **Alternatives considered:**
  - Keep clean runs silent and treat inventory as “exceptions only” — rejected because it contradicts the clarified inventory contract.
  - Relax families to hide positive paths again — rejected because it would preserve the reporting blind spot.

### Keep sidecar semantics owned by assertions crates even for the new inventory-contract tests
- **Chose:** Move new semantic checks introduced by the `hexarch` and `test` inventory work behind sibling assertions helpers/constants instead of letting sidecars assert on result shape directly.
- **Why:** The new inventory tests initially tripped `RS-TEST-03` and `RS-TEST-16`. Fixing those regressions inside the same sweep kept the stricter `RS-TEST` behavior intact.
- **Alternatives considered:**
  - Add direct assertion macros in sidecars over `CheckResult` fields — rejected because that would reintroduce exactly the semantic leakage `RS-TEST-16` is meant to block.
  - Special-case the new family-level tests in `RS-TEST` — rejected because it would weaken the family’s own contract.

### Commit only the inventory-contract families, not unrelated in-flight repo work
- **Chose:** Stage only `arch`, `cargo`, `clippy`, `hexarch`, `test`, and the worklog.
- **Why:** The worktree also contains unrelated `deps`, `release`, `Cargo.lock`, and `project-tree` changes. Bundling them would make the commit incoherent and risk overwriting ongoing work.
- **Alternatives considered:**
  - Commit the entire dirty worktree — rejected because the scope would stop matching the actual task.
  - Cherry-pick family changes later from a larger mixed commit — rejected because it creates avoidable cleanup.

## Architectural Notes
This sweep reinforces a repo-wide rule of thumb for finished guardrail families:
- failure/warn behavior remains the enforcement contract
- successful/evaluated branches must still produce inventory info when `--inventory` is requested
- sidecar tests may verify that inventory behavior exists, but semantic `CheckResult` expectations still belong in sibling assertions crates

`RS-HEXARCH` needed a new assertions-side helper module (`inventory_contract.rs`) because the new family-level contract tests were not rule-specific and still had to stay within the `RS-TEST-03/16` boundaries.

## Information Sources
- User clarification in-session that `--inventory` is for showing all evaluated checks, while default clean runs may hide info.
- Current family runtimes and assertions modules under:
  - `apps/guardrail3/crates/app/rs/families/arch`
  - `apps/guardrail3/crates/app/rs/families/cargo`
  - `apps/guardrail3/crates/app/rs/families/clippy`
  - `apps/guardrail3/crates/app/rs/families/hexarch`
  - `apps/guardrail3/crates/app/rs/families/test`
- Prior corrective commit/worklog restoring `RS-TEST-11..15` pass inventory:
  - `.worklogs/2026-03-29-200525-restore-rs-test-inventory-contract.md`

## Open Questions / Future Considerations
- `release`, `deny`, and `code` still have separate outstanding rule debt; this commit does not address those families.
- The app-root `RS-TEST --inventory` run currently emits a very large info count because compliant checks now inventory broadly and the repo contains many test-bearing roots, including generated/build surfaces under `target/`. That may be acceptable under the current contract, but the broader question of whether build output should be routed/excluded remains separate from this inventory fix.

## Key Files for Context
- `apps/guardrail3/crates/app/rs/families/test/crates/runtime/src/lib.rs` — family orchestrator now emits clean input-failure inventory and drives the stricter sidecar checks.
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/runtime/src/lib.rs` — hexarch orchestrator now inventories compliant success paths broadly.
- `apps/guardrail3/crates/app/rs/families/hexarch/crates/assertions/src/inventory_contract.rs` — new assertions helper used by the family-level hexarch inventory contract tests.
- `apps/guardrail3/crates/app/rs/families/clippy/crates/runtime/src/lib.rs` — clippy family inventory expansion wiring.
- `apps/guardrail3/crates/app/rs/families/cargo/crates/runtime/src/lib.rs` — cargo family inventory expansion wiring.
- `apps/guardrail3/crates/app/rs/families/arch/crates/runtime/src/lib.rs` — arch family inventory expansion wiring.
- `.worklogs/2026-03-29-200525-restore-rs-test-inventory-contract.md` — prior step that restored the mutation-rule inventory contract after an incorrect quiet-on-pass change.

## Next Steps / Continuation Plan
1. Re-run repo-root family summaries after this commit for `arch`, `cargo`, `clippy`, `hexarch`, and `test` to confirm the committed tree still matches the validated dirty tree.
2. Continue the broader inventory-contract audit only on families the user considers “finished”; `release`, `deny`, and `code` should not be mixed into this commit.
3. When returning to `release`/`deps`/other in-flight areas, read this worklog plus `.worklogs/2026-03-29-200525-restore-rs-test-inventory-contract.md` first so the inventory policy does not regress again.
