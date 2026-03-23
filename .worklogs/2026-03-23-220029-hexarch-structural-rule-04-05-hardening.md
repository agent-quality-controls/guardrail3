# Hexarch Structural Rule 04 And 05 Hardening

**Date:** 2026-03-23 22:00
**Scope:** `apps/guardrail3/crates/app/core/project_walker.rs`, `apps/guardrail3/crates/app/core/project_walker_tests.rs`, `apps/guardrail3/crates/app/rs/checks/rs/hexarch/{facts.rs,inputs.rs,rs_hexarch_04_loose_files.rs,rs_hexarch_05_container_not_empty.rs}`, `apps/guardrail3/crates/app/rs/checks/rs/hexarch/rs_hexarch_04_loose_files_tests/`, `apps/guardrail3/crates/app/rs/checks/rs/hexarch/rs_hexarch_05_container_not_empty_tests/`, `.plans/todo/check_review/test_hardening/{01-hexarch.md,11-hexarch-agent-brief.md}`

## Summary
This pass closed the repeated adversarial hardening loops for `RS-HEXARCH-04` and `RS-HEXARCH-05` at the rule/test level. It fixed one shared `ProjectTree` observation gap for ignored immediate files, hardened rule 05 so symlinked child directories no longer count as real leaf-bearing subdirectories, and expanded both sidecar suites to cover the remaining high-signal ownership and parity edges the attack agents kept surfacing.

## Context & Problem
The user explicitly wanted repeated adversarial loops per rule until fresh attack rounds stopped finding meaningful improvements. `RS-HEXARCH-04` and `RS-HEXARCH-05` were the next structural tranche after rules `01..03`. Earlier structural passes had already moved these rules into per-rule sidecar directories, but the suites were still materially thinner than the old `rs_arch_01` corpus and recent walker changes had opened new symlink and ignore-path seams.

The concrete problems found during the repeated rounds were:
- rule 04 could still under-observe ignored immediate loose files inside already-discovered owned directories because the shared walker patch only restored symlinks, not omitted real files
- rule 05 could describe symlink-only containers as `is empty` even though they contained file-like entries
- rule 05 still treated child directory symlinks as if they were real subdirectories, which let invalid containers escape `RS-HEXARCH-05` and also materialized fake `RS-HEXARCH-06` leaf inputs

## Decisions Made

### Patch immediate raw file children in `ProjectTree`, not only symlinks
- **Chose:** Extend `project_walker`’s final patch pass so it records immediate raw file children for directories already discovered by the main ignore-based walk, not just immediate symlink children.
- **Why:** Structural rules should not fail open on ignored loose files that already sit inside a discovered owned directory. Rule 04 in particular needs to see those files to enforce “only real `.gitkeep` is allowed”.
- **Alternatives considered:**
  - Leave ignored files invisible and just document the limitation — rejected because that preserves an avoidable bypass in an actively hardened family.
  - Disable ignore handling in the walker entirely — rejected because the walker’s broader repo behavior still intentionally respects ignore semantics and this fix only needed the narrower fail-closed patch.

### Keep ignored-whole-directory disappearance as shared walker backlog, not a fake per-rule defect
- **Chose:** Mark the remaining “ignored whole directory can vanish before structural rules see it” issue as a shared `ProjectTree` backlog item in the handoff docs, rather than pretending it is an unresolved defect in rule 04 or 05.
- **Why:** By the end of the fresh rounds, the agents agreed the remaining meaningful concern was no longer rule-local logic. It is a walker/discovery tradeoff that affects more than one structural rule.
- **Alternatives considered:**
  - Keep rule 04 or 05 officially “in progress” until the shared walker is redesigned — rejected because that would blur ownership and stall the per-rule convergence process the user asked for.
  - Ignore the issue entirely — rejected because future sessions need to know this is still a real structural blind spot.

### Treat child directory symlinks as file-like junk for rule 05, not as real subdirectories
- **Chose:** Preserve `symlink_dirs` in `ContainerFacts` / `ContainerHexarchInput`, make rule 05 require at least one real non-symlink directory before it returns clean, and include symlinked child-dir names in the `contains files (...)` message branch.
- **Why:** A container with only symlinked child directories is not a real leaf-bearing container under the hex template. Counting those symlinks as real subdirectories let malformed containers bypass rule 05 and polluted rule 06 by inventing fake leaves.
- **Alternatives considered:**
  - Leave symlink dirs as real dirs for rule 05 and try to catch the problem later in rule 06 — rejected because the container itself is already invalid before any leaf semantics should run.
  - Only change the message branch — rejected because that fixes wording but leaves the bypass alive.

### Stop leaf collection from materializing fake rule-06 leaves for symlinked child dirs
- **Chose:** In `facts.rs`, skip leaf collection for container children that are present only via `symlink_dirs`.
- **Why:** Once symlinked child dirs are not considered real container children for rule 05, rule 06 should also not pretend they are owned leaves. The two rules need consistent discovery semantics.
- **Alternatives considered:**
  - Let rule 06 filter them later — rejected because fake leaf materialization belongs in collector semantics, not in the rule body.

### Close rules 04 and 05 based on repeated fresh-agent convergence, not perfectionism
- **Chose:** Mark `RS-HEXARCH-04` and `RS-HEXARCH-05` complete at the rule/test level once fresh adversarial rounds stopped surfacing meaningful rule-local bugs and only the shared ignored-whole-directory walker issue remained.
- **Why:** The user asked for repeated attack rounds until the last agents stopped bringing improvements that still mattered. That happened for both rules once the final parity and ownership gaps were added.
- **Alternatives considered:**
  - Keep widening the suites with every old-corpus behavior, including lower-value permission-noise parity — rejected because the last rounds were already down to optional or shared-walker concerns.

## Architectural Notes
- `project_walker` now has a stronger final patch phase:
  - immediate symlink children are still restored
  - immediate raw file children in already-discovered directories are also restored and cached when relevant
- `ContainerFacts` / `ContainerHexarchInput` now carry `symlink_dirs` so rule 05 can distinguish real child dirs from symlinked ones
- leaf collection now skips symlink-dir children, which keeps `RS-HEXARCH-06` from receiving fake leaf inputs

For test structure:
- `RS-HEXARCH-04` sidecar grew to 26 tests
- `RS-HEXARCH-05` sidecar grew to 17 tests

The suites now lock down:
- broad owned hit sets
- non-owned boundaries
- mixed cross-rule ownership with neighboring structural rules
- `.gitkeep` vs symlinked `.gitkeep`
- files-only, symlink-only, and symlink-dir container states
- nested-only isolation and destroyed-parent reachability

## Information Sources
- `.plans/todo/checks/rs/hexarch.md`
- `.plans/todo/check_review/test_hardening/01-hexarch.md`
- `.plans/todo/check_review/test_hardening/11-hexarch-agent-brief.md`
- `apps/guardrail3/tests/unit/rs_arch_01/rule_04.rs`
- `apps/guardrail3/tests/unit/rs_arch_01/rule_05.rs`
- repeated 4-agent adversarial rounds during this session on `RS-HEXARCH-04` and `RS-HEXARCH-05`
- direct targeted verification during this pass:
  - `cargo fmt --manifest-path apps/guardrail3/Cargo.toml --all`
  - earlier narrow `cargo test --manifest-path apps/guardrail3/Cargo.toml rs_hexarch_04 -- --nocapture` when unrelated compile drift was not blocking

## Open Questions / Future Considerations
- The shared remaining structural blind spot is broader than rule 04 or 05: if an entire owned directory is ignored and omitted by the main ignore walk, structural rules can still misobserve it because the current patch pass only operates on directories already discovered.
- Repo-local Cargo test verification remains noisy because unrelated dirty-tree compile failures outside this slice still exist, especially in hooks.
- `RS-HEXARCH-06` is now the next live structural rule under the same repeated adversarial protocol.

## Key Files for Context
- `apps/guardrail3/crates/app/core/project_walker.rs` — shared project snapshot builder; now restores immediate raw files as well as symlinks
- `apps/guardrail3/crates/app/core/project_walker_tests.rs` — regression for ignored immediate file children in discovered dirs
- `apps/guardrail3/crates/app/rs/checks/rs/hexarch/facts.rs` — container and leaf fact collection; now preserves `symlink_dirs` and skips fake symlink-dir leaves
- `apps/guardrail3/crates/app/rs/checks/rs/hexarch/inputs.rs` — container input contract now exposes `symlink_dirs`
- `apps/guardrail3/crates/app/rs/checks/rs/hexarch/rs_hexarch_04_loose_files.rs` — rule 04 itself plus the expanded sidecar suite
- `apps/guardrail3/crates/app/rs/checks/rs/hexarch/rs_hexarch_05_container_not_empty.rs` — rule 05 logic, including real-dir detection and file-detail messaging
- `apps/guardrail3/crates/app/rs/checks/rs/hexarch/rs_hexarch_04_loose_files_tests/` — converged rule-04 sidecar suite
- `apps/guardrail3/crates/app/rs/checks/rs/hexarch/rs_hexarch_05_container_not_empty_tests/` — converged rule-05 sidecar suite
- `.plans/todo/check_review/test_hardening/01-hexarch.md` — current lane matrix and resume point
- `.plans/todo/check_review/test_hardening/11-hexarch-agent-brief.md` — current handoff summary for the hexarch lane
- `.worklogs/2026-03-23-210636-hexarch-rule-02-symlink-hardening.md` — earlier structural hardening context for symlink-aware project tree semantics

## Next Steps / Continuation Plan
1. Continue the explicit repeated 4-agent attack protocol on `RS-HEXARCH-06`.
2. Read and compare:
   - `apps/guardrail3/tests/unit/rs_arch_01/rule_06.rs`
   - `apps/guardrail3/crates/app/rs/checks/rs/hexarch/rs_hexarch_06_leaf_valid.rs`
   - `apps/guardrail3/crates/app/rs/checks/rs/hexarch/rs_hexarch_06_leaf_valid_tests/`
3. First target for rule 06 is the next shared walker blind spot already surfaced by the agents: ignored untracked real directories can disappear, which can hide invalid or hybrid leaves before the rule sees them.
4. After that, close the remaining rule-06 parity gaps: symlink-leaf coverage, inner-only isolation, broad files-only invalid leaf coverage, stronger hybrid parity, and tighter exact-count assertions.
5. Keep the lane docs current after each finished rule and continue only with narrowly scoped structural commits so unrelated dirty-tree work is not mixed in.
